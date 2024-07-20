use std::io;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone, Copy)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn icon(&self) -> &str {
        match self {
            Item::Rock => "🪨",
            Item::Paper => "📄",
            Item::Scissors => "✂️",
        }
    }

    fn play(&self, other: &Item) -> GameResult {
        match (self, other) {
            (Item::Rock, Item::Scissors) => GameResult::Win,
            (Item::Rock, Item::Paper) => GameResult::Lose,
            (Item::Rock, Item::Rock) => GameResult::Draw,
            (Item::Paper, Item::Rock) => GameResult::Win,
            (Item::Paper, Item::Scissors) => GameResult::Lose,
            (Item::Paper, Item::Paper) => GameResult::Draw,
            (Item::Scissors, Item::Paper) => GameResult::Win,
            (Item::Scissors, Item::Rock) => GameResult::Lose,
            (Item::Scissors, Item::Scissors) => GameResult::Draw,
        }
    }
}

fn get_selection_item(opt: &str) -> Item {
    match opt {
        "1" => Item::Rock,
        "2" => Item::Paper,
        _ => Item::Scissors,
    }
}

fn main() {
    let mut stop = false;
    while !stop {
        let mut rng = thread_rng();
        let machine_selection = get_selection_item(&rng.gen_range(1..=3).to_string());
        println!("Welcome to Rock-Paper-Scissors");
        println!("Select one of the following item by its number: ");
        println!("  1: Rock 🪨");
        println!("  2: Paper 📄");
        println!("  3: Scissors ✂️");
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read user input");
        let user = get_selection_item(selection.trim_end());
        // println!("Number {selection}");
        println!("You selected: {}", user.icon());
        println!("Machine selected: {}", machine_selection.icon());
        let result = user.play(&machine_selection);
        match result {
            GameResult::Win => {
                println!("Congratulations, you win!");
            }
            GameResult::Draw => {
                println!("It's a draw!");
            }
            GameResult::Lose => {
                println!("You lose :(");
            }
        }

        let mut keep_playing = String::new();
        println!("Keep playing? y/n");
        io::stdin()
            .read_line(&mut keep_playing)
            .expect("Failed to read user input")
            .to_string();

        if keep_playing.trim_end() == "n" {
            stop = true;
        }
    }
}
