use std::{env, fs, u128};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
mod modify;
use modify::process_modified_files;

#[tokio::main]
async fn main() -> Result<()> {

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
      println!("{}",enable_fib);
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or("100".to_string());

    println!("{}", max_threshold);
    let string = "good bad 34 12";
    let output = extract_integer_strings(string);


    let enable_fib:bool = enable_fib.trim().parse().unwrap_or(true);
    let max_threshold:u128 = max_threshold.trim().parse().unwrap_or(100);
    // Fetching environment variables\
    // let input_enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
    // let max_threshold: u128 = env::var("INPUT_MAX_THRESHOLD")
    //     .unwrap_or("100".to_string())
    //     .parse()
    //     .context("Failed to parse INPUT_MAX_THRESHOLD")?;

    // Extract PR number from GITHUB_REF
    // let pr_number: u32 = env::var("GITHUB_REF")
    // .ok()
    // .and_then(|ref_value| {
    //     if ref_value.starts_with("refs/pull/") {
    //         ref_value
    //             .split('/')
    //             .nth(2)
    //             .and_then(|num| num.parse().ok())
    //     } else {
    //         None
    //     }
    // })
    // .context("Failed to parse PR number from GITHUB_REF. Please ensure the workflow is triggered by a pull request.")?;

    // Fetch PR content
    // let pr_content = fetch_pr_content("USHER-PB", "Fibbot-action")
    //     .await
    //     .context("Failed to fetch PR content")?;

    // println!("PR Content: {}", pr_content);

    // Process PR content if Fibonacci calculation is enabled
    if enable_fib == true {
        let integers = output;

        for number in integers {
            if number < max_threshold {
                // Handle the Result returned by fibo_calculator
                if let Err(e) = fibo_calculator(number).await {
                    // eprintln!("Error calculating Fibonacci for {}: {}", number, e);
                }
            }
        }

        let file_paths = "tri.txt";
        let result = process_modified_files(file_paths).await?;

        // Post the result as a comment on the PR
        post_comment(result).await?;
    }

    Ok(())
}

// async fn fetch_pr_content(owner: &str, repo: &str,) -> Result<String> {
//     let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
//     let client = Client::new();
//     let url = format!(
//         "https://api.github.com/repos/{}/{}/pulls/",
//         owner, repo
//     );

//     let response = client
//         .get(&url)
//         .header("Authorization", format!("Bearer {}", token))
//         .header("Content-Type", "application/vnd.github+json")
//         .header("User-Agent", "USHER-PB")
//         .send()
//         .await
//         .context("Failed to send request to GitHub API")?;

//     let response_text = response.text().await.context("Failed to read response body")?;
//     Ok(response_text)
// }

async fn post_comment(body: String) -> Result<()> {
    let client = Client::new();
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
    let repo = env::var("GITHUB_REPOSITORY").context("GITHUB_REPOSITORY not set")?;
    let pr_number = env::var("PR_NUMBER").context("PR_NUMBER not set")?;
    let owner = env::var("GITHUB_OWNER").context("GITHUB_OWNER not set")?;

    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments",owner, repo, pr_number);
    client
        .post(&url)
        .bearer_auth(token)
        .header("User-Agent", "USHER-PB")
        .json(&json!({ "body": body }))
        .send()
        .await
        .context("Failed to post comment")?;  

    Ok(())
}


async fn fibo_calculator(number: u128) -> Result<()> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    if number == 0 {
        post_comment(format!("The Fibonacci value of {} is {}", number, a)).await?;
        println!("the fibonnaci of {} is {}", number,a);
        return Ok(());
    }

    for i in 2..=number {
        let pre_fib = a + b;
        a = b;
        b = pre_fib;
        if i == number {
            println!("the fibonnaci of {} is {}", number,b);
            post_comment(format!("The Fibonacci value of {} is {}", number, b)).await?;          
        }
    }

    Ok(())
}

fn extract_integer_strings(input: &str) -> Vec<u128> {
    input
        .split_whitespace() // Split the input string into substrings
        .filter(|s| s.chars().all(char::is_numeric)) // Keep only substrings that are all numeric
        .filter_map(|s| s.parse::<u128>().ok()) // Parse to u128 and filter out any parsing errors
        .collect() // Collect the results into a Vec<u128>

}

