use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    let y = &x;
    let z = MyBox::new(x);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);


    asser_eq!(5,x);
    assert_eq(5,*y);
    assert_eq!(5,*z);


}

struct MyBox <T> (T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref (&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hallo {}!", name);
}