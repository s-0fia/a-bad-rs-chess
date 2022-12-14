use std::io;
use std::io::Write;
use crate::piece::{Piece, Type, Colour};
mod piece;
mod lib;

fn main() {
    let empty_piece = Piece(Type::None, Colour::None);
    let a = Piece(Type::Bishop, Colour::White);
    let b = Piece(Type::Pawn, Colour::Black);
    let mut board: [[Piece; 8]; 8] = [[empty_piece; 8]; 8];
    board[3][0] = a;
    board[5][2] = b;

    draw(&board);

    let (can_move, result) = piece::move_to(&mut board, (0, 3), (2, 5));
    println!("Can move: {}, Game over: {}", can_move, result);
    draw(&board);
    

    // let (can_move, result) = piece::move_to(&mut board, (0, 3), (6, 3));
    // draw(&board);
    // println!("Can move: {}, Game over: {}", can_move, result);
}

fn draw(board: &[[Piece; 8]; 8]) {
    print!("|---|---|---|---|---|---|---|---|\n");
    io::stdout().flush().unwrap();
    for y in 0..8 {
        print!("|");
        for x in 0..8 {
            let (c, t) = board[y][x].to_string();
            print!("{}{} |", c, t);
        }
        print!("\n|---|---|---|---|---|---|---|---|\n");
        io::stdout().flush().unwrap();
    }
    println!("\n");
}