use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sende eine Beendigungsnachricht an alle worker.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Fahre alle Worker herunter.");

        for worker in &mut self.workers {
            println!("Worker {} herunterfahren", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Erzeuge einen neuen ThreadPool.
   ///
   /// Die Größe ist die Anzahl der Stränge im Vorrat.
   ///
   /// # Panics
   ///
   /// Die Funktion `new` stürzt ab, wenn die Größe Null ist.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));

        }
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->Worker {
        let thread = thread::spawn(move ||loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} hat einen Auftrag erhalten; führe ihn aus.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} wurde aufgefordert sich zu beenden.", id);

                    break;
                }

            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}