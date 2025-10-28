use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellData {
    pub letter: char,
    pub state: char, // 'w' = not in word, 'm' = misplaced, 'c' = correct
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineData {
    pub word: String,
    pub cells: [CellData; 5],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub lines: Vec<LineData>,
    pub contains_not: HashSet<char>,
    pub correct_positions: [Option<char>; 5],
    pub misplaced_letters: HashMap<usize, HashSet<char>>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            contains_not: HashSet::new(),
            correct_positions: [None, None, None, None, None],
            misplaced_letters: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, word: &str, pattern: &str) {
        let mut cells = Vec::new();

        for (i, (ch, state)) in word.chars().zip(pattern.chars()).enumerate() {
            let cell = CellData { letter: ch, state };
            cells.push(cell.clone());

            match state {
                'c' => self.correct_positions[i] = Some(ch),
                'm' => {
                    self.misplaced_letters.entry(i).or_default().insert(ch);
                }
                'w' => {
                    if !self.correct_positions.contains(&Some(ch))
                        && !self.misplaced_letters.values().any(|v| v.contains(&ch))
                    {
                        self.contains_not.insert(ch);
                    }
                }
                _ => {}
            }
        }

        let cells: [CellData; 5] = cells.try_into().unwrap();
        self.lines.push(LineData {
            word: word.to_string(),
            cells,
        });
    }

    pub fn print_summary(&self) {
        println!("\n=== Current Game State ===");
        println!("Guesses so far: {}", self.lines.len());
        println!("Not in word: {:?}", self.contains_not);
        println!("Correct positions: {:?}", self.correct_positions);
        println!("Misplaced letters: {:?}", self.misplaced_letters);
        println!("==========================\n");
    }
}

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
            println!("(Next best word suggestion goes here...)");
            self.current_words = self.update_wordlist();
            println!("Words left: {}\n", self.current_words.len());
        }

        Ok(())
    }

    pub fn update_wordlist(&self) -> Vec<String> {
        let mut filtered_words = Vec::new();

        'outer: for word in &self.current_words {
            let chars: Vec<char> = word.chars().collect();

            // 1. Skip words containing forbidden letters
            for ch in &chars {
                if self.game.contains_not.contains(ch) {
                    continue 'outer;
                }
            }

            // 2. Skip words that don't have correct letters in correct positions
            for (i, correct_opt) in self.game.correct_positions.iter().enumerate() {
                if let Some(expected) = correct_opt {
                    if chars[i] != *expected {
                        continue 'outer;
                    }
                }
            }

            // 3. Skip words that violate misplaced letter rules
            for (pos, misplaced_set) in &self.game.misplaced_letters {
                for &m in misplaced_set {
                    // Rule 1: letter m must NOT appear in this position
                    if chars[*pos] == m {
                        continue 'outer;
                    }
                    // Rule 2: letter m must appear somewhere else in the word
                    if !chars.contains(&m) {
                        continue 'outer;
                    }
                }
            }

            filtered_words.push(word.clone());
        }

        filtered_words
    }
}
