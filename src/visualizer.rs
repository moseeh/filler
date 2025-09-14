use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::sync::{Arc, Mutex};
use std::thread;

// Color scheme for the game
const WINDOW_SIZE: u32 = 1000;
const BORDER_COLOR: Color = Color::RGB(64, 64, 64);
const EMPTY_COLOR: Color = Color::RGB(200, 200, 200);
const PLAYER1_TERRITORY: Color = Color::RGB(255, 100, 100); // Red
const PLAYER1_RECENT: Color = Color::RGB(255, 50, 50); // Bright Red
const PLAYER2_TERRITORY: Color = Color::RGB(100, 100, 255); // Blue
const PLAYER2_RECENT: Color = Color::RGB(50, 50, 255); // Bright Blue

pub struct GameVisualizer {
    canvas: WindowCanvas,
    event_pump: EventPump,
    cell_width: f32,
    cell_height: f32,
    board_width: usize,
    board_height: usize,
}

impl GameVisualizer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Filler Game Visualizer", WINDOW_SIZE, WINDOW_SIZE)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            canvas,
            event_pump,
            cell_width: 0.0,
            cell_height: 0.0,
            board_width: 0,
            board_height: 0,
        })
    }

    pub fn update_board_dimensions(&mut self, width: usize, height: usize) {
        self.board_width = width;
        self.board_height = height;
        self.cell_width = WINDOW_SIZE as f32 / width as f32;
        self.cell_height = WINDOW_SIZE as f32 / height as f32;
    }

    pub fn render_board(&mut self, board: &Vec<Vec<char>>) -> Result<(), String> {
        // Clear canvas with background color
        self.canvas.set_draw_color(Color::RGB(240, 240, 240));
        self.canvas.clear();

        // Draw each cell
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                self.draw_cell(col_idx, row_idx, cell)?;
            }
        }

        // Present the rendered frame
        self.canvas.present();
        Ok(())
    }

    fn draw_cell(&mut self, col: usize, row: usize, cell_char: char) -> Result<(), String> {
        let x = (col as f32 * self.cell_width) as i32;
        let y = (row as f32 * self.cell_height) as i32;
        let width = self.cell_width as u32;
        let height = self.cell_height as u32;

        // Determine cell color based on character
        let color = match cell_char {
            '.' => EMPTY_COLOR,
            '@' => PLAYER1_TERRITORY,
            'a' => PLAYER1_RECENT,
            '$' => PLAYER2_TERRITORY,
            's' => PLAYER2_RECENT,
            _ => EMPTY_COLOR, // Fallback for unknown characters
        };

        // Fill the cell with the appropriate color
        self.canvas.set_draw_color(color);
        let cell_rect = Rect::new(x + 1, y + 1, width - 2, height - 2);
        self.canvas.fill_rect(cell_rect)?;

        // Draw border around the cell
        self.canvas.set_draw_color(BORDER_COLOR);
        let border_rect = Rect::new(x, y, width, height);
        self.canvas.draw_rect(border_rect)?;

        Ok(())
    }

    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,
                _ => {}
            }
        }
        true
    }
}

// Thread-safe wrapper for the visualizer
pub struct VisualizerHandle {
    board_data: Arc<Mutex<Option<(usize, usize, Vec<Vec<char>>)>>>,
}

impl VisualizerHandle {
    pub fn new() -> Self {
        Self {
            board_data: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_visualizer(&self) {
        let board_data = self.board_data.clone();

        thread::spawn(move || {
            let mut visualizer = match GameVisualizer::new() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to create visualizer: {}", e);
                    return;
                }
            };

            let mut running = true;
            while running {
                // Check for new board data
                if let Ok(mut data) = board_data.try_lock() {
                    if let Some((width, height, board)) = data.take() {
                        visualizer.update_board_dimensions(width, height);
                        if let Err(e) = visualizer.render_board(&board) {
                            eprintln!("Failed to render board: {}", e);
                        }
                    }
                }

                // Handle SDL events
                running = visualizer.handle_events();

                // Small delay to prevent busy waiting
                std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
            }
        });
    }

    pub fn update_board(&self, width: usize, height: usize, board: Vec<Vec<char>>) {
        if let Ok(mut data) = self.board_data.lock() {
            *data = Some((width, height, board));
        }
    }
}

// Global visualizer instance (optional - for easy access)
use std::sync::OnceLock;
static VISUALIZER: OnceLock<VisualizerHandle> = OnceLock::new();

pub fn get_visualizer() -> &'static VisualizerHandle {
    VISUALIZER.get_or_init(|| {
        let handle = VisualizerHandle::new();
        handle.start_visualizer();
        handle
    })
}
