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
    // Heat map for strategic placement
    heat_map: Vec<Vec<i32>>,
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
            heat_map: Vec::new(),
        }
    }

    // Updates the AI's internal board state with new dimensions and layout
    // Replaces the existing board data with the provided width, height, and grid
    pub fn update_board(&mut self, width: usize, height: usize, board: Vec<Vec<char>>) {
        self.board_width = width;
        self.board_height = height;
        self.board = board.clone();
        
        // Initialize heat map
        self.heat_map = vec![vec![0; width]; height];
        self.generate_heat_map();

        // Update visualizer with new board state
        get_visualizer().update_board(width, height, board);
    }

    // Updates the current piece that the AI needs to place
    // Replaces the existing piece with the new piece data
    pub fn update_piece(&mut self, piece: Piece) {
        self.current_piece = piece;
    }

    // Generate heat map based on distance to opponent territory
    fn generate_heat_map(&mut self) {
        // Reset heat map
        for row in &mut self.heat_map {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }

        // Find all opponent positions
        let mut opponent_positions = Vec::new();
        for y in 0..self.board_height {
            for x in 0..self.board_width {
                let cell = self.board[y][x];
                if cell == self.opponent_player.territory_symbol 
                    || cell == self.opponent_player.last_placed_symbol {
                    opponent_positions.push((x, y));
                }
            }
        }

        if opponent_positions.is_empty() {
            return;
        }

        // Calculate heat for each cell based on Manhattan distance to nearest opponent
        for y in 0..self.board_height {
            for x in 0..self.board_width {
                // Skip cells already occupied
                let cell = self.board[y][x];
                if cell != '.' {
                    continue;
                }

                // Find minimum Manhattan distance to any opponent cell
                let mut min_distance = i32::MAX;
                for &(opp_x, opp_y) in &opponent_positions {
                    let manhattan_dist = (x as i32 - opp_x as i32).abs() + (y as i32 - opp_y as i32).abs();
                    min_distance = min_distance.min(manhattan_dist);
                }

                // Higher heat = closer to opponent (inverted distance)
                // Use max possible distance minus actual distance for heat value
                let max_possible_dist = (self.board_width + self.board_height) as i32;
                self.heat_map[y][x] = max_possible_dist - min_distance;
            }
        }
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

    // Calculate heat score for a placement based on heat map
    fn calculate_heat_score(&self, placement_x: usize, placement_y: usize) -> i32 {
        let mut total_heat = 0;
        let mut solid_cells = 0;

        for (piece_y, piece_row) in self.current_piece.pattern.iter().enumerate() {
            for (piece_x, piece_char) in piece_row.iter().enumerate() {
                if *piece_char != '.' {
                    let board_x = placement_x + piece_x;
                    let board_y = placement_y + piece_y;

                    if board_x < self.board_width && board_y < self.board_height {
                        total_heat += self.heat_map[board_y][board_x];
                        solid_cells += 1;
                    }
                }
            }
        }

        if solid_cells == 0 { 0 } else { total_heat / solid_cells }
    }

    // Advanced blocking strategy: Try to cut off opponent expansion paths
    fn calculate_blocking_score(&self, placement_x: usize, placement_y: usize) -> i32 {
        let mut blocking_score = 0;

        // Get positions where this piece would be placed
        let mut piece_positions = Vec::new();
        for (piece_y, piece_row) in self.current_piece.pattern.iter().enumerate() {
            for (piece_x, piece_char) in piece_row.iter().enumerate() {
                if *piece_char != '.' {
                    let board_x = placement_x + piece_x;
                    let board_y = placement_y + piece_y;
                    if board_x < self.board_width && board_y < self.board_height {
                        piece_positions.push((board_x, board_y));
                    }
                }
            }
        }

        // Check how many opponent expansion paths this placement would block
        for &(px, py) in &piece_positions {
            // Check surrounding cells for potential opponent expansion
            for dy in -2..=2i32 {
                for dx in -2..=2i32 {
                    let check_x = px as i32 + dx;
                    let check_y = py as i32 + dy;
                    
                    if check_x >= 0 && check_x < self.board_width as i32 
                        && check_y >= 0 && check_y < self.board_height as i32 {
                        let check_x = check_x as usize;
                        let check_y = check_y as usize;
                        let cell = self.board[check_y][check_x];
                        
                        // If there's an opponent piece nearby, this placement blocks their expansion
                        if cell == self.opponent_player.last_placed_symbol
                            || cell == self.opponent_player.territory_symbol {
                            blocking_score += (3 - dx.abs().max(dy.abs())); // Closer = higher score
                        }
                    }
                }
            }
        }

        blocking_score
    }

    // Territory expansion strategy: Prefer moves that expand our territory efficiently
    fn calculate_expansion_score(&self, placement_x: usize, placement_y: usize) -> i32 {
        let mut expansion_score = 0;
        
        // Count how many new empty cells this piece would be adjacent to
        for (piece_y, piece_row) in self.current_piece.pattern.iter().enumerate() {
            for (piece_x, piece_char) in piece_row.iter().enumerate() {
                if *piece_char != '.' {
                    let board_x = placement_x + piece_x;
                    let board_y = placement_y + piece_y;
                    
                    if board_x < self.board_width && board_y < self.board_height {
                        // Check adjacent cells for expansion potential
                        for dy in -1..=1i32 {
                            for dx in -1..=1i32 {
                                if dx == 0 && dy == 0 { continue; }
                                
                                let adj_x = board_x as i32 + dx;
                                let adj_y = board_y as i32 + dy;
                                
                                if adj_x >= 0 && adj_x < self.board_width as i32
                                    && adj_y >= 0 && adj_y < self.board_height as i32 {
                                    let adj_x = adj_x as usize;
                                    let adj_y = adj_y as usize;
                                    
                                    if self.board[adj_y][adj_x] == '.' {
                                        expansion_score += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        expansion_score
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
