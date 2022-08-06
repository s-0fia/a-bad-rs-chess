use std::fmt;
use crate::lib::{min, max, min_max};

#[derive(Copy, Clone)]
pub struct Piece(pub Type, pub Colour);

#[derive(Copy, Clone, PartialEq)]
pub enum Type {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
    None
}

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    White,
    None
}

impl Piece {
    pub fn to_debug_string(&self) -> String {
        let which_type: &str = match self.0 {
            Type::Bishop => "Bishop",
            Type::King => "King",
            Type::Knight => "Knight",
            Type::Pawn => "Pawn",
            Type::Queen => "Queen",
            Type::Rook => "Rook",
            Type::None => "Piece"
        };
        let which_colour: &str = match self.1 {
            Colour::Black => "Black",
            Colour::White => "White",
            Colour::None => "Blank",
        };
        String::from(format!("{} {}", which_colour, which_type))
    }

    pub fn to_string(&self) -> (char, char) {
        let which_type: char = match self.0 {
            Type::Bishop => 'T',
            Type::King => 'K',
            Type::Knight => 'Z',
            Type::Pawn => 'P',
            Type::Queen => 'Q',
            Type::Rook => 'I',
            Type::None => ' '
        };
        let which_colour: char = match self.1 {
            Colour::Black => '\'',
            Colour::White => ',',
            Colour::None => ' ',
        };
        (which_colour, which_type)
    }
}

// this function makes a piece go from *from to *to and handles replacement of pieces, the result is (move made, game ended)
pub fn move_to(board: &mut [[Piece; 8]; 8], from: (usize, usize), to: (usize, usize)) -> (bool, bool) {
    let empty_piece = Piece(Type::None, Colour::None);
    let piece: Piece = board[from.1][from.0];

    if can_move_to(board, &from, &to) {
        board[from.1][from.0] = empty_piece;
        
        // if the piece is a king, end the game
        let game_over: bool = board[to.1][to.0].0 == Type::King;

        board[to.1][to.0] = piece;
        return (true, game_over);
    }

    (false, false)
}


// checks if a piece can move from *from to *to, the result is (can make move) 
pub fn can_move_to(board: &[[Piece; 8]; 8], from: &(usize, usize), to: &(usize, usize)) -> bool {
    let from = *from;
    let to = *to;

    let piece: Piece = board[from.1][from.0];

    let (x1, x2) = min_max(from.0, to.0);
    let (y1, y2) = min_max(from.1, to.1);

    match piece.0 {
        Type::Bishop => {

        },
        Type::King => {

        },
        Type::Knight => {
            // new scope so that the shadowed vars (from and to) are 
            // converted back to their original types and all other vars are discarded
            {
                // shadow the variables into isize type for easy addition
                let from: (isize, isize) = (from.0 as isize, from.1 as isize);
                let to: (isize, isize) = (to.0 as isize, to.1 as isize);

                let possible_moves: [(isize, isize); 8] = [(-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1)];
                let mut can_move: bool = false;
                
                // iterate through the moves and check if any are equal to the to position
                for possible_move in possible_moves {
                    if ((from.0) + possible_move.0, (from.1) + possible_move.1) == to {
                        can_move = true;
                        break;
                    }
                }

                if !can_move {
                    return false;
                }
            }

            // if the final position's piece colour the same colour as the piece
            if board[to.1][to.0].1 == piece.1 {
                return false;
            }

            return true;
        },
        Type::Pawn => {

        },
        Type::Queen => {

        },
        Type::Rook => {
            // if the positions are not only aligned horizontally or vertically
            if !(from.0 == to.0 || from.1 == to.1) {
                return false;
            }

            // checks for pieces blocking the path (excluding the last piece)
            // two cases, either the Ys are equal
            if y1 == y2 {
                for x in (x1 + 1)..x2 {
                    if board[y1][x].0 != Type::None {
                        println!("Piece in the way! ({}, {})", x, y1);
                        return false;
                    }
                }
            }
            // or Xs are equal
            else {
                for y in (y1 + 1)..y2 {
                    if board[y][x1].0 != Type::None {
                        println!("Piece in the way! ({}, {})", x1, y);
                        return false;
                    }
                }
            }

            // if the final position's piece colour the same colour as the piece
            if board[to.1][to.0].1 == piece.1 {
                return false;
            }

            return true;
        },
        Type::None => {

        }
    }

    return false;
}

fn in_check(board: &[[Piece; 8]; 8], king_position: (usize, usize)) -> bool {
    false
}

fn in_checkmate(board: &[[Piece; 8]; 8], king_position: (usize, usize)) -> bool {
    false
}