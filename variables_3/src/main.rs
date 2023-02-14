use std::io;

const MAX_POINTS: u32 = 100_000;

fn variable(gg: i32) {
    println!("function input is: {}",gg);
    let x = 5;
    println!("The value of x is: {}", x);
    println!("The Max Point is: {}", MAX_POINTS);

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let y = 2.0;

    let z: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    let remainder = 43 % 5;
    let t = true;

    let f: bool = false;
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x_tup, y_tup, z_tup) = tup;

    println!("The value of y is: {}", y_tup);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    let a = [3; 5];
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let first = a[0];
    let second = a[1];
}

fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    x + 1
}

fn counter() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn watch_a() {

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn convert_temperature (mode: &str, temperature: i32) -> i32 {
    match mode == "Fahrenheit" {
        true => temperature + 32,
        false => temperature - 32,
    } 
}

fn main() {
    let a: [i32; 5] = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}",index, element);

    variable(element);

    let x = five();

    println!("The five function value of x is: {}", x);

    let y: i32 = plus_one(40);

    println!("The plus one value of x is: {}", y);

    if y > 20 {
        println!("number is greater than 20!");
    } else {
        println!("number is less than equal 20!");
    }

    counter();
    watch_a();
    let temp = convert_temperature("Fahrenheit", 20);
    println!("{}", temp);

}
