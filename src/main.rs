use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            ..Default::default()
        })
        .add_resource(WindowDescriptor {
            title: "Game!".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_resources.system())
        .add_startup_stage("player_setup")
        .add_startup_system_to_stage("player_setup", setup_player.system())
        .add_system(movement.system())
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
pub struct Player;
pub struct PlayerSkin(Handle<ColorMaterial>);
pub struct PlayerMaxVelocity(f32);

pub fn setup_resources(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(PlayerSkin(materials.add(Color::rgb(0.3, 0.7, 0.7).into())));
}

pub fn setup_player(mut commands: Commands, player_skin: Res<PlayerSkin>) {
    commands
        .spawn(SpriteComponents {
            material: player_skin.0,
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic())
        .with(ColliderBuilder::cuboid(100.0, 100.0))
        .with(Player)
        .with(PlayerMaxVelocity(1000.));
}

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<&RigidBodyHandleComponent>,
    mut players_pos: Query<(&Player, Entity, &PlayerMaxVelocity)>,
) {
    for (_player, ent, max_v) in &mut players_pos.iter() {
        if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(ent) {
            let mut body = bodies.get_mut(body_handle.handle()).unwrap();
            body.wake_up(true);
            let mut direction = Vector2::zeros();
            if keyboard_input.pressed(KeyCode::Left) {
                direction = Vector2::x() * -1.;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction = Vector2::x();
            }
            if keyboard_input.pressed(KeyCode::Down) {
                direction = Vector2::y() * -1.;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                direction = Vector2::y();
            }

            body.linvel = direction * max_v.0;
            // body.apply_force(direction * max_v.0);
            // info!("{:?}", body.linvel,);
        }
    }
}
