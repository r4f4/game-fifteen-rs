extern crate clap;

use clap::{App, Arg};
use std::io;
use std::io::Read;
use std::process;

use game15::*;

fn print_game_replay(board: &mut Board, moves: Vec<Direction>) {
    for &dir in moves.iter() {
        assert!(!board.slide_safe(dir).is_err());
        println!("{}", dir);
        println!("{}", board);
    }
}

fn main() {
    let matches = App::new("Game Fifteen (15-puzzle)")
        .version("0.1.0")
        .author("Rafael Fonseca <r4f4rfs@gmail.com>")
        .about("Solves a 15-puzzle instance")
        .usage("game15 [--replay] [--random|<stdin>]")
        .after_help(
            "If --random is not supplied, it reads a board configuration from stdin.
The format expected is one row per line, each row containing 4 space-separated numbers.
Example:
0 1 2 3
4 5 6 7
8 9 10 11
12 13 14 15",
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .takes_value(false)
                .help("Use a randomly generated board"),
        )
        .arg(
            Arg::with_name("replay")
                .long("replay")
                .takes_value(false)
                .help("Replays the moves instead of just printing a list"),
        )
        .get_matches();

    let mut board = match matches.is_present("random") {
        true => Board::new_random(),
        false => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Err(err) => panic!("IO error: {}", err),
                Ok(_) => {
                    let mut config: Vec<u8> = Vec::new();
                    let lines: Vec<&str> = buffer.split('\n').collect();
                    for line in lines {
                        let mut v: Vec<&str> = line.split(' ').collect();
                        v.retain(|&x| x != "");
                        let mut l: Vec<u8> = v
                            .iter()
                            .map(|x| x.parse::<u8>().expect("failed to parse number"))
                            .collect();
                        config.append(&mut l);
                    }
                    match Board::new_from(config.as_slice()) {
                        Ok(b) => b,
                        Err(msg) => {
                            eprintln!("Invalid board: {}", msg);
                            process::exit(1)
                        }
                    }
                }
            }
        }
    };
    println!("{}", board);
    if !board.solvable() {
        println!("Board cannot be solved");
        return;
    }
    match Astar::run(&board) {
        Some(moves) => {
            println!("Number of moves needed: {}", moves.len());
            match matches.is_present("replay") {
                true => print_game_replay(&mut board, moves),
                false => println!("{:?}", moves),
            }
        }
        None => println!("Could not solve board"),
    }
}
