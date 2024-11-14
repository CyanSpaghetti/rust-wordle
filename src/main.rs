use rand::prelude::*;
use std::io::{stdin, stdout, Write};

fn main() {
    // choose a word from a random list of words
    let word = get_word();
    // take input
    let mut attempt = get_attempt();
    let mut attempt_count = 1;
    while compare_result(attempt, word.clone()) && attempt_count < 5 {
        attempt = get_attempt();
        attempt_count += 1;
        println!("Attempt: {}/5", attempt_count);
    }
}

fn compare_result(input : String, word : String) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut n : usize = 0;
    for _ in input.chars() {
        if input.chars().nth(n).unwrap() == word.chars().nth(n).unwrap() { 
            println!("C: {}", input.as_bytes()[n] as char);
        } else { 
            println!("W: *"); 
        }
        n+=1;
    }

    if input == word {
        return false;
    }
    return true;
}
fn get_attempt() -> String {
    let mut s : String = String::new();
    print!("Enter Attempt: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

fn get_word() -> String {
    let possible_words = ["chess", "court", "count", "creek", "cunty"];
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0..possible_words.len());
    String::from(possible_words[n]) // return the string
}
