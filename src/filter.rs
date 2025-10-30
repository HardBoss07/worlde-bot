use std::collections::{HashSet, HashMap};

use crate::game::GameData; // assuming GameData is defined in game.rs

pub struct Filter<'a> {
    game: &'a GameData,
    words: &'a [String],
}

impl<'a> Filter<'a> {
    pub fn new(game: &'a GameData, words: &'a [String]) -> Self {
        Self { game, words }
    }

    /// Entry point: runs all filtering steps and returns the valid words
    pub fn filter_words(&self) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| {
                let chars: Vec<char> = word.chars().collect();

                self.has_no_forbidden_letters(&chars)
                    && self.matches_correct_positions(&chars)
                    && self.matches_misplaced_constraints(&chars)
            })
            .cloned()
            .collect()
    }

    /// Step 1: Reject words containing forbidden letters
    fn has_no_forbidden_letters(&self, chars: &[char]) -> bool {
        for ch in chars {
            if self.game.contains_not.contains(ch) {
                return false;
            }
        }
        true
    }

    /// Step 2: Enforce correct-position matches
    fn matches_correct_positions(&self, chars: &[char]) -> bool {
        for (i, correct_opt) in self.game.correct_positions.iter().enumerate() {
            if let Some(expected) = correct_opt {
                if chars[i] != *expected {
                    return false;
                }
            }
        }
        true
    }

    /// Step 3: Enforce misplaced letter constraints
    fn matches_misplaced_constraints(&self, chars: &[char]) -> bool {
        for (pos, misplaced_set) in &self.game.misplaced_letters {
            for &m in misplaced_set {
                // 3.1: misplaced letter cannot be at that position
                if chars[*pos] == m {
                    return false;
                }

                // 3.2: must appear elsewhere in the word
                let mut found_elsewhere = false;
                for (i, &c) in chars.iter().enumerate() {
                    if c == m {
                        // skip banned position
                        if i == *pos {
                            continue;
                        }

                        // skip position if it’s already a confirmed correct position
                        if let Some(correct_ch) = self.game.correct_positions[i] {
                            if correct_ch == m {
                                continue;
                            }
                        }

                        found_elsewhere = true;
                        break;
                    }
                }

                if !found_elsewhere {
                    return false;
                }
            }
        }

        true
    }
}
