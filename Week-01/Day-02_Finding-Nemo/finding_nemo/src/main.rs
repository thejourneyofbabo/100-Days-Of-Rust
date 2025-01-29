use std::io;

fn main() {
    loop {
        println!("Let's find Nemo (or type 'q' to quit)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // Trim whitespace including newline

        if input == "q" {
            break;
        }

        let words: Vec<&str> = input.split_whitespace().collect();

        let mut count = 0;

        for (index, word) in words.iter().enumerate() {
            if *word == "Nemo" {
                println!("We found Nemo at position {}", index + 1);
                count += 1;
            }
        }

        if count == 0 {
            println!("There is no Nemo");
        }

        println!(); // Add a blank line for readability
    }

    println!("Thanks for playing!");
}
