# 15-puzzle
The 15-puzzle is a sliding puzzle that consists of a frame of numbered tiles in random order with one tile missing. The objective of the puzzle is to place the tiles in order by making sliding moves that use the empty space.

## Building
```
cargo build
```

## Running
To generate a random board:
```
cargo run -- --random
```
or
```
cargo run
```
to supply a board configuration on stdin.

## Output
The output consists of the list of moves necessary to solve the game.
