# wordle-bot

A command-line Rust bot for analyzing, ranking, and solving Wordle puzzles.
It provides detailed letter statistics, ranks words based on frequency and position, and can simulate solving games.

## Features

* **Analyze**: Generate letter statistics from a word list (`letter_stats.json`).
* **Rank**: Rank words by letter frequency and positional value.
* **Solve**: Solve Wordle puzzles using a frequency-based algorithm with adjustable weighting.
* Fully written in Rust, with a modular design (`analysis`, `ranking`, `solver`, `filter`, `game`).

## Installation

### From crates.io

```bash
cargo install worlde-bot
```

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
  Runs the interactive solver module. You can enter guesses and feedback (`w`, `m`, `c`) to progressively narrow down possible words.

## Example

```bash
# Analyze letter statistics
wordle-bot analyze

# Rank words
wordle-bot rank

# Solve a puzzle
wordle-bot solve
```

## Tweaking the Solver (`solver_config.json`)

The solver uses a **weighted ranking system** to balance three factors when suggesting the next guess:

| Weight      | Meaning              | Description                                     |
| ----------- | -------------------- | ----------------------------------------------- |
| `w_pos`     | Positional frequency | How common a letter is in a specific position   |
| `w_overall` | Overall frequency    | How common a letter is overall in all positions |
| `w_unique`  | Uniqueness           | Preference for words with more unique letters   |

These values are configured in `solver_config.json`, which defines a list of weights applied per turn (first guess, second guess, etc.):

```json
[
    [0.1, 0.2, 0.7],
    [0.15, 0.25, 0.6],
    [0.25, 0.35, 0.4],
    [0.35, 0.45, 0.2],
    [0.45, 0.45, 0.1],
    [0.6, 0.35, 0.05]
]
```

Each entry corresponds to a turn number:

| Turn | `[w_pos, w_overall, w_unique]`     | Behavior                                                              |
| ---- | ---------------------------------- | --------------------------------------------------------------------- |
| 1    | `[0.1, 0.2, 0.7]`                  | Focus on letter variety to reveal as many unique letters as possible. |
| 2    | `[0.15, 0.25, 0.6]`                | Still prioritizes diversity but starts weighing frequency more.       |
| 3-6  | Increasing `w_pos` and `w_overall` | Gradually shifts toward accuracy and positional matching.             |

To tweak solver behavior:

1. Open `solver_config.json`.
2. Adjust the numbers (they must sum roughly to 1.0, but it's not required).
3. Run the solver again — it automatically reloads the new weights each turn.

### Tips

* Increase `w_unique` for early-game exploration.
* Increase `w_pos` and `w_overall` for late-game precision.
* You can define more entries if you want to simulate longer games (e.g., 7th or 8th guesses).

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
* `solver_config.json` : Solver weight configuration file

## License

AGPL-3.0 (see [LICENSE](LICENSE))

## Notes / TODO

* Add a simulate mode to run games automatically and calculate average guesses.
* Improve solver algorithm for optimal guess selection.
* Potential future CLI options for custom wordlists or game modes.