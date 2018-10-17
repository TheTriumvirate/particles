use graphics::{Drawable, Rectangle, position};
use gui::UiElement;
use State;
use na::Matrix4;

/// A simple button that can be pressed.
pub struct Button {
    pos_abs: position::Absolute,
    pos_rel: position::Relative,
    rect: Rectangle,
    button_toggles: bool,
    toggle_state: bool,
    color: (f32, f32, f32),
    color_toggled: (f32, f32, f32),
    func: Box<dyn FnMut(&mut State, bool)>,
}

impl Button {
    /// Creates a new button.
    /// If `toggles` is `true`, the button will change color when toggled, and the `func` will be
    /// called with a boolean argument that is true when it's toggled on, and false when toggled
    /// off. Otherwise it will always be false.
    pub fn new(
        pos_abs: position::Absolute,
        color: (f32, f32, f32),
        screensize: (f32, f32),
        button_toggles: bool,
        func: Box<dyn FnMut(&mut State, bool)>,
    ) -> Self {
        let pos_rel = pos_abs.to_relative(screensize);
        let mut color_toggled = color.clone();
        color_toggled.0 += 0.1;
        color_toggled.1 += 0.1;
        color_toggled.2 += 0.1;
        Self {
            pos_abs,
            pos_rel,
            func,
            button_toggles,
            toggle_state: false,
            color,
            color_toggled,
            rect: Rectangle::new(pos_rel.get_coordinates(), color),
        }
    }

    /// Returns the toggle state of the button.
    /// If the button was not initialized as a toggle button, always returns false.
    pub fn toggle_state(&self) -> bool {
        self.toggle_state
    }
}

impl UiElement for Button {
    fn is_within(&self, x: f64, y: f64) -> bool {
        let c = self.pos_rel.get_coordinates();
        x > c.x1.into() && x < c.x2.into() && y > c.y1.into() && y < c.y2.into()
    }

    fn click(&mut self, _x: f64, _y: f64, state: &mut State) {
        if self.button_toggles {
            self.toggle_state = !self.toggle_state;
            let color = if self.toggle_state {
                self.color
            } else {
                self.color_toggled
            };
            self.rect = Rectangle::new(self.pos_rel.get_coordinates(), color);
        }

        let func = &mut self.func;
        func(state, self.toggle_state);
    }

    fn resize(&mut self, screensize: (f32, f32)) {
        self.pos_rel = self.pos_abs.to_relative(screensize);
        let color = if self.toggle_state {
            self.color
        } else {
            self.color_toggled
        };
        self.rect = Rectangle::new(self.pos_rel.get_coordinates(), color);
    }
}

impl Drawable for Button {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.rect.draw_transformed(view_matrix);
    }
}
