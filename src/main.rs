use std::io;
use rand::Rng;
use std::time::Duration;
use std::thread;
use std::collections::HashSet;


// draw stick figure depending on how many incorrect attempts have been taken
fn draw_stick_man(incorrect: usize, winner: bool) {
    match incorrect {
        0 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |       ");
            println!(" |       ");
            println!(" |       ");
            println!("_|_______");
            println!();
        }
        1 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |       ");
            println!(" |       ");
            println!("_|_______");
            println!();
        }
        2 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |    |  ");
            println!(" |       ");
            println!("_|_______");
            println!();
        }
        3 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |   /|  ");
            println!(" |       ");
            println!("_|_______");
            println!();
        }
        4 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |   /|\\ ");
            println!(" |       ");
            println!("_|_______");
            println!();
        }
        5 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |   /|\\ ");
            println!(" |   /   ");
            println!("_|_______");
            println!();
        }
        6 => {
            println!("  ____   ");
            println!(" |    |  ");
            println!(" |    O  ");
            println!(" |   /|\\ ");
            println!(" |   / \\ ");
            println!("_|_______");
            println!();
        }
        _ => {
            println!("Invalid value for incorrect.");
        }
    }


    // makes the stickman dance if they solved the puzzle
    if winner{


        //pause time
        let duration = Duration::from_secs_f64(0.5);


        for _ in 0..3 {
            println!("         ");
            println!("         ");
            println!("         ");
            println!("      O  ");
            println!("     \\|/ ");
            println!("_____/_\\_____");
            thread::sleep(duration);


            println!("         ");
            println!("         ");
            println!("         ");
            println!("      O  ");
            println!("     /|/ ");
            println!("_____/_\\_____");
            thread::sleep(duration);


            println!("         ");
            println!("         ");
            println!("         ");
            println!("      O  ");
            println!("     /|\\ ");
            println!("_____/_\\_____");
            thread::sleep(duration);
        }
    }


}


// main
fn main() {
    println!();
    println!();
    println!();
    println!();


    // randomly selects a code word from the answer bank
    let answer_bank = vec!["Nephi", "Lehi", "Moroni", "Alma", "Helaman", "Mosiah", "Benjamin", "Ether", "Jared", "Ammon",
    "Abinadi", "Zarahemla", "Enos", "Jarom", "Omni", "Mormon", "Teancum"];


    let length = answer_bank.len();
    let rand_int = rand::thread_rng().gen_range(0..=length - 1);
    let code_word = answer_bank[rand_int].to_uppercase();


    // creates a set of the letter in the code word
    let mut letter_set = HashSet::new();


    for letter in code_word.chars() {
        letter_set.insert(letter);
    }


    // creates a set to compare to the letter_set to tell when puzzle solved
    let mut correct_guesses_set = HashSet::new();


    // creates vectors to store all guesses or incorrect guesses
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<char> = Vec::new();


    // game status
    let mut solved = false;


    // pause time
    let duration = Duration::from_secs(2);




    println!("Guess the word to save the man!");


    loop {


        // draws current state of the stick man
        let incorrect_guesses_length = incorrect_guesses.len();
        draw_stick_man(incorrect_guesses_length, solved);


        // ends game if fail
        if incorrect_guesses_length == 6 {
            break;
        }


        // prints underscores or letters depending on if the user has guessed a letter in the code word
        for letter in code_word.chars() {
            if correct_guesses_set.contains(&letter) {
                print!("{letter}");
            } else {
                print!("_");
            }
        }
        println!();
        println!();


        // prints previous guesses
        println!("Guessed Letters:");
            for char in &guessed_letters {
                print!("{char}, ");
            }
        println!();
        println!();


        println!("Guess a letter!");


        let mut guess_letter = String::new();
       
        // read user input
        io::stdin()
            .read_line(&mut guess_letter)
            .expect("Failed to read line");


        // capitalizes guess and trims extra characters
        let guess_letter = guess_letter.trim().to_uppercase();
        println!();


        // checks to see if the user inputted only one char
        if guess_letter.len() == 1 {
            let letter = guess_letter.chars().next().unwrap();


            // assigns letter to correct group
            if code_word.contains(letter) {
                correct_guesses_set.insert(letter);
                println!("Great job! {letter} is in the word!!");
            } else {
                incorrect_guesses.push(letter);
                println!("{letter} is not in the word!");
            }
            println!();


            // checks to see if letter is already in previous guesses
            if !guessed_letters.contains(&letter){
                guessed_letters.push(letter);
            } else {
                println!("You already guessed that letter!");
            }
           
        } else {
            println!("Please enter exactly one letter.");
            continue;
        }


        println!();


        // checks to see if solved
        if letter_set == correct_guesses_set {
            solved = true;
            break;
        }


        thread::sleep(duration);


    }


    // lets user know if they won or lost
    if solved {
        draw_stick_man(0, solved);
        println!("You Won!!! He Survived!! The word was {code_word}!");
    } else {
        println!("You lost! The word was {code_word}");
    }


}
