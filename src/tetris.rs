use rand::prelude::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrominoType {
    I, O, T, S, Z, J, L,
}

impl TetrominoType {
    pub fn color(&self) -> &'static str {
        match self {
            TetrominoType::I => "#00FFFF", // Cyan
            TetrominoType::O => "#FFFF00", // Yellow
            TetrominoType::T => "#800080", // Purple
            TetrominoType::S => "#00FF00", // Green
            TetrominoType::Z => "#FF0000", // Red
            TetrominoType::J => "#0000FF", // Blue
            TetrominoType::L => "#FFA500", // Orange
        }
    }

    pub fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            TetrominoType::I => vec![
                vec![false, false, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
            ],
            TetrominoType::O => vec![
                vec![true, true],
                vec![true, true],
            ],
            TetrominoType::T => vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, false, false],
            ],
            TetrominoType::S => vec![
                vec![false, true, true],
                vec![true, true, false],
                vec![false, false, false],
            ],
            TetrominoType::Z => vec![
                vec![true, true, false],
                vec![false, true, true],
                vec![false, false, false],
            ],
            TetrominoType::J => vec![
                vec![true, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ],
            TetrominoType::L => vec![
                vec![false, false, true],
                vec![true, true, true],
                vec![false, false, false],
            ],
        }
    }

    pub fn random() -> Self {
        let types = [
            TetrominoType::I, TetrominoType::O, TetrominoType::T,
            TetrominoType::S, TetrominoType::Z, TetrominoType::J, TetrominoType::L
        ];
        let mut rng = rand::thread_rng();
        types[rng.gen_range(0..types.len())]
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub piece_type: TetrominoType,
    pub shape: Vec<Vec<bool>>,
    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(piece_type: TetrominoType) -> Self {
        Self {
            piece_type,
            shape: piece_type.shape(),
            x: 3,
            y: 0,
        }
    }

    pub fn rotate(&self) -> Self {
        let rows = self.shape.len();
        let cols = self.shape[0].len();
        let mut rotated = vec![vec![false; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                rotated[j][rows - 1 - i] = self.shape[i][j];
            }
        }

        Self {
            piece_type: self.piece_type,
            shape: rotated,
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug)]
pub struct TetrisGame {
    pub board: [[Option<TetrominoType>; BOARD_WIDTH]; BOARD_HEIGHT],
    pub current_piece: Option<Piece>,
    pub next_piece: Piece,
    pub score: u32,
    pub lines: u32,
    pub level: u32,
    pub is_game_over: bool,
    pub is_paused: bool,
    pub drop_timer: u32,
    pub drop_interval: u32,
}

impl TetrisGame {
    pub fn new() -> Self {
        let mut game = Self {
            board: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: None,
            next_piece: Piece::new(TetrominoType::random()),
            score: 0,
            lines: 0,
            level: 0,
            is_game_over: false,
            is_paused: false,
            drop_timer: 0,
            drop_interval: 60, // 60 ticks at 60fps = 1 second
        };
        game.spawn_next_piece();
        game
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn spawn_next_piece(&mut self) {
        let new_piece = Piece::new(self.next_piece.piece_type);
        self.next_piece = Piece::new(TetrominoType::random());

        if self.is_valid_position(&new_piece) {
            self.current_piece = Some(new_piece);
        } else {
            self.is_game_over = true;
        }
    }

    pub fn is_valid_position(&self, piece: &Piece) -> bool {
        for (row, shape_row) in piece.shape.iter().enumerate() {
            for (col, &cell) in shape_row.iter().enumerate() {
                if cell {
                    let board_x = piece.x + col as i32;
                    let board_y = piece.y + row as i32;

                    if board_x < 0 || board_x >= BOARD_WIDTH as i32 ||
                       board_y < 0 || board_y >= BOARD_HEIGHT as i32 {
                        return false;
                    }

                    if self.board[board_y as usize][board_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn move_left(&mut self) -> bool {
        if self.is_paused || self.is_game_over {
            return false;
        }

        if let Some(piece) = &self.current_piece {
            let mut new_piece = piece.clone();
            new_piece.x -= 1;
            if self.is_valid_position(&new_piece) {
                self.current_piece = Some(new_piece);
                return true;
            }
        }
        false
    }

    pub fn move_right(&mut self) -> bool {
        if self.is_paused || self.is_game_over {
            return false;
        }

        if let Some(piece) = &self.current_piece {
            let mut new_piece = piece.clone();
            new_piece.x += 1;
            if self.is_valid_position(&new_piece) {
                self.current_piece = Some(new_piece);
                return true;
            }
        }
        false
    }

    pub fn soft_drop(&mut self) -> bool {
        if self.is_paused || self.is_game_over {
            return false;
        }

        if let Some(piece) = &self.current_piece {
            let mut new_piece = piece.clone();
            new_piece.y += 1;
            if self.is_valid_position(&new_piece) {
                self.current_piece = Some(new_piece);
                self.score += 1;
                return true;
            }
        }
        false
    }

    pub fn hard_drop(&mut self) -> u32 {
        if self.is_paused || self.is_game_over {
            return 0;
        }

        let mut distance = 0;
        while self.soft_drop() {
            distance += 1;
        }

        if distance > 0 {
            self.place_current_piece();
        }

        distance
    }

    pub fn rotate(&mut self) -> bool {
        if self.is_paused || self.is_game_over {
            return false;
        }

        if let Some(piece) = &self.current_piece {
            let rotated_piece = piece.rotate();
            if self.is_valid_position(&rotated_piece) {
                self.current_piece = Some(rotated_piece);
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self) {
        if self.is_paused || self.is_game_over {
            return;
        }

        self.drop_timer += 1;
        if self.drop_timer >= self.drop_interval {
            self.drop_timer = 0;
            if !self.soft_drop() {
                self.place_current_piece();
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn get_ghost_piece(&self) -> Piece {
        if let Some(piece) = &self.current_piece {
            let mut ghost = piece.clone();
            while self.is_valid_position(&{
                let mut test_piece = ghost.clone();
                test_piece.y += 1;
                test_piece
            }) {
                ghost.y += 1;
            }
            ghost
        } else {
            Piece::new(TetrominoType::I) // Fallback
        }
    }

    fn place_current_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            // Place piece on board
            for (row, shape_row) in piece.shape.iter().enumerate() {
                for (col, &cell) in shape_row.iter().enumerate() {
                    if cell {
                        let board_x = piece.x + col as i32;
                        let board_y = piece.y + row as i32;
                        if board_x >= 0 && board_x < BOARD_WIDTH as i32 &&
                           board_y >= 0 && board_y < BOARD_HEIGHT as i32 {
                            self.board[board_y as usize][board_x as usize] = Some(piece.piece_type);
                        }
                    }
                }
            }

            // Check for completed lines
            self.clear_lines();

            // Spawn next piece
            self.spawn_next_piece();
        }
    }

    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();

        for y in 0..BOARD_HEIGHT {
            if self.board[y].iter().all(|cell| cell.is_some()) {
                lines_to_clear.push(y);
            }
        }

        for &line in &lines_to_clear {
            // Remove the line
            for y in (1..=line).rev() {
                self.board[y] = self.board[y - 1];
            }
            self.board[0] = [None; BOARD_WIDTH];
        }

        let lines_cleared = lines_to_clear.len() as u32;
        if lines_cleared > 0 {
            self.lines += lines_cleared;

            // Calculate score based on Tetris scoring system
            let line_score = match lines_cleared {
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200, // Tetris!
                _ => 0,
            };
            self.score += line_score * (self.level + 1);

            // Update level (every 10 lines)
            let new_level = self.lines / 10;
            if new_level > self.level {
                self.level = new_level;
                // Increase speed (decrease drop interval)
                self.drop_interval = (60 - self.level * 3).max(5);
            }
        }
    }
}