mod piece;
mod filler_ai;
mod piece;
mod player;

use std::fs::OpenOptions;
use std::io::{self, Write, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::filler_ai::FillerAi;

fn log_to_file(filename: &str, message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename) 
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        writeln!(file, "[{}] {}", timestamp, message).ok();
    }
}

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
