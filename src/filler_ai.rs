use crate::piece::Piece;
use crate::player::Player;
use crate::visualizer::get_visualizer;

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
        self.board = board.clone();

        // Update visualizer with new board state
        get_visualizer().update_board(width, height, board);
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

    // Find opponent's latest piece positions
    fn find_opponent_latest_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for (y, row) in self.board.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == self.opponent_player.last_placed_symbol {
                    positions.push((x, y));
                }
            }
        }

        positions
    }

    // Find my latest piece positions
    fn find_my_latest_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for (y, row) in self.board.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == self.my_player.last_placed_symbol {
                    positions.push((x, y));
                }
            }
        }

        positions
    }

    // Calculate Euclidean distance between two points
    fn euclidean_distance(&self, pos1: (usize, usize), pos2: (usize, usize)) -> f64 {
        let dx = (pos1.0 as f64) - (pos2.0 as f64);
        let dy = (pos1.1 as f64) - (pos2.1 as f64);
        (dx * dx + dy * dy).sqrt()
    }

    // Get all positions where this placement would put solid piece cells
    fn get_piece_solid_positions(
        &self,
        placement_x: usize,
        placement_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut solid_positions = Vec::new();

        for (piece_y, piece_row) in self.current_piece.pattern.iter().enumerate() {
            for (piece_x, piece_char) in piece_row.iter().enumerate() {
                if *piece_char != '.' {
                    let board_x = placement_x + piece_x;
                    let board_y = placement_y + piece_y;
                    solid_positions.push((board_x, board_y));
                }
            }
        }

        solid_positions
    }

    // Find minimum distance from any of my new piece positions to any opponent latest positions
    fn min_distance_to_opponent_latest(
        &self,
        placement_x: usize,
        placement_y: usize,
        opponent_positions: &[(usize, usize)],
    ) -> f64 {
        if opponent_positions.is_empty() {
            return f64::INFINITY;
        }

        let my_solid_positions = self.get_piece_solid_positions(placement_x, placement_y);
        let mut min_distance = f64::INFINITY;

        for my_pos in &my_solid_positions {
            for opp_pos in opponent_positions {
                let distance = self.euclidean_distance(*my_pos, *opp_pos);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }

        min_distance
    }

    // Find minimum distance from any of my new piece positions to any of my latest positions
    fn min_distance_to_my_latest(
        &self,
        placement_x: usize,
        placement_y: usize,
        my_positions: &[(usize, usize)],
    ) -> f64 {
        if my_positions.is_empty() {
            return f64::INFINITY;
        }

        let my_solid_positions = self.get_piece_solid_positions(placement_x, placement_y);
        let mut min_distance = f64::INFINITY;

        for new_pos in &my_solid_positions {
            for latest_pos in my_positions {
                let distance = self.euclidean_distance(*new_pos, *latest_pos);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }

        min_distance
    }

    pub fn find_best_move(&self) -> Option<(usize, usize)> {
        let valid_moves = self.find_all_valid_placements();

        if valid_moves.is_empty() {
            return None;
        }

        // Strategy: Latest vs Latest aggressive approach with fallback
        let opponent_latest = self.find_opponent_latest_positions();

        if !opponent_latest.is_empty() {
            // Primary strategy: Move toward opponent's latest piece
            let mut best_move = None;
            let mut closest_distance = f64::INFINITY;

            for &(x, y) in &valid_moves {
                let distance = self.min_distance_to_opponent_latest(x, y, &opponent_latest);
                if distance < closest_distance {
                    closest_distance = distance;
                    best_move = Some((x, y));
                }
            }

            if let Some(move_pos) = best_move {
                return Some(move_pos);
            }
        }

        // Fallback strategy: Move close to my latest piece
        let my_latest = self.find_my_latest_positions();

        if !my_latest.is_empty() {
            let mut best_move = None;
            let mut closest_distance = f64::INFINITY;

            for &(x, y) in &valid_moves {
                let distance = self.min_distance_to_my_latest(x, y, &my_latest);
                if distance < closest_distance {
                    closest_distance = distance;
                    best_move = Some((x, y));
                }
            }

            if let Some(move_pos) = best_move {
                return Some(move_pos);
            }
        }

        // Ultimate fallback: Just return first valid move
        Some(valid_moves[0])
    }
}
