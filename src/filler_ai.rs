use crate::piece::{self, Piece};
use crate::player::Player;

// AI struct that manages game state for the Filler game
// Contains board data, player information, and current piece details
pub struct FillerAi {
    // Board data
    pub board_width: usize,
    pub board_height: usize,
    pub board: Vec<Vec<char>>,
    // Player info
    pub my_player: Player,
    pub opponent_player: Player,
    // Piece info
    pub current_piece: Piece,
}

impl FillerAi {
    // Creates a new FillerAi instance with the specified player number
    // Automatically determines the opponent's player number (1 or 2)
    pub fn new(my_player_number: u8) -> Self {
        let opponent_number = if my_player_number == 1 { 2 } else { 1 };
        Self {
            board_width: 0,
            board_height: 0,
            board: Vec::new(),
            my_player: Player::new(my_player_number),
            opponent_player: Player::new(opponent_number),
            current_piece: Piece::new(0, 0, Vec::new()),
        }
    }

    // Updates the AI's internal board state with new dimensions and layout
    // Replaces the existing board data with the provided width, height, and grid
    pub fn update_board(&mut self, width: usize, height: usize, board: Vec<Vec<char>>) {
        self.board_width = width;
        self.board_height = height;
        self.board = board;
    }

    // Updates the current piece that the AI needs to place
    // Replaces the existing piece with the new piece data
    pub fn update_piece(&mut self, piece: Piece) {
        self.current_piece = piece;
    }

    // Find all valid placements for current piece
    fn find_all_valid_placements(&self) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        for y in 0..self.board_height {
            'outer: for x in 0..self.board_width {
                let mut cell_overlap_count = 0;

                // Check each solid cell in the piece
                for (piece_y, piece_row) in self.current_piece.pattern.iter().enumerate() {
                    for (piece_x, piece_char) in piece_row.iter().enumerate() {
                        if *piece_char == '.' {
                            continue; // Skip empty piece cells
                        }

                        // Calculate where this piece cell would land on board
                        let board_x = x + piece_x;
                        let board_y = y + piece_y;

                        // Check bounds
                        if board_x >= self.board_width || board_y >= self.board_height {
                            continue 'outer;
                        }

                        let board_cell = self.board[board_y][board_x];

                        // Check opponent collision
                        if board_cell == self.opponent_player.last_placed_symbol
                            || board_cell == self.opponent_player.territory_symbol
                        {
                            continue 'outer;
                        }

                        // Count overlaps with my territory
                        if board_cell == self.my_player.last_placed_symbol
                            || board_cell == self.my_player.territory_symbol
                        {
                            cell_overlap_count += 1;
                            if cell_overlap_count > 1 {
                                continue 'outer;
                            }
                        }
                    }
                }

                // Valid placement if exactly one overlap
                if cell_overlap_count == 1 {
                    valid_moves.push((x, y));
                }
            }
        }

        valid_moves
    }

    
}
