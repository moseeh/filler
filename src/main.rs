mod filler_ai;
mod piece;
mod player;
mod utils;

use std::io::{self, BufRead};

use crate::filler_ai::FillerAi;
use crate::utils::*;
use crate::piece::*;

fn main() {
    std::fs::write("game_input.log", "").ok();
    std::fs::write("ai_decisions.log", "").ok();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(player_line)) = lines.next() {
        let player_number = if player_line.contains("p1") { 1 } else { 2 };
        let mut ai = FillerAi::new(player_number);

        loop {
            if let Some(Ok(board_header)) = lines.next() {
                if let Some((width, height)) = parse_board_header(&board_header) {
                    // Skip column number line
                    if let Some(Ok(_col_line)) = lines.next() {}

                    let mut board = Vec::new();
                    for _ in 0..height {
                        if let Some(Ok(board_row)) = lines.next() {
                            if let Some(space_pos) = board_row.find(" ") {
                                let row_data: Vec<char> =
                                    board_row[(space_pos + 1)..].chars().collect();
                                board.push(row_data);
                            }
                        }
                    }
                    ai.update_board(width, height, board);

                    if let Some(Ok(piece_header)) = lines.next() {
                        if let Some((piece_width, piece_height)) = parse_piece_header(&piece_header)
                        {
                            // Parse piece pattern
                            let mut piece_pattern = Vec::new();
                            for _ in 0..piece_height {
                                if let Some(Ok(piece_row)) = lines.next() {
                                    let pattern_row: Vec<char> = piece_row.chars().collect();
                                    piece_pattern.push(pattern_row);
                                }
                            }
                            let piece = Piece::new(piece_width, piece_height, piece_pattern);
                            ai.update_piece(piece);
                        }
                    }
                }
            }
        }
    }
}
