mod filler_ai;
mod piece;
mod player;
mod utils;

use std::io::{self, BufRead};

use crate::filler_ai::FillerAi;
use crate::piece::*;
use crate::utils::*;

fn main() {
    std::fs::write("game_input.log", "").ok();
    std::fs::write("ai_decisions.log", "").ok();

    log_to_file("ai_decisions.log", "=== AI STARTING ===");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(player_line)) = lines.next() {
        log_to_file("game_input.log", &player_line);

        let player_number = if player_line.contains("p1") { 1 } else { 2 };
        log_to_file(
            "ai_decisions.log",
            &format!("I am player {}", player_number),
        );

        let mut ai = FillerAi::new(player_number);
        log_to_file("ai_decisions.log", "FillerAi initialized");

        loop {
            log_to_file("ai_decisions.log", "--- Starting new turn ---");

            // Parse board header
            if let Some(Ok(board_header)) = lines.next() {
                log_to_file("game_input.log", &board_header);

                if let Some((width, height)) = parse_board_header(&board_header) {
                    log_to_file(
                        "ai_decisions.log",
                        &format!("Board dimensions: {}x{}", width, height),
                    );

                    // Skip column number line
                    if let Some(Ok(col_line)) = lines.next() {
                        log_to_file("game_input.log", &col_line);
                    }

                    // Parse board data
                    let mut board = Vec::new();
                    for _ in 0..height {
                        if let Some(Ok(board_row)) = lines.next() {
                            log_to_file("game_input.log", &board_row);

                            // Extract actual board data (skip row number prefix)
                            if let Some(space_pos) = board_row.find(' ') {
                                let row_data: Vec<char> =
                                    board_row[(space_pos + 1)..].chars().collect();
                                board.push(row_data);
                            }
                        }
                    }

                    // Update AI with board data
                    ai.update_board(width, height, board);
                    log_to_file("ai_decisions.log", "Board data updated");

                    // Parse piece header
                    if let Some(Ok(piece_header)) = lines.next() {
                        log_to_file("game_input.log", &piece_header);

                        if let Some((piece_width, piece_height)) = parse_piece_header(&piece_header)
                        {
                            log_to_file(
                                "ai_decisions.log",
                                &format!("Piece dimensions: {}x{}", piece_width, piece_height),
                            );

                            // Parse piece pattern
                            let mut piece_pattern = Vec::new();
                            for _ in 0..piece_height {
                                if let Some(Ok(piece_row)) = lines.next() {
                                    log_to_file("game_input.log", &piece_row);
                                    let pattern_row: Vec<char> = piece_row.chars().collect();
                                    piece_pattern.push(pattern_row);
                                }
                            }

                            // Create piece and update AI
                            let piece = Piece::new(piece_width, piece_height, piece_pattern);
                            ai.update_piece(piece);
                            log_to_file("ai_decisions.log", "Piece data updated");

                            // Make a move (placeholder logic for now)
                            log_to_file("ai_decisions.log", "Calculating move...");
                            if let Some((move_x, move_y)) = ai.find_best_move() {
                                log_to_file(
                                    "ai_decisions.log",
                                    &format!("Making move: {} {}", move_x, move_y),
                                );
                                println!("{} {}", move_x, move_y);
                            } else {
                                log_to_file(
                                    "ai_decisions.log",
                                    "No best move found for this piece",
                                );
                                println!("0 0");
                            }
                        }
                    }
                } else {
                    log_to_file("ai_decisions.log", "Failed to parse board header");
                    break;
                }
            } else {
                log_to_file("ai_decisions.log", "No more input - game ended");
                break;
            }
        }
    }

    log_to_file("ai_decisions.log", "=== AI ENDING ===");
}
