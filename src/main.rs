use std::{env, io};

fn fibo_calulator() {
  

    let mut number = String::new();
    println!("enter the the value you want its fibo");
    io::stdin()
        .read_line(&mut number)
        .expect("no value enterred please restart");

    let number: u128 = number
        .trim()
        .parse()
        .expect("the value is not an integer .. please enter an integer");

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    let args: Vec<String> = env::args().collect();
    let max_threshold = &args[2];
    let max_threshold: u128 = max_threshold
        .trim()
        .parse()
        .expect("expected boolean found string");

    for i in 2..=number {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == number && number < max_threshold {
            println!("the fibo value of {:?} is ::{}", number, b)
        }
    }
}


fn params() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <enable_fibbot> <max_threshold>");
        std::process::exit(1);
    }
    let enable_fibbot = &args[1];
   
    let enable_fibbot: bool = enable_fibbot
        .trim()
        .parse()
        .expect("expected boolean found string");
    
    if enable_fibbot == true {
        println!("welcome to our fibo calculator");
        fibo_calulator();
    } else {
        // enable_fibbot == false;
        // println!("{}", enable_fibbot);
           println!("the boolean you place is not valid")

    }
    }
    // if max_threshold > number{
    //     let input = fibo_calulator();
    //     println!("{:?}", input)



fn main() {
    println!("hello world");
    params();
    
}
