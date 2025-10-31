# worlde-bot

A command-line Rust bot for analyzing, ranking, and solving Wordle puzzles.  
It provides detailed letter statistics, ranks words based on frequency and position, and can simulate solving games.

## Features

- **Analyze**: Generate letter statistics from a word list (`letter_stats.json`).
- **Rank**: Rank words by letter frequency and positional value.
- **Solve**: Solve Wordle puzzles using a frequency-based algorithm.
- Fully written in Rust, with a modular design (`analysis`, `ranking`, `solver`, `filter`, `game`).

## Installation

### From crates.io
```bash
cargo install worlde-bot
````

### From source (GitHub)

```bash
git clone https://github.com/yourusername/worlde-bot.git
cd worlde-bot
cargo build --release
```

## Usage

```bash
wordle-bot <analyze|rank|solve>
```

### Commands

* **analyze**
  Generates `letter_stats.json` from `wordlist.txt`, containing frequency and positional statistics for all letters.

* **rank**
  Ranks all words in `wordlist.txt` using the precomputed letter statistics. Outputs the top 10 words.

* **solve**
  Runs the solver module. Currently a placeholder for implementing game simulation or interactive solving.

## Example

```bash
# Analyze letter statistics
wordle-bot analyze

# Rank words
wordle-bot rank

# Solve a puzzle
wordle-bot solve
```

## Project Structure

```
src/
├── analysis.rs   # Letter statistics computation
├── ranking.rs    # Word ranking logic
├── solver.rs     # Wordle solving logic
├── filter.rs     # Word filtering logic
├── game.rs       # Game management and simulation
└── main.rs       # CLI entry point
```

* `wordlist.txt` : Input word list (5-letter words)
* `letter_stats.json` : Generated letter statistics

## License

GPL-3.0 (see [LICENSE](LICENSE))

## Notes / TODO

* Add a simulate mode to run games automatically and calculate average guesses.
* Improve solver algorithm for optimal guess selection.
* Potential future CLI options for custom wordlists or game modes.