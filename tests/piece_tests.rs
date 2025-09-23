// tests/piece_tests.rs
use filler::piece::Piece;

#[test]
fn test_piece_creation() {
    let pattern = vec![vec!['O', '.'], vec!['.', 'O']];

    let piece = Piece::new(2, 2, pattern.clone());

    assert_eq!(piece.width, 2);
    assert_eq!(piece.height, 2);
    assert_eq!(piece.pattern, pattern);
}

#[test]
fn test_empty_piece() {
    let pattern = vec![vec!['.', '.'], vec!['.', '.']];

    let piece = Piece::new(2, 2, pattern.clone());

    assert_eq!(piece.width, 2);
    assert_eq!(piece.height, 2);

    // Verify all cells are empty
    for row in &piece.pattern {
        for &cell in row {
            assert_eq!(cell, '.');
        }
    }
}

#[test]
fn test_solid_piece() {
    let pattern = vec![vec!['O', 'O'], vec!['O', 'O']];

    let piece = Piece::new(2, 2, pattern.clone());

    // Verify all cells are solid
    for row in &piece.pattern {
        for &cell in row {
            assert_eq!(cell, 'O');
        }
    }
}

#[test]
fn test_tetris_l_piece() {
    let pattern = vec![vec!['O', '.'], vec!['O', '.'], vec!['O', 'O']];

    let piece = Piece::new(2, 3, pattern.clone());

    assert_eq!(piece.width, 2);
    assert_eq!(piece.height, 3);
    assert_eq!(piece.pattern[0][0], 'O');
    assert_eq!(piece.pattern[0][1], '.');
    assert_eq!(piece.pattern[2][1], 'O');
}

#[test]
fn test_single_cell_piece() {
    let pattern = vec![vec!['O']];
    let piece = Piece::new(1, 1, pattern);

    assert_eq!(piece.width, 1);
    assert_eq!(piece.height, 1);
    assert_eq!(piece.pattern[0][0], 'O');
}

#[test]
fn test_horizontal_line_piece() {
    let pattern = vec![vec!['O', 'O', 'O', 'O']];
    let piece = Piece::new(4, 1, pattern);

    assert_eq!(piece.width, 4);
    assert_eq!(piece.height, 1);

    for &cell in &piece.pattern[0] {
        assert_eq!(cell, 'O');
    }
}

#[test]
fn test_vertical_line_piece() {
    let pattern = vec![vec!['O'], vec!['O'], vec!['O']];

    let piece = Piece::new(1, 3, pattern);

    assert_eq!(piece.width, 1);
    assert_eq!(piece.height, 3);

    for row in &piece.pattern {
        assert_eq!(row[0], 'O');
    }
}

#[test]
fn test_complex_piece_pattern() {
    let pattern = vec![
        vec!['.', 'O', '.'],
        vec!['O', 'O', 'O'],
        vec!['.', 'O', '.'],
    ];

    let piece = Piece::new(3, 3, pattern.clone());

    assert_eq!(piece.pattern, pattern);

    // Test specific positions
    assert_eq!(piece.pattern[0][1], 'O'); // Top center
    assert_eq!(piece.pattern[1][0], 'O'); // Middle left
    assert_eq!(piece.pattern[1][1], 'O'); // Center
    assert_eq!(piece.pattern[0][0], '.'); // Top left corner
}
