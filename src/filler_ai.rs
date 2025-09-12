use crate::piece::Piece;
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
}