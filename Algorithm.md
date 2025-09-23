# Filler AI Algorithm Documentation

## Overview

This documentation provides a comprehensive step-by-step breakdown of the Filler AI algorithm that uses a heat map strategy to compete effectively against opponent bots, including the challenging terminator bot.

## Algorithm Architecture

The AI operates on a **multi-layered scoring system** with heat mapping as the primary strategy, supplemented by blocking, expansion, and efficiency calculations.

## Core Data Structures

### AI State
```rust
pub struct FillerAi {
    board_width: usize,           // Board dimensions
    board_height: usize,
    board: Vec<Vec<char>>,        // Current board state
    my_player: Player,            // My player info (@/a or $/s)
    opponent_player: Player,      // Opponent player info
    current_piece: Piece,         // Piece to place this turn
    heat_map: Vec<Vec<i32>>,     // Strategic heat values
}
```

### Player Symbols
- **Player 1**: `@` (territory), `a` (last placed)
- **Player 2**: `$` (territory), `s` (last placed)
- **Empty**: `.` (available space)

## Step-by-Step Algorithm Flow

### Phase 1: Game State Update

#### Step 1.1: Board State Processing
```
Input: New board dimensions and grid data
Process:
1. Store board_width and board_height
2. Clone board data to internal Vec<Vec<char>>
3. Initialize heat_map with same dimensions
4. Trigger heat map generation
5. Update visualizer (if enabled)
```

#### Step 1.2: Piece Processing  
```
Input: New piece dimensions and pattern
Process:
1. Store piece width, height, and 2D pattern
2. Pattern uses '.' for empty cells, any other char for solid cells
```

### Phase 2: Heat Map Generation (Core Strategy)

#### Step 2.1: Opponent Position Detection
```
For each cell (x, y) in board:
    If cell == opponent_territory_symbol OR cell == opponent_last_placed_symbol:
        Add (x, y) to opponent_positions list
```

#### Step 2.2: Heat Calculation Using Manhattan Distance
```
For each empty cell (x, y) in board:
    min_distance = INFINITY
    
    For each opponent_position (opp_x, opp_y):
        manhattan_distance = |x - opp_x| + |y - opp_y|
        min_distance = min(min_distance, manhattan_distance)
    
    max_possible_distance = board_width + board_height
    heat_map[y][x] = max_possible_distance - min_distance
```

**Heat Map Logic:**
- **Higher heat value** = Closer to opponent = More strategic
- **Lower heat value** = Further from opponent = Less strategic
- **Zero heat** = Occupied cells (not placeable)

### Phase 3: Valid Move Detection

#### Step 3.1: Placement Validation
```
For each position (x, y) on board:
    overlap_count = 0
    
    For each solid cell in current_piece:
        board_x = x + piece_x
        board_y = y + piece_y
        
        // Boundary check
        If board_x >= board_width OR board_y >= board_height:
            INVALID - Skip this position
        
        board_cell = board[board_y][board_x]
        
        // Opponent collision check
        If board_cell == opponent_territory OR board_cell == opponent_last_placed:
            INVALID - Skip this position
        
        // Own territory overlap check
        If board_cell == my_territory OR board_cell == my_last_placed:
            overlap_count += 1
            If overlap_count > 1:
                INVALID - Skip this position
    
    // Valid if exactly one overlap
    If overlap_count == 1:
        Add (x, y) to valid_moves
```

### Phase 4: Multi-Factor Scoring System

#### Step 4.1: Heat Score Calculation (Primary Strategy - 100x weight)
```
total_heat = 0
solid_cells = 0

For each solid cell in piece at placement (x, y):
    board_x = x + piece_x
    board_y = y + piece_y
    
    If within bounds:
        total_heat += heat_map[board_y][board_x]
        solid_cells += 1

heat_score = total_heat / solid_cells (average heat)
```

#### Step 4.2: Blocking Score Calculation (Secondary Strategy - 20x weight)
```
blocking_score = 0

For each solid cell in piece at placement (x, y):
    For each adjacent position (8 directions):
        If adjacent_cell == opponent_territory OR opponent_last_placed:
            blocking_score += 5
```

#### Step 4.3: Expansion Score Calculation (Tertiary Strategy - 5x weight)
```
expansion_score = 0

For each solid cell in piece at placement (x, y):
    For each adjacent position (8 directions):
        If adjacent_cell == empty ('.')
            expansion_score += 1
```

