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
