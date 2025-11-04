use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IndexedRandom;
use crate::game::{GameData, CellData, LineData};
use std::collections::HashMap;

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

    pub fn evaluate_word(&mut self, guessed_word: &str) -> LineData {
        let guessed_chars: Vec<char> = guessed_word.chars().collect();
        let target_chars: Vec<char> = self.word.chars().collect();

        let mut result_cells: [CellData; 5] = [
            CellData { letter: ' ', state: 'w' },
            CellData { letter: ' ', state: 'w' },
            CellData { letter: ' ', state: 'w' },
            CellData { letter: ' ', state: 'w' },
            CellData { letter: ' ', state: 'w' },
        ];

        // Count remaining letters in target for handling duplicates
        let mut remaining_counts: HashMap<char, usize> = HashMap::new();
        for &c in &target_chars {
            *remaining_counts.entry(c).or_insert(0) += 1;
        }

        // First pass: correct positions
        for i in 0..5 {
            let g = guessed_chars[i];
            let t = target_chars[i];

            if g == t {
                result_cells[i] = CellData { letter: g, state: 'c' };
                *remaining_counts.get_mut(&g).unwrap() -= 1;
            } else {
                result_cells[i].letter = g;
            }
        }

        // Second pass: misplaced or wrong
        for i in 0..5 {
            if result_cells[i].state == 'c' {
                continue;
            }
            let g = guessed_chars[i];
            if let Some(count) = remaining_counts.get_mut(&g) {
                if *count > 0 {
                    result_cells[i].state = 'm';
                    *count -= 1;
                } else {
                    result_cells[i].state = 'w';
                }
            } else {
                result_cells[i].state = 'w';
            }
        }

        LineData {
            word: guessed_word.to_string(),
            cells: result_cells,
        }
    }
}