#### Step 4.4: Piece Efficiency Score (Bonus Strategy - 10x weight)
```
solid_count = 0

For each cell in piece pattern:
    If cell != '.' AND within board bounds:
        solid_count += 1

efficiency_score = solid_count * 2
```

### Phase 5: Best Move Selection

#### Step 5.1: Weighted Score Calculation
```
For each valid_move (x, y):
    heat_score = calculate_heat_score(x, y)
    blocking_score = calculate_blocking_score(x, y)  
    expansion_score = calculate_expansion_score(x, y)
    efficiency_score = calculate_piece_efficiency(x, y)
    
    total_score = heat_score * 100      // Primary: Aggressive positioning
                + blocking_score * 20   // Secondary: Block opponent
                + expansion_score * 5   // Tertiary: Maintain options  
                + efficiency_score * 10 // Bonus: Piece size efficiency
```

#### Step 5.2: Move Selection
```
best_move = None
best_score = NEGATIVE_INFINITY

For each valid_move with calculated total_score:
    If total_score > best_score:
        best_score = total_score
        best_move = current_move

Return best_move (or first valid move if no best found)
```

## Strategic Priorities (Weight Analysis)

### 1. Heat Map Strategy (100x) - PRIMARY
- **Purpose**: Aggressive positioning toward opponent
- **Logic**: Always try to get as close to opponent as possible
- **Effect**: Creates pressure and limits opponent expansion

### 2. Blocking Strategy (20x) - SECONDARY  
- **Purpose**: Prevent opponent expansion
- **Logic**: Prefer positions adjacent to opponent pieces
- **Effect**: Creates barriers around opponent territory

### 3. Expansion Strategy (5x) - TERTIARY
- **Purpose**: Maintain future move options
- **Logic**: Prefer positions with many empty adjacent cells
- **Effect**: Keeps territory flexible for future pieces

### 4. Efficiency Strategy (10x) - BONUS
- **Purpose**: Maximize territory gained per move
- **Logic**: Prefer placements that use larger pieces effectively
- **Effect**: Gains more board control per turn

## Algorithm Complexity

### Time Complexity
- **Heat Map Generation**: O(W × H × N) where N = opponent positions
- **Valid Move Detection**: O(W × H × P) where P = piece cells
- **Scoring**: O(V × P) where V = valid moves
- **Total**: O(W × H × (N + P + V))

### Space Complexity
- **Heat Map**: O(W × H)
- **Board Storage**: O(W × H)  
- **Valid Moves**: O(V) typically much smaller than W × H
- **Total**: O(W × H)

## Key Algorithm Advantages

### 1. Manhattan Distance Efficiency
- Simple calculation: `|x1 - x2| + |y1 - y2|`
- No expensive square root operations
- Accurate for grid-based movement

### 2. Heat Map Caching
- Generated once per turn
- Reused for all move evaluations
- Significant performance improvement

### 3. Weighted Strategy Balance
- Heat map dominates (proven winning strategy)
- Other factors provide fine-tuning
- Prevents over-optimization of secondary strategies

### 4. Aggressive Positioning
- Always seeks opponent proximity
- Creates territorial pressure
- Forces opponent into defensive positions

## Decision Flow Summary

```
1. Receive new board state and piece
2. Generate heat map based on opponent positions
3. Find all valid piece placements (exactly 1 overlap rule)
4. For each valid placement:
   a. Calculate heat score (distance to opponent)
   b. Calculate blocking score (adjacent to opponent)  
   c. Calculate expansion score (adjacent to empty)
   d. Calculate efficiency score (piece size bonus)
5. Combine scores with strategic weights
6. Select highest scoring placement
7. Output coordinates to game engine
```

## Performance Characteristics

### Strengths
- **Consistent aggressive play** toward opponent
- **Efficient calculation** using Manhattan distance
- **Multi-factor decision making** prevents one-dimensional play

### Strategic Focus
- **Primary Goal**: Get as close to opponent as possible (heat map)
- **Secondary Goal**: Block opponent expansion (blocking score)
- **Tertiary Goal**: Maintain flexibility (expansion score)
- **Bonus Goal**: Maximize territory efficiency (piece size)

This algorithm successfully balances aggressive positioning with tactical awareness, making it competitive against advanced bots while maintaining computational efficiency.