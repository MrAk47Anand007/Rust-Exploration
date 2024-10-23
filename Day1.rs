use std::io::{stdin, stdout, Read, Write};
fn main() {
    print!("Enter Value of X:");
    stdout().flush().unwrap();
    let  mut x : String = String::new();
    stdin().read_line(&mut x).unwrap();

    println!("Enter Value of Y:");
    stdout().flush().unwrap();
    let  mut y : String = String::new();
    stdin().read_line(&mut y).unwrap();


    let add = addition(x.trim().parse::<i32>().unwrap(),y.trim().parse::<i32>().unwrap());
    let subtract = subtraction(x.trim().parse::<i32>().unwrap(),y.trim().parse::<i32>().unwrap());
    println!("Addition of X and Y is: {}",add);
    println!("Subtraction of X and Y is: {}",subtract);


    if(add > subtract){
        println!("Add of overflow!");
    }
}

fn addition(x:i32, y:i32) -> i32 {
    x + y
}

fn subtraction(x:i32, y:i32) -> i32 {
    x - y
}