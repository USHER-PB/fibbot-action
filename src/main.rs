use std::{env, u128};
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Fetching environment variables
    let input_enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
    let max_threshold: u128 = env::var("INPUT_MAX_THRESHOLD").unwrap_or("100".to_string()).parse().unwrap();
    let pr_number_str = env::var("PR_NUMBER").expect("PR_NUMBER not set")
    println!("PR_NUMBER: {}", pr_number_str);
    // Parse the PR number with error handling
    let pr_number: u32 = match pr_number_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: PR_NUMBER must be a valid integer.");
            std::process::exit(1); // Exit with an error code
  
        }
    };

    // Fetch PR content
    let pr_content = fetch_pr_content("USHER-PB", "Fibbot", pr_number).await.expect("Failed to fetch PR content");
   // Example content for processing

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

async fn fetch_pr_content(owner: &str, repo: &str, pr_number: u32) -> Result<String, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_number
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}

async fn post_comment(body: String) {
    let client = Client::new();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER").expect("PR_NUMBER not set");

    let url = format!("https://api.github.com/repos/{}/issues/{}/comments", repo, pr_number);
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