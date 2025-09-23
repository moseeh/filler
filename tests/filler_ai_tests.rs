use filler::filler_ai::FillerAi;
use filler::piece::Piece;

#[test]
fn test_ai_initialization() {
    let ai = FillerAi::new(1);
    assert_eq!(ai.my_player.number, 1);
    assert_eq!(ai.my_player.territory_symbol, '@');
    assert_eq!(ai.my_player.last_placed_symbol, 'a');
    assert_eq!(ai.opponent_player.number, 2);
    assert_eq!(ai.opponent_player.territory_symbol, '$');
}

#[test]
fn test_board_update() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['.', '@', '.', '$'],
        vec!['.', '.', '.', '.'],
        vec!['@', '.', '.', '$'],
    ];

    ai.update_board(4, 3, board.clone());

    assert_eq!(ai.board_width, 4);
    assert_eq!(ai.board_height, 3);
    assert_eq!(ai.board, board);
}

#[test]
fn test_piece_update() {
    let mut ai = FillerAi::new(1);
    let piece = Piece::new(2, 2, vec![vec!['O', '.'], vec!['.', 'O']]);

    ai.update_piece(piece);
    assert_eq!(ai.current_piece.width, 2);
    assert_eq!(ai.current_piece.height, 2);
    assert_eq!(ai.current_piece.pattern[0][0], 'O');
}

#[test]
fn test_heat_map_generation() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['@', '.', '.', '$'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
    ];

    ai.update_board(4, 3, board);

    // Heat should be higher (closer to opponent) on the right side
    assert!(ai.heat_map[1][3] > ai.heat_map[1][0]);
}

#[test]
fn test_valid_placement_detection() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['@', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '$'],
    ];

    ai.update_board(4, 3, board);

    let piece = Piece::new(2, 1, vec![vec!['O', 'O']]);
    ai.update_piece(piece);

    let valid_moves = ai.find_all_valid_placements();
    assert!(!valid_moves.is_empty());

    // Should find valid moves adjacent to '@' position
    assert!(valid_moves.contains(&(0, 0)));
}

#[test]
fn test_no_valid_moves_when_blocked() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['$', '$', '$'],
        vec!['$', '@', '$'],
        vec!['$', '$', '$'],
    ];

    ai.update_board(3, 3, board);

    let piece = Piece::new(2, 1, vec![vec!['O', 'O']]);
    ai.update_piece(piece);

    let valid_moves = ai.find_all_valid_placements();
    assert!(valid_moves.is_empty());
}

#[test]
fn test_heat_score_calculation() {
    let mut ai = FillerAi::new(1);
    let board = vec![vec!['@', '.', '.', '$'], vec!['.', '.', '.', '.']];

    ai.update_board(4, 2, board);

    let piece = Piece::new(1, 1, vec![vec!['O']]);
    ai.update_piece(piece);

    // Position (2, 0) should have higher heat than (1, 0) (closer to opponent)
    let heat_close = ai.calculate_heat_score(2, 0);
    let heat_far = ai.calculate_heat_score(1, 0);

    assert!(heat_close > heat_far);
}

#[test]
fn test_blocking_score_calculation() {
    let mut ai = FillerAi::new(1);
    let board = vec![vec!['@', '.', 's', '.'], vec!['.', '.', '.', '.']];

    ai.update_board(4, 2, board);

    let piece = Piece::new(1, 1, vec![vec!['O']]);
    ai.update_piece(piece);

    // Position (2, 1) should have higher blocking score (adjacent to 's')
    let blocking_adjacent = ai.calculate_blocking_score(2, 1);
    let blocking_far = ai.calculate_blocking_score(0, 1);

    assert!(blocking_adjacent > blocking_far);
}

#[test]
fn test_expansion_score_calculation() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['@', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '$'],
    ];

    ai.update_board(4, 3, board);

    let piece = Piece::new(1, 1, vec![vec!['O']]);
    ai.update_piece(piece);

    // Position (1, 1) should have higher expansion score (more empty neighbors)
    let expansion_center = ai.calculate_expansion_score(1, 1);
    let expansion_corner = ai.calculate_expansion_score(0, 0);

    assert!(expansion_center > expansion_corner);
}

#[test]
fn test_piece_efficiency_scoring() {
    let mut ai = FillerAi::new(1);
    let board = vec![vec!['@', '.', '.', '.'], vec!['.', '.', '.', '.']];

    ai.update_board(4, 2, board);

    // Large piece should have higher efficiency score
    let large_piece = Piece::new(2, 2, vec![vec!['O', 'O'], vec!['O', 'O']]);
    ai.update_piece(large_piece);
    let large_efficiency = ai.calculate_piece_efficiency(1, 0);

    let small_piece = Piece::new(1, 1, vec![vec!['O']]);
    ai.update_piece(small_piece);
    let small_efficiency = ai.calculate_piece_efficiency(1, 0);

    assert!(large_efficiency > small_efficiency);
}

#[test]
fn test_best_move_selection() {
    let mut ai = FillerAi::new(1);
    let board = vec![vec!['@', '.', '.', 's'], vec!['.', '.', '.', '.']];

    ai.update_board(4, 2, board);

    let piece = Piece::new(1, 1, vec![vec!['O']]);
    ai.update_piece(piece);

    let best_move = ai.find_best_move();
    assert!(best_move.is_some());

}

#[test]
fn test_no_moves_returns_none() {
    let mut ai = FillerAi::new(1);
    let board = vec![
        vec!['$', '$', '$'],
        vec!['$', '@', '$'],
        vec!['$', '$', '$'],
    ];

    ai.update_board(3, 3, board);

    let piece = Piece::new(
        3,
        3,
        vec![
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
        ],
    );
    ai.update_piece(piece);

    let best_move = ai.find_best_move();
    assert!(best_move.is_none());
}
