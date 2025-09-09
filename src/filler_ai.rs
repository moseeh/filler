struct FillerAi {
    // Board data
    pub board_width : usize,
    pub board_height: usize,
    pub board: Vec<Vec<char>>,

    // Player info
    pub my_player_number : u8, 
    pub my_player_symbol: char,
    pub opponent_symbol: char, 
}