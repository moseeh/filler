pub struct Player {
    pub number: u8,
    pub territory_symbol: char,    // '@' or '$'
    pub last_placed_symbol: char,  // 'a' or 's'
}

impl Player {
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