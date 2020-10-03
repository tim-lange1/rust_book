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