fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}",x);
    x=6;
    println!("The value of x is: {}",x);

    let x=5;
    let x = x+1;
    let x= x*2;
    println!("The value of x is: {}",x);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

fn another_function (){

}