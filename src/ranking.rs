use crate::analysis::LetterStats;
use anyhow::Result;
use serde_json;
use std::collections::HashMap;

pub fn rank_words<'a>(words: &[&'a str], stats_json: &str) -> Result<Vec<(String, f64)>> {
    let stats: LetterStats = serde_json::from_str(stats_json)?;

    // Totals per position
    let mut totals = [0.0; 5];
    for counts in stats.counts.values() {
        for (i, &c) in counts.iter().enumerate() {
            totals[i] += c as f64;
        }
    }

    // Overall frequency across all positions
    let mut overall_totals = HashMap::new();
    let mut grand_total = 0.0;
    for (ch, counts) in &stats.counts {
        let sum: f64 = counts.iter().map(|&c| c as f64).sum();
        overall_totals.insert(*ch, sum);
        grand_total += sum;
    }

    // Weight between positional (0.0–1.0)
    let alpha = 0.7;
    let mut scores: Vec<(String, f64)> = Vec::new();

    for &word in words {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() != 5 || !chars.iter().all(|c| c.is_ascii_lowercase()) {
            continue;
        }

        let mut base_score = 0.0;

        for (i, &ch) in chars.iter().enumerate() {
            let pos_score = if let Some(counts) = stats.counts.get(&ch) {
                if totals[i] > 0.0 {
                    counts[i] as f64 / totals[i]
                } else {
                    0.0
                }
            } else {
                0.0
            };

            let overall_score = if let Some(&sum) = overall_totals.get(&ch) {
                sum / grand_total
            } else {
                0.0
            };

            // Blend position and overall letter frequency
            let blended = alpha * pos_score + (1.0 - alpha) * overall_score;
            base_score += blended;
        }

        // Uniqueness factor (0.5–1.0)
        let mut unique_letters = HashMap::new();
        for &ch in &chars {
            *unique_letters.entry(ch).or_insert(0) += 1;
        }
        let uniqueness = unique_letters.len() as f64 / 5.0;
        let final_score = base_score * (0.5 + 0.5 * uniqueness);

        scores.push((word.to_string(), final_score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    Ok(scores)
}
