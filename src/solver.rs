use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use anyhow::Result;
use serde::{Serialize, Deserialize};

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
}

impl Solver {
    pub fn new() -> Self {
        Self {
            game: GameData::new(),
        }
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
            println!("(Next best word suggestion goes here...)\n");
        }

        Ok(())
    }
}
