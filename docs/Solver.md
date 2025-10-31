# Solver

## Found Edge Cases

### Number 1:
> This edge case has been solved

| No. | Word  | Pattern |
| --- | ----- | ------- |
| 1.  | salet | cccww   |
| 2.  | salon | cccww   |
| 3.  | salic | cccww   |
| 4.  | salps | cccwm   |
| 5.  | salad | cccmw   |
| 6.  | salsa | ccccc   |

These are the suggested words:
- sally 0.12534
- salsa 0.11568
- Total Words Left: 2

**Why this happens:**
This happens since the wordlist word remover misses certain words because the misplaced letter already appears at the correct position before and misses it because:
- `a` is at pos 1 (correct)
- `s` is at pos 0 (correct)
- But `a` also appears at position 3 (this position was flagged as misplaced for `a`)
- `s` doesn’t appear at position 4 (ok)

### Number 2:
> This edge case has been solved

| No. | Word  | Pattern |
| --- | ----- | ------- |
| 1.  | salet | wcmmm   |
| 2.  | table | mcwmc   |

Goal Word was: lathe

After entering first word:

- Top suggested words:
- table      0.62935
- latke      0.62887
- lathe      0.62844
- latte      0.51113
- Total Words Left: 4

After entering second word:

- Top suggested words:
- Total Words Left: 0

This happens because the wordlist filter is too strict.

### Number 3:

> This edge case has been solved.

| No. | Word  | Pattern |
| --- | ----- | ------- |
| 1.  | salet | wmwww   |
| 2.  | moray | wmmmw   |
| 3.  | aroid | cmmww   |

Output after 2nd word input:
Top suggested words:
aroid      0.43935
krona      0.43558
rioja      0.43544
draco      0.43453
orgia      0.43452
orang      0.43390
adorn      0.43376
argon      0.43376
acorn      0.43363
abord      0.43309
Total Words Left: 24

Output after 3rd word input:
Top suggested words:
Total Words Left: 0

The word to be found was abhor and i've gotten this with these further inputs (not in the actual solver):

| No. | Word  | Pattern |
| --- | ----- | ------- |
| 4.  | birch | mwmwm   |
| 5.  | abhor | ccccc   |

## Structs

### CellData
Contains a single cells data. Contains its Letter and state.

### LineData
Contains a single word and all its cells data.

### GameData
Contains all words, a list of all letters which arent in the word, a list of correctly placed letters, and a table of each correct but wrongly placed letters and the respective position.