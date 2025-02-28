use std::env;
use reqwest::Client;
use serde_json::json;

fn fibo_calculator(max_threshold: u128, number: u128) {
    let args: Vec<String> = env::args().collect();

   if args.len() == 3{
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for i in 2..=number {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == number{
            println!("the fibo value of {:?} is ::{}", number, b)
        
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

fn extract_integer_strings(input: &str) -> Vec<u128> {
    input
        .split_whitespace() // Split the input string into substrings
        .filter(|s| s.chars().all(char::is_numeric)) // Keep only substrings that are all numeric
        .filter_map(|s| s.parse::<u128>().ok()) // Parse to u128 and filter out any parsing errors
        .collect() // Collect the results into a Vec<u128>

}



fn params() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
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

            // let number = &args[3];

            // let number: u128 = number
            //     .trim()
            //     .parse()
            //     .expect("Expected a positive integer for number");
        }
    
        }
    }

// if max_threshold > number{
//     let input = fibo_calulator();
//     println!("{:?}", input)

async fn post_comment(issue_number: u64, body: &str) {
    let client = Client::new();
    let token = env::var("GITHUB_TOKEN").unwrap();
    let owner = env::var("GITHUB_REPOSITORY_OWNER").unwrap();
    let repo = env::var("GITHUB_REPOSITORY").unwrap();


    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", owner, repo, issue_number);
    
    let res = client.post(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "FibBot")
        .json(&json!({ "body": body }))
        .send()
        .await;
    
    match res {
        Ok(response) => println!("Comment posted: {:?}", response),
        Err(err) => eprintln!("Error posting comment: {:?}", err),
    }
}



fn main() {
    println!("hello world");
    let pr_content = "Here are some numbers: 5, 8, 13, and 21.";
    let numbers = extract_integer_strings(pr_content);
    let max_threshold = 100;

    for number  in integers {
        fibo_calculator(max_threshold ,number);
    params();
}
}