use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn parse_board_header(line: &str) -> Option<(usize, usize)> {
    if let Some(dimensions) = line.strip_prefix("Anfield ") {
        if let Some(colon_pos) = dimensions.find(':') {
            let dims = &dimensions[..colon_pos];
            let parts: Vec<&str> = dims.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(width), Ok(height)) = (parts[0].parse(), parts[1].parse()) {
                    return Some((width, height));
                }
            }
        }
    }
    None
}

pub fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
    if let Some(dimensions) = line.strip_prefix("Piece ") {
        if let Some(colon_pos) = dimensions.find(':') {
            let dims = &dimensions[..colon_pos];
            let parts: Vec<&str> = dims.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(width), Ok(height)) = (parts[0].parse(), parts[1].parse()) {
                    return Some((width, height));
                }
            }
        }
    }
    None
}

pub fn log_to_file(filename: &str, message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(filename) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        writeln!(file, "[{}] {}", timestamp, message).ok();
    }
}
