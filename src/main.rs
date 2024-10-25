use std::{io::{stdin, stdout, Write}, time::{Duration, Instant}};

use colored::Colorize;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let mut correct = 0;
    let mut time = Duration::ZERO;

    println!("Velkommen til tabelmester 1.0!");
    let duration_seconds = prompt_duration();
    let _ = prompt_input("Tryk enter for at starte!");

    let start = Instant::now();

    while time.as_secs() < duration_seconds {
        let a = rng.gen_range(2..=9);
        let b = rng.gen_range(2..=9);

        let prompt = format!("{:02}: {} * {} = ", time.as_secs(), a, b);
        let input = prompt_input(&prompt);

        // Removes the newline from stdout which occurs when the user presses enter
        print!("\x1b[1A\x1b[{}C", prompt.len() + input.len());

        time = Instant::now() - start;

        match input.parse::<i32>() {
            Ok(_) if time.as_secs() >= duration_seconds => println!(" ⏰"),
            Ok(answer) if a * b == answer => {
                println!(" {}", "✔".green());
                correct += 1;
            },
            _ => println!(" {}", "X".red())
        }
    }

    println!("Du fik {} korrekte svar på {} sekunder!", correct, duration_seconds);
    let _ = prompt_input("Tryk enter for at afslutte");
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    
    if input.ends_with('\n') {
        input.pop();
    }
    if input.ends_with('\r') {
        input.pop();
    }

    input
}

fn prompt_duration() -> u64 {
    loop {
        let result = prompt_input("Indtast tid i antal sekunder: ").parse();
        match result {
            Ok(duration) => return duration,
            Err(_) => println!("Du skal indtaste et tal!"),
        }
    }
}