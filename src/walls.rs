use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

const WALL_THICKNESS: f32 = 10.0;
const COLOR_WALL: Color = Color::rgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    friction: Friction,
    restitution: Restitution,
}

impl WallBundle {
    fn new(translation: Vec3, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WALL,
                    ..Default::default()
                },
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
            friction: Friction::coefficient(0.0),
            restitution: Restitution::coefficient(1.0),
        }
    }
}

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands,  window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let wall_length_x = window.width() * 0.8;
    let wall_length_y = window.height() * 0.8;

    commands.spawn(WallBundle::new(Vec3
        ::new(0.0, wall_length_y / 2.0, 0.0), Vec3::new(wall_length_x, WALL_THICKNESS, 1.0)));
    commands.spawn(WallBundle::new(Vec3
        ::new(wall_length_x / 2.0, 0.0, 0.0), Vec3::new(WALL_THICKNESS, wall_length_y + 10.0, 1.0)));
    commands.spawn(WallBundle::new(Vec3
        ::new(0.0, -wall_length_y / 2.0, 0.0), Vec3::new(wall_length_x, WALL_THICKNESS, 1.0)));
    commands.spawn(WallBundle::new(Vec3
        ::new(-wall_length_x / 2.0, 0.0, 0.0), Vec3::new(WALL_THICKNESS, wall_length_y + 10.0, 1.0)));
    
    // let shape_top_and_bottom_wall = shapes::Rectangle {
    //     extents: Vec2::new(
    //         crate::PIXELS_PER_METER * 0.73,
    //         crate::PIXELS_PER_METER * 0.03,
    //     ),
    //     origin: shapes::RectangleOrigin::Center,
    // };
    // let bottom_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * -0.64);

        // .insert(Transform::from_xyz(
        //     bottom_wall_pos.x,
        //     bottom_wall_pos.y,
        //     0.0,
        // ));
    // let top_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * 0.64);
    // let shape_left_and_right_wall = shapes::Rectangle {
    //     extents: Vec2::new(
    //         crate::PIXELS_PER_METER * 0.03,
    //         crate::PIXELS_PER_METER * 1.3,
    //     ),
    //     origin: shapes::RectangleOrigin::Center,
    // };
    // let left_wall_pos = Vec2::new(crate::PIXELS_PER_METER * -0.35, 0.0);
    // let right_wall_pos = Vec2::new(crate::PIXELS_PER_METER * 0.35, 0.0);
}