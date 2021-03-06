//! A module containing the definitions for the various UI elements.

#[cfg(not(target_arch = "wasm32"))]
use nfd;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};
#[cfg(target_arch = "wasm32")]
use stdweb::*;

use super::{Button, Label, Map, Slider, StatusLabel, UiElement, WorldPoints};
use crate::graphics::{position, Font};

/// A slider acting as a low-pass filter.
pub fn lowpass_filter(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 40,
            margin_horizontal: 40,
        },
        20,
        1.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.lowpass_filter = value;
        }),
        "Low-pass filter".to_owned(),
        font,
    ))
}

/// A slider acting as a high-pass filter.
pub fn highpass_filter(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 120,
            margin_horizontal: 40,
        },
        20,
        0.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.highpass_filter = value;
        }),
        "High-pass filter".to_owned(),
        font,
    ))
}

/// A slider controlling the particle speeds.
pub fn speed_multiplier(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 40,
            margin_horizontal: 285,
        },
        10,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.speed_multiplier = value;
        }),
        "Speed".to_owned(),
        font,
    ))
}

/// A slider controlling the size of the seeding area.
pub fn seeding_size(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 120,
            margin_horizontal: 285,
        },
        80,
        1.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.seeding_size = value;
        }),
        "Seeding size".to_owned(),
        font,
    ))
}

/// A slider controlling the lifetime of particles on the CPU.
pub fn cpu_lifetime(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 200,
            margin_horizontal: 40,
        },
        80,
        0.2,
        screensize,
        Box::new(|ref mut context, value| {
            context.lifetime = value * 500.0;
        }),
        "Lifetime".to_owned(),
        font,
    ))
}

/// A slider controlling the transparency of the marching cubes mesh.
pub fn mesh_transparency(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 200,
            margin_horizontal: 285,
        },
        50,
        0.02,
        screensize,
        Box::new(|ref mut context, value| {
            context.mesh_transparency = value;
        }),
        "Mesh transparency".to_owned(),
        font,
    ))
}

/// A slider controlling the size of the rendered particles on the CPU.
pub fn cpu_particle_size(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 280,
            margin_horizontal: 40,
        },
        20,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.particle_size = value * 16.0;
        }),
        "Particle size".to_owned(),
        font,
    ))
}

/// A slider controlling the spawn speed of particles on the CPU.
pub fn cpu_particle_spawn_rate(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 280,
            margin_horizontal: 285,
        },
        50,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.particle_respawn_per_tick = (value * 2000.0) as u32;
        }),
        "Particle spawn rate".to_owned(),
        font,
    ))
}

/// A slider controlling the particle transparency on the GPU.
pub fn gpu_transparency(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 200,
            margin_horizontal: 40,
        },
        100,
        0.2,
        screensize,
        Box::new(|ref mut context, value| {
            context.particle_transparency = value;
        }),
        "Particle transparency".to_owned(),
        font,
    ))
}

/// A button letting the user load a new file.
pub fn load_file(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 200,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(None, None) {
                    context.file_path = Some(PathBuf::from(path));
                    context.reload_file = true;
                }
            }
            #[cfg(target_arch = "wasm32")]
            js!(openFileDialog());
        }),
        "       Load file".to_owned(),
        font,
    ))
}

/// A button toggling the UI visibility.
pub fn toggle_ui(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Button {
    Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 40,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut _context, _toggle_state| {}),
        "     Toggle UI".to_owned(),
        font,
    )
}

/// A label with additional methods for displaying statuses.
pub fn status_label(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> StatusLabel {
    StatusLabel::new(
        position::Absolute {
            height: 10,
            width: 10,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 30,
            margin_horizontal: 10,
        },
        screensize,
        font,
    )
}

/// The credits!
pub fn credits_label(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<dyn UiElement> {
    Box::new(Label::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::TopLeft,
            margin_vertical: 0,
            margin_horizontal: 20,
        },
        screensize,
        "By Robin Grundvåg, Vegard Itland and Stian Soltvedt".to_owned(),
        font,
    ))
}

/// CPU/GPU particles toggle.
pub fn cpu_gpu_particles_toggle(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<dyn UiElement> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 120,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut context, toggle_state| context.use_cpu_particles = toggle_state),
        "     Use CPU".to_owned(),
        font,
    ))
}

/// A button toggling world point visibility.
pub fn toggle_world_points(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Button {
    Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 280,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut _context, _toggle_state| {}),
        " Toggle points".to_owned(),
        font,
    )
}

pub fn map(screensize: (f32, f32)) -> Map {
    Map::new(
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 5,
        },
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 205,
        },
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 405,
        },
        screensize,
    )
}

pub fn world_points(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> WorldPoints {
    WorldPoints::new(screensize, font)
}
