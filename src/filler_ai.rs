use crate::piece::Piece;
use crate::player::Player;

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
    pub fn update_board(&mut self, width: usize, height: usize, board: Vec<Vec<char>>) {
        self.board_width = width;
        self.board_height = height;
        self.board = board;
    }
}
