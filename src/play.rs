use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::game::GameData;

pub struct Play {
    word: String,
    num_guesses: usize,
    game_data: GameData,
}

impl Play {
    pub fn new() -> Self {
        let allowed_words: Vec<String> = fs::read_to_string("wordlist.txt")
            .expect("Failed to read wordlist.txt")
            .lines()
            .skip(10657) // Skip to guess list
            .map(|line| line.trim().to_string())
            .collect();

        let mut rng = thread_rng();
        let random_word = allowed_words
            .choose(&mut rng)
            .expect("No words available")
            .to_lowercase();

        Self {
            word: random_word,
            num_guesses: 6,
            game_data: GameData::new(),
        }
    }
}
