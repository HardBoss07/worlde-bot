use std::collections::{HashMap, HashSet};
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