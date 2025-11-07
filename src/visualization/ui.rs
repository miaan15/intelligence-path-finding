//! User interface management for visualization

use crate::data_structures::Position;
use macroquad::prelude::*;

/// UI state management
#[derive(Debug, Clone)]
pub struct UIState {
    pub mouse_position: Vec2,
    pub grid_position: Option<Position>,
    pub is_mouse_pressed: bool,
    pub selected_tool: Tool,
    pub show_info: bool,
    pub animation_speed: f32,
    pub is_running: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tool {
    SetStart,
    SetEnd,
    ToggleWall,
    Erase,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            mouse_position: Vec2::new(0.0, 0.0),
            grid_position: None,
            is_mouse_pressed: false,
            selected_tool: Tool::SetStart,
            show_info: true,
            animation_speed: 1.0,
            is_running: false,
        }
    }
}

/// UI Manager for handling user input and interface
pub struct UIManager {
    state: UIState,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
        }
    }

    /// Get current UI state
    pub fn state(&self) -> &UIState {
        &self.state
    }

    /// Get mutable UI state
    pub fn state_mut(&mut self) -> &mut UIState {
        &mut self.state
    }

    /// Handle user input
    pub fn handle_input(&mut self) {
        self.update_mouse_state();
        self.handle_keyboard_input();
        self.handle_mouse_input();
    }

    /// Update mouse position and grid position
    fn update_mouse_state(&mut self) {
        self.state.mouse_position = mouse_position();

        // Convert mouse position to grid coordinates (this would need the grid renderer)
        // For now, we'll update this in the main application
    }

    /// Handle keyboard input
    fn handle_keyboard_input(&mut self) {
        // Toggle tools with number keys
        if is_key_pressed(KeyCode::Key1) {
            self.state.selected_tool = Tool::SetStart;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.state.selected_tool = Tool::SetEnd;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.state.selected_tool = Tool::ToggleWall;
        }
        if is_key_pressed(KeyCode::Key4) {
            self.state.selected_tool = Tool::Erase;
        }

        // Toggle info display
        if is_key_pressed(KeyCode::I) {
            self.state.show_info = !self.state.show_info;
        }

        // Adjust animation speed
        if is_key_down(KeyCode::Up) {
            self.state.animation_speed = (self.state.animation_speed + 0.1).min(5.0);
        }
        if is_key_down(KeyCode::Down) {
            self.state.animation_speed = (self.state.animation_speed - 0.1).max(0.1);
        }
    }

    /// Handle mouse input
    fn handle_mouse_input(&mut self) {
        self.state.is_mouse_pressed = is_mouse_down(MouseButton::Left);

        // Handle mouse clicks based on current tool
        if is_mouse_button_pressed(MouseButton::Left) {
            // This will be handled by the main application
            // since it needs access to the grid
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.state.selected_tool = Tool::SetEnd;
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            self.state.selected_tool = Tool::ToggleWall;
        }
    }

    /// Update grid position based on mouse and grid renderer
    pub fn update_grid_position(&mut self, grid_pos: Option<Position>) {
        self.state.grid_position = grid_pos;
    }

    /// Render UI elements
    pub fn render(&self) {
        if !self.state.show_info {
            return;
        }

        self.render_tool_selector();
        self.render_status_bar();
    }

    /// Render tool selector
    fn render_tool_selector(&self) {
        let x = 10.0;
        let y = screen_height() - 100.0;
        let button_width = 80.0;
        let button_height = 25.0;
        let spacing = 5.0;

        let tools = [
            (Tool::SetStart, "1.Start", GREEN),
            (Tool::SetEnd, "2.End", RED),
            (Tool::ToggleWall, "3.Wall", DARKGRAY),
            (Tool::Erase, "4.Erase", LIGHTGRAY),
        ];

        for (i, (tool, label, color)) in tools.iter().enumerate() {
            let button_x = x + i as f32 * (button_width + spacing);
            let is_selected = self.state.selected_tool == *tool;

            let bg_color = if is_selected { color } else { GRAY };
            let text_color = if is_selected { WHITE } else { BLACK };

            draw_rectangle(button_x, y, button_width, button_height, bg_color);
            draw_rectangle_lines(button_x, y, button_width, button_height, 2.0, BLACK);

            let text_x = button_x + 5.0;
            let text_y = y + button_height - 5.0;
            draw_text(label, text_x, text_y, 12.0, text_color);
        }
    }

    /// Render status bar
    fn render_status_bar(&self) {
        let y = screen_height() - 30.0;
        let status_text = if self.state.is_running {
            "Running..."
        } else {
            "Ready"
        };

        draw_text(
            &format!("Status: {} | Speed: {:.1}x", status_text, self.state.animation_speed),
            10.0,
            y,
            14.0,
            BLACK,
        );

        // Show current tool
        let tool_text = match self.state.selected_tool {
            Tool::SetStart => "Tool: Set Start Position",
            Tool::SetEnd => "Tool: Set End Position",
            Tool::ToggleWall => "Tool: Toggle Walls",
            Tool::Erase => "Tool: Erase",
        };

        draw_text(tool_text, 200.0, y, 14.0, DARKGRAY);

        // Show mouse grid position
        if let Some(grid_pos) = &self.state.grid_position {
            draw_text(
                &format!("Grid: ({}, {})", grid_pos.x, grid_pos.y),
                500.0,
                y,
                14.0,
                DARKGRAY,
            );
        }
    }

    /// Check if a specific key was pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        is_key_pressed(key)
    }

    /// Check if a specific key is currently down
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        is_key_down(key)
    }

    /// Check if mouse button was pressed
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        is_mouse_button_pressed(button)
    }

    /// Check if mouse button is currently down
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        is_mouse_down(button)
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}