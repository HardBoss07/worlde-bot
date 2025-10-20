mod analysis;
mod ranking;

use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wordle-bot <analyze|rank>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "analyze" => analyze()?,
        "rank" => rank()?,
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn analyze() -> Result<()> {
    use analysis::LetterStats;

    let content = fs::read_to_string("wordlist.txt")?;
    let words: Vec<&str> = content.lines().collect();
    let stats = LetterStats::from_words(&words);

    fs::write("letter_stats.json", serde_json::to_string_pretty(&stats)?)?;
    println!("Saved letter stats to letter_stats.json");

    Ok(())
}

fn rank() -> Result<()> {
    use ranking::rank_words;
    let content = fs::read_to_string("wordlist.txt")?;
    let words: Vec<&str> = content.lines().collect();

    let stats_json = fs::read_to_string("letter_stats.json")?;
    let results = rank_words(&words, &stats_json)?;

    println!("Top 10 words by letter position frequency:");
    for (word, score) in results.iter().take(10) {
        println!("{word:<10} {score:.2}");
    }

    Ok(())
}
