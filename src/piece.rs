use std::fmt;

#[derive(Copy, Clone)]
pub struct Piece(pub Type, pub Colour);

#[derive(Copy, Clone, PartialEq)]
pub enum Type {
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Pawn,
    None
}

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    White,
    Black,
    None
}

impl Piece {
    pub fn to_debug_string(&self) -> String {
        let which_type: &str = match self.0 {
            Type::Rook => "Rook",
            Type::Knight => "Knight",
            Type::Bishop => "Bishop",
            Type::King => "King",
            Type::Queen => "Queen",
            Type::Pawn => "Pawn",
            Type::None => "Piece"
        };
        let which_colour: &str = match self.1 {
            Colour::White => "White",
            Colour::Black => "Black",
            Colour::None => "Blank",
        };
        String::from(format!("{} {}", which_colour, which_type))
    }

    pub fn to_string(&self) -> (char, char) {
        let which_type: char = match self.0 {
            Type::Rook => 'I',
            Type::Knight => 'Z',
            Type::Bishop => 'T',
            Type::King => 'K',
            Type::Queen => 'Q',
            Type::Pawn => 'P',
            Type::None => ' '
        };
        let which_colour: char = match self.1 {
            Colour::White => ',',
            Colour::Black => '\'',
            Colour::None => ' ',
        };
        (which_colour, which_type)
    }
}

// the result is (can move?, game ended?)
pub fn move_to(board: &mut [[Piece; 8]; 8], move_from: (usize, usize), move_to: (usize, usize)) -> (bool, bool) {
    let empty_piece = Piece(Type::None, Colour::None);
    let piece: Piece = board[move_from.1][move_from.0];

    if can_move_to(board, &move_from, &move_to) {
        board[move_from.1][move_from.0] = empty_piece;
        
        // if the piece is a king, end the game
        if board[move_to.1][move_to.0].0 == Type::King {
            board[move_to.1][move_to.0] = piece;
            return (true, true);    
        }

        board[move_to.1][move_to.0] = piece;
        return (true, false);
    }

    (false, false)
}

pub fn can_move_to(board: &[[Piece; 8]; 8], move_from: &(usize, usize), move_to: &(usize, usize)) -> bool {
    let move_from = *move_from;
    let move_to = *move_to;

    let piece: Piece = board[move_from.1][move_from.0];

    let x1 = if move_from.0 > move_to.0 { move_to.0 } else { move_from.0 };
    let x2 = if move_from.0 < move_to.0 { move_to.0 } else { move_from.0 };
    let y1 = if move_from.1 > move_to.1 { move_to.1 } else { move_from.1 };
    let y2 = if move_from.1 < move_to.1 { move_to.1 } else { move_from.1 };

    match piece.0 {
        Type::Rook => {
            // if the positions are not only aligned horizontally or vertically
            if !(move_from.0 == move_to.0 || move_from.1 == move_to.1) {
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
            if board[move_to.1][move_to.0].1 == piece.1 {
                return false;
            }

            return true;
        },
        Type::Knight => {
            // new scope so that the shadowed vars (move_from and move_to) are 
            // converted back to their original types and all other vars are discarded
            {
                // shadow the variables into isize type for easy addition
                let move_from: (isize, isize) = (move_from.0 as isize, move_from.1 as isize);
                let move_to: (isize, isize) = (move_to.0 as isize, move_to.1 as isize);

                let possible_moves: [(isize, isize); 8] = [(-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1)];
                let mut can_move: bool = false;
                
                // iterate through the moves and check if any are equal to the move_to position
                for possible_move in possible_moves {
                    if ((move_from.0) + possible_move.0, (move_from.1) + possible_move.1) == move_to {
                        can_move = true;
                        break;
                    }
                }

                if !can_move {
                    return false;
                }
            }

            // if the final position's piece colour the same colour as the piece
            if board[move_to.1][move_to.0].1 == piece.1 {
                return false;
            }

            return true;
        },
        Type::Bishop => {

        },
        Type::King => {

        },
        Type::Queen => {

        },
        Type::Pawn => {

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