use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

const COLOR_BALL: Color = Color::rgb(0.60, 0.55, 0.60);

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_balls);
    }
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let ball_pos = Vec2::new(
    //     crate::PIXELS_PER_METER * 0.3,
    //     crate::PIXELS_PER_METER * -0.2,
    // );

    for i in -50..=50 {
        for j in -50..=50 {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::default().into()).into(),
                    material: materials.add(ColorMaterial::from(COLOR_BALL)),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * 3.0, j as f32 * 3.0, 0.0),
                        scale: Vec3::new(2.0, 2.0, 1.0),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Collider::ball(0.5))
                // .insert(Collider::ball(crate::PIXELS_PER_METER * 0.03))
                // .insert(Transform::from_xyz(ball_pos.x, ball_pos.y, 0.0))
                .insert(Velocity {
                    linvel: Vec2::new(i as f32 * 8.0, j as f32 * 8.0),
                    angvel: 0.4
                })
                .insert(GravityScale(0.0))
                // .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Friction::coefficient(0.0001))
                .insert(Restitution::coefficient(1.0));
                // .insert(Ball);
        }
    }
}
