use std::io::{stdin, stdout, Read, Write};
fn main() {

    print!("Enter Value of N:");
    stdout().flush().unwrap();
    let  mut x : String = String::new();
    stdin().read_line(&mut x).unwrap();

    let mut num = x.trim().parse::<i32>().unwrap();

    println!("For Loop Exercise");
    //printing even number from 1 to till N
    for n in 1..num {
        if n % 2 == 0 {
            println!("{}", n);
        }
    }

    println!("While Loop Exercise"); //printing even number between N to 100
    while num < 100{

        if num % 2 == 0 {
            println!("{}", num);
        }
        num+=1;
    }

    //use of Some() and None
    let result  = find_number(num);

    match result {
        Some(n) => println!("Number found {}", n),
        None => println!("Not found"),
    }

}

fn find_number(value :i32) -> Option<i32>{
    if value > 0{
        Some(value)
    }else {
        None
    }
}

