// mod math {
//     mod math_js {
//         #[link(wasm_import_module = "Math")]
//         extern "C" {
//             pub fn random() -> f64;
//         }
//     }

//     pub fn random() -> f64 {
//         unsafe { math_js::random() }
//     }
// }

// #[no_mangle]
// pub extern "C" fn add(left: usize, right: usize) -> usize {
//     left + right + math::random()
// }

mod balls;
mod walls;

use wasm_bindgen::prelude::*;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use balls::BallsPlugin;
use walls::WallsPlugin;

// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub const PIXELS_PER_METER: f32 = 200.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.13, 0.13, 0.23);

#[wasm_bindgen]
pub fn main(window_width: f32, window_height: f32) {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ten-k-balls".to_string(),
                    resolution: WindowResolution::new(window_width, window_height),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
            // RapierDebugRenderPlugin::default(),
            BallsPlugin,
            WallsPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
