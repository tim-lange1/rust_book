use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Fehler beim Parsen der Argumente: {}", err);
        process::exit(1);
    });

   if let Err(e) =  minigrep::run(config) {
       eprintln!("Anweisungsfehler: {}", e);
       process::exit(1);
   }
}
