// Represents a game piece with dimensions and a 2D character pattern
// Used to store the shape and layout of pieces in the Filler game
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<Vec<char>>,
}

impl Piece {
    // Creates a new Piece instance with the specified dimensions and pattern
    // Takes width, height, and a 2D vector representing the piece's shape
    pub fn new(width: usize, height: usize, pattern: Vec<Vec<char>>) -> Self {
        Self {
            width,
            height,
            pattern,
        }
    }
}