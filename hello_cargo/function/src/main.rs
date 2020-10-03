fn main () {
    println!("Hello, world!");
    another_function(5,6);
    let z= 220;
    let number = ruck();
    println!("The number of ruck is: {}",number);
    let z = sum(5,6);
    println!("z is {}", z);

}

fn another_function (x: i32, y:i32){
    println!("The Value of x is: {}",x);
    println!("The Value of y is: {}",y);

    let p = {
        let i= 2;
        i+1
    };

    println!("The Value of p is: {}",p);
}

fn ruck()-> i32 {
    5
}

fn sum(x:i32, y:i32)-> i32{
    x+y
}
