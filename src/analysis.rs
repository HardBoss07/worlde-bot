use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LetterStats {
    // For each letter, store count per position [0..4]
    pub counts: HashMap<char, [u32; 5]>,
}

impl LetterStats {
    pub fn new() -> Self {
        let mut counts = HashMap::new();

        // Initialize for a-z
        for ch in 'a'..='z' {
            counts.insert(ch, [0; 5]);
        }

        Self { counts }
    }

    pub fn from_words(words: &[&str]) -> Self {
        let mut stats = Self::new();

        for word in words {
            let chars: Vec<char> = word.chars().collect();
            if chars.len() != 5 {
                continue; // skip invalid words
            }

            for (i, &ch) in chars.iter().enumerate() {
                if let Some(counts) = stats.counts.get_mut(&ch) {
                    counts[i] += 1;
                }
            }
        }

        stats
    }
}
