use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

fn main() {
    let filename = "input.txt";
    let bingo_info = parse_bingo_file(filename);
    let bingo_numbers = bingo_info.0;
    let bingo_boards = bingo_info.1;
}

fn parse_bingo_file(filename:&str) -> ( Vec<i32>, Vec<[[i32;5];5]>){
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();
    let mut index = 0;
    let mut board_row = 0;
    let mut numbers: Vec<i32> = Vec::new();
    let mut boards:Vec<[[i32;5];5]> = Vec::new();
    let mut temp_board = [[0;5];5];
    for line in lines {
        let goodline = line.unwrap();
        if !goodline.is_empty() {
            if index == 0 { //bingo numbers are first line
                for s in goodline.split(",") {
                    numbers.push(s.parse().unwrap());
                }
            }
            
            if index > 2 { // will skip empty lines so then next 5 lines are a single bingo board
                if board_row <= 4 {
                    let mut temp_ind = 0;
                    for s in goodline.split(" ") {
                        temp_board[board_row][temp_ind] = s.parse().unwrap();
                        temp_ind+=1;
                    }
                    board_row+=1;
                }
            }
            if board_row > 4 {
                board_row = 0;
                boards.push(temp_board);
                temp_board = [[0;5];5];
            }
        }
    }
    return (numbers, boards);
}

fn play_bingo( numbers:Vec<i32>, boards:Vec<[[i32;5];5]>) {

}

fn check_board_won(board:[[i32;5];5]) {

}

fn calculate_winning_score(winning_number:i32, winning_board:[[i32;5];5]) {

}