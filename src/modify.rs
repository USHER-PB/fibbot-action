
use crate::extract_integer_strings;
use crate::fibo_calculator;
use std::fs;
use anyhow::Result;

pub(crate) async fn process_modified_files(file_paths: &str) -> Result<String> {
    let mut all_numbers = Vec::new();

    for file_path in file_paths.split(',') {
        if let Ok(content) = fs::read_to_string(file_path) {
            let numbers = extract_integer_strings(&content);
            all_numbers.extend(numbers);
        }
    }

    if all_numbers.is_empty() {
        return Ok("No numbers found in the modified files.".to_string());
    }

    let mut response = String::from("#### Fibonacci Results:\n");
    for &num in &all_numbers {
        let fib = fibo_calculator(num).await?; // Await the Future and handle the Result
        response.push_str(&format!("- Fibonacci({}) = {:?}\n", num, fib));
    }

    Ok(response)
}