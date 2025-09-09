mod piece;
mod filler_ai;
mod player;

fn main() {

    std::fs::write("game_input.log", "").ok();
    std::fs::write("ai_decisions.log", "").ok();
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(player_line)) = lines.next() {
        let player_number = if player_line.contains("p1") { 1 } else { 2 };
        let mut ai = FillerAi::new(player_number);
    }
}
