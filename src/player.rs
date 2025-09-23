// Represents a player in the Filler game with their identifying symbols
// Stores player number and the characters used to mark their territory and placements
#[allow(dead_code)]
pub struct Player {
    pub number: u8,
    pub territory_symbol: char, // '@' or '$'
    pub last_placed_symbol: char, // 'a' or 's'
}

impl Player {
    // Creates a new Player instance with symbols based on player number
    // Player 1 gets '@' and 'a', Player 2 gets '$' and 's'
    pub fn new(number: u8) -> Self{
        if number == 1 {
            Self{
                number: 1,
                territory_symbol: '@',
                last_placed_symbol: 'a',
            }
        } else {
            Self{
                number: 2,
                territory_symbol: '$',
                last_placed_symbol: 's',
            }
        }
    }
}