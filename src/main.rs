use rand::prelude::*;
use std::io;

const WORDS: [&str; 20] = [
    "Puzzle",
    "Strong",
    "Gadget",
    "Banana",
    "Wonder",
    "Castle",
    "Dragon",
    "Spirit",
    "Planet",
    "Jungle",
    "Adventure",
    "Butterfly",
    "Chocolate",
    "Excellent",
    "Happiness",
    "Paradise",
    "Secretive",
    "Beautiful",
    "Mysterious",
    "Enthusiastic",
];

fn main() {
    let mut rnd = rand::thread_rng();
    let word = WORDS[rnd.gen_range(0..(WORDS.len()))];
    let mut guessed_letters: Vec<char> = vec![];
    let mut lives = 6;

    clear_screen();
    loop {
        draw_hang(word, &guessed_letters, lives);

        let guessed_letter = user_input().to_ascii_lowercase();
        clear_screen();

        if !guessed_letter.is_ascii_alphabetic() {
            println!("Only english letters are allowed... try again!");
        } else if guessed_letters.contains(&guessed_letter) {
            guessed_letters.push(guessed_letter);
            println!("You already guessed {}... try again!", guessed_letter);
        } else if word.to_ascii_lowercase().contains(guessed_letter) {
            guessed_letters.push(guessed_letter);
            println!("You guessed: {}", guessed_letter);
        } else if lives > 0 {
            guessed_letters.push(guessed_letter);
            println!(
                "Sorry, there aren't any '{}' in the word... try again!",
                guessed_letter
            );
            lives -= 1;
        }

        if all_letters_found(word, &guessed_letters) {
            clear_screen();
            println!("You win!");
            draw_hang(word, &guessed_letters, lives);
            break;
        } else if lives <= 0 {
            clear_screen();
            println!("You're dead!");
            draw_hang(word, &guessed_letters, lives);
            break;
        }
    }
}

fn all_letters_found(word: &str, guessed_letters: &[char]) -> bool {
    word.to_ascii_lowercase()
        .chars()
        .all(|ch| guessed_letters.contains(&ch))
}

fn draw_hang(word: &str, guessed_letters: &[char], lives: i32) {
    let word = word.to_ascii_lowercase();

    let mut chars = String::new();
    for ch in word.chars() {
        if guessed_letters.contains(&ch) {
            chars.push_str(format!("{} ", ch).as_str());
        } else {
            chars.push_str("_ ");
        }
    }

    let head = if lives < 6 { "O" } else { " " };
    let left_arm = if lives < 5 { "/" } else { " " };
    let right_arm = if lives < 4 { r"\" } else { " " };
    let body = if lives < 3 { "|" } else { " " };
    let left_leg = if lives < 2 { "/" } else { " " };
    let right_leg = if lives < 1 { r"\" } else { " " };

    let guesses = guessed_letters
        .iter()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!(r"    ___         ");
    println!(r"    |  |     Guesses: {}", guesses);
    println!(r"    |  {}       ", head);
    println!(r"    | {}{}{}      ", left_arm, body, right_arm);
    println!(r"    | {} {}      ", left_leg, right_leg);
    println!(r"  __|__      {}", chars);
}

fn clear_screen() {
    print!("{}[2J", 27 as char); // Windows only?
}

fn user_input() -> char {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Could not read line from user.");

    buf.chars().next().unwrap_or_default()
}
