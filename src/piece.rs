pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(width: usize, height: usize, pattern: Vec<Vec<char>>) -> Self {
        Self {
            width,
            height,
            pattern,
        }
    }
}
