use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::anyhow;
use crate::ranking::weighted_rank;
use crate::filter::Filter;
use crate::game::{GameData, LineData, CellData};

pub struct Solver {
    game: GameData,
    current_words: Vec<String>,
}

impl Solver {
    pub fn new() -> Result<Self> {
        let content = fs::read_to_string("wordlist.txt")
            .map_err(|e| anyhow!("Failed to read wordlist.txt: {}", e))?;

        let words: Vec<String> = content
            .lines()
            .map(|w| w.trim().to_lowercase())
            .filter(|w| w.len() == 5)
            .collect();

        if words.is_empty() {
            return Err(anyhow!("wordlist.txt is empty or invalid"));
        }

        Ok(Self {
            game: GameData::new(),
            current_words: words,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // Step 1: enter word
            print!("Enter your 5-letter guess (or 'exit'): ");
            io::stdout().flush()?;
            let mut word = String::new();
            io::stdin().read_line(&mut word)?;
            let word = word.trim().to_lowercase();
            if word == "exit" {
                println!("Exiting solver.");
                break;
            }
            if word.len() != 5 {
                println!("Please enter a 5-letter word.\n");
                continue;
            }

            // Step 2: enter pattern
            print!("Enter pattern (w = wrong, m = misplaced, c = correct): ");
            io::stdout().flush()?;
            let mut pattern = String::new();
            io::stdin().read_line(&mut pattern)?;
            let pattern = pattern.trim().to_lowercase();
            if pattern.len() != 5 || !pattern.chars().all(|c| "wmc".contains(c)) {
                println!("Invalid pattern. Use only w, m, c.\n");
                continue;
            }

            // Update game
            self.game.add_line(&word, &pattern);

            // Show summary
            self.game.print_summary();

            // Placeholder for ranking logic
            let stats_json = fs::read_to_string("letter_stats.json")
                .map_err(|e| anyhow!("Failed to read letter_stats.json: {}", e))?;
            self.rank_words(&stats_json);
        }

        Ok(())
    }

    pub fn rank_words(&mut self, stats_json: &str) -> Result<()> {
        // Read solver_config.json as Vec of tuples
        let config_content = fs::read_to_string("solver_config.json")
            .map_err(|e| anyhow!("Failed to read solver_config.json: {}", e))?;

        let weights: Vec<(f64, f64, f64)> = serde_json::from_str(&config_content)
            .map_err(|e| anyhow!("Failed to parse solver_config.json: {}", e))?;

        // Select weight set based on number of guesses
        let attempt = self.game.lines.len().min(weights.len() - 1);
        let weight_tuple = weights[attempt];

        // Update wordlist
        self.current_words = self.update_wordlist();

        // Prepare for ranking
        let word_refs: Vec<&str> = self.current_words.iter().map(|s| s.as_str()).collect();
        let ranked_words = weighted_rank(&word_refs, stats_json, weight_tuple)?;

        println!("Top suggested words:");
        for (word, score) in ranked_words.iter().take(10) {
            println!("{word:<10} {score:.5}");
        }
        println!("Total Words Left: {}\n", self.current_words.len());

        Ok(())
    }

    pub fn update_wordlist(&self) -> Vec<String> {
        let filter = Filter::new(&self.game, &self.current_words);
        filter.filter_words()
    }
}
