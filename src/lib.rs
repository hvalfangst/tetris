use wasm_bindgen::prelude::*;
use web_sys::*;

mod tetris;
use tetris::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"🎮 Tetris WASM initialized!".into());
}

#[wasm_bindgen]
pub struct WasmTetris {
    game: TetrisGame,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl WasmTetris {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmTetris, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(300);
        canvas.set_height(600);

        let html_element: &HtmlElement = canvas.as_ref();
        html_element.style().set_property("border", "2px solid #40e0d0")?;
        html_element.style().set_property("background", "#000")?;

        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(WasmTetris {
            game: TetrisGame::new(),
            canvas,
            ctx,
        })
    }

    #[wasm_bindgen]
    pub fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas.clone()
    }

    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key_code: &str) {
        match key_code {
            "ArrowLeft" | "KeyA" => { self.game.move_left(); },
            "ArrowRight" | "KeyD" => { self.game.move_right(); },
            "ArrowDown" | "KeyS" => { self.game.soft_drop(); },
            "ArrowUp" | "KeyW" => { self.game.rotate(); },
            "Space" => { self.game.hard_drop(); },
            "KeyP" => { self.game.toggle_pause(); },
            "KeyR" => { self.game.reset(); },
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.game.tick();
        self.render();
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> u32 {
        self.game.score
    }

    #[wasm_bindgen]
    pub fn get_level(&self) -> u32 {
        self.game.level
    }

    #[wasm_bindgen]
    pub fn get_lines(&self) -> u32 {
        self.game.lines
    }

    #[wasm_bindgen]
    pub fn is_game_over(&self) -> bool {
        self.game.is_game_over
    }

    #[wasm_bindgen]
    pub fn is_paused(&self) -> bool {
        self.game.is_paused
    }

    fn render(&self) {
        // Clear canvas
        self.ctx.set_fill_style(&JsValue::from_str("#000"));
        self.ctx.fill_rect(0.0, 0.0, 300.0, 600.0);

        // Draw grid
        self.ctx.set_stroke_style(&JsValue::from_str("#333"));
        self.ctx.set_line_width(0.5);
        for i in 0..=10 {
            let x = i as f64 * 30.0;
            self.ctx.begin_path();
            self.ctx.move_to(x, 0.0);
            self.ctx.line_to(x, 600.0);
            self.ctx.stroke();
        }
        for i in 0..=20 {
            let y = i as f64 * 30.0;
            self.ctx.begin_path();
            self.ctx.move_to(0.0, y);
            self.ctx.line_to(300.0, y);
            self.ctx.stroke();
        }

        // Draw placed pieces
        for (y, row) in self.game.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(piece_type) = cell {
                    self.draw_cell(x as f64, y as f64, piece_type.color());
                }
            }
        }

        // Draw current piece
        if let Some(piece) = &self.game.current_piece {
            for (row, shape_row) in piece.shape.iter().enumerate() {
                for (col, &cell) in shape_row.iter().enumerate() {
                    if cell {
                        let x = piece.x + col as i32;
                        let y = piece.y + row as i32;
                        if x >= 0 && x < 10 && y >= 0 && y < 20 {
                            self.draw_cell(x as f64, y as f64, piece.piece_type.color());
                        }
                    }
                }
            }
        }

        // Draw ghost piece
        if let Some(piece) = &self.game.current_piece {
            let ghost = self.game.get_ghost_piece();
            self.ctx.set_global_alpha(0.3);
            for (row, shape_row) in ghost.shape.iter().enumerate() {
                for (col, &cell) in shape_row.iter().enumerate() {
                    if cell {
                        let x = ghost.x + col as i32;
                        let y = ghost.y + row as i32;
                        if x >= 0 && x < 10 && y >= 0 && y < 20 {
                            if self.game.board[y as usize][x as usize].is_none() {
                                self.draw_cell(x as f64, y as f64, piece.piece_type.color());
                            }
                        }
                    }
                }
            }
            self.ctx.set_global_alpha(1.0);
        }
    }

    fn draw_cell(&self, x: f64, y: f64, color: &str) {
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill_rect(x * 30.0 + 1.0, y * 30.0 + 1.0, 28.0, 28.0);

        // Add highlight for 3D effect
        self.ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.3)"));
        self.ctx.fill_rect(x * 30.0 + 1.0, y * 30.0 + 1.0, 28.0, 4.0);
        self.ctx.fill_rect(x * 30.0 + 1.0, y * 30.0 + 1.0, 4.0, 28.0);

        // Add shadow
        self.ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.3)"));
        self.ctx.fill_rect(x * 30.0 + 25.0, y * 30.0 + 5.0, 4.0, 24.0);
        self.ctx.fill_rect(x * 30.0 + 5.0, y * 30.0 + 25.0, 24.0, 4.0);
    }
}