use std::io;

fn main() {
    println!("Hello, world!");
    let number =30;

    if number>5{
        println!("The number: {} is bigger than 5",number);
    }
    else { println!("The number: {} is lower than 5",number);
    }

    let mut counter = 0;

    let x = loop {
        counter +=1;
        if counter ==10{
            break counter *2;
        }
    };
    println!("The result is {}", x);


    z(5);
    u();
    i();
    o();

    println!("Please input your numbers.");

    let mut guess = 30;
    let x:u32 = {
        sub(guess, 5)
    };
    println!("x ist: {}",x);
}

fn z (mut number:i32) {

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn u () {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn i (){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn o () {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn sub (num1:u32, num2:u32) -> u32{
    num1+num2

}