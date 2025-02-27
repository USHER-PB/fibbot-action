use std::env;

fn fibo_calculator(max_threshold: u128, number: u128) {
    let args: Vec<String> = env::args().collect();

   if args.len() == 3{
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for i in 2..=max_threshold {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == max_threshold{
            println!("the fibo value of {:?} is ::{}", max_threshold, b)
        
   }
}
   }
   else{
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for i in 2..=number {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == number && number < max_threshold {
            println!("the fibo value of {:?} is ::{}", number, b)
        }
        }
        // else {
        //     let max_threshold = number;
        //     println!("the fibo value of {:?} is ::{}", max_threshold, b)
        // }
    }
}

fn params() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let enable_fibbot = &args[1];

        let enable_fibbot: bool = enable_fibbot
            .trim()
            .parse()
            .expect("expected boolean found string");

        if enable_fibbot == true {
            println!("welcome to our fibo calculator");

            let max_threshold = &args[2];
            let max_threshold: u128 = max_threshold
                .trim()
                .parse()
                .expect("Expected a positive integer for max threshold.");

            let number = &args[3];

            let number: u128 = number
                .trim()
                .parse()
                .expect("Expected a positive integer for number");
            fibo_calculator(max_threshold, number);
        }
    } else if args.len() == 3 {
        let enable_fibbot = &args[1];

        let enable_fibbot: bool = enable_fibbot
            .trim()
            .parse()
            .expect("expected boolean found string");

        if enable_fibbot == true {
            println!("welcome to our fibo calculator");

            let max_threshold = &args[2];
            let max_threshold: u128 = max_threshold
                .trim()
                .parse()
                .expect("Expected a positive integer for max threshold.");
            let number = max_threshold;
            fibo_calculator(max_threshold,number);
        }
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
