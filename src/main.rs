use std::env;
use reqwest::Client;
use serde_json::json;

 // Assuming you have a lib.rs for core logic

#[tokio::main]
async fn main() {
    // Fetching environment variables
    let input_enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("false".to_string());
    let max_threshold: u128 = env::var("INPUT_MAX_THRESHOLD").unwrap_or("100".to_string()).parse().unwrap();

    // Fetch PR content
    let pr_content = get_pr_content().await;

    // Process PR content if Fibonacci calculation is enabled
    if input_enable_fib == "true" {
        let integers = extract_integer_strings(&pr_content);

        for number in integers {
            if number < max_threshold {
                fibo_calculator(number).await; // Calculate and post Fibonacci value
            }
        }
    }
}

async fn get_pr_content() -> String {
    // Implement logic to fetch PR content using GitHub API
    // Replace with actual GitHub API call
    String::from("Sample PR content with numbers like 5 and 8.")
}

async fn post_comment(body: String) {
    let client = Client::new();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let issue_number = env::var("GITHUB_REF").expect("GITHUB_REF not set");

    let url = format!("https://api.github.com/repos/{}/issues/{}/comments", repo, issue_number);
    client.post(&url)
        .bearer_auth(token)
        .header("User-Agent", "FibBot")
        .json(&json!({ "body": body }))
        .send()
        .await
        .expect("Failed to post comment");
}

async fn fibo_calculator(number: u128) {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    if number == 0 {
        post_comment(format!("The Fibonacci value of {} is {}", number, a)).await;
        return;
    }

    for i in 2..=number {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == number {
            post_comment(format!("The Fibonacci value of {} is {}", number, b)).await; // Post the result as a comment
        }
    }
}

fn extract_integer_strings(input: &str) -> Vec<u128> {
    input
        .split_whitespace() // Split the input string into substrings
        .filter(|s| s.chars().all(char::is_numeric)) // Keep only substrings that are all numeric
        .filter_map(|s| s.parse::<u128>().ok()) // Parse to u128 and filter out any parsing errors
        .collect() // Collect the results into a Vec<u128>
}



// uses: ./
// with:
//   enable_fibbot: "true"
//   max_threshold: "100"

