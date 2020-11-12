use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
struct Player;
struct PlayerSkin(Handle<ColorMaterial>);

fn setup_resources(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(PlayerSkin(materials.add(Color::rgb(0.3, 0.7, 0.7).into())));
}

fn setup_player(mut commands: Commands, player_skin: Res<PlayerSkin>) {
    commands
        .spawn(SpriteComponents {
            material: player_skin.0,
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .with(Player);
}

#[derive(Default)]
struct CursorEventState(EventReader<CursorMoved>);

fn movement(
    mut state: Local<CursorEventState>,
    cursor_events: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    mut players_pos: Query<(&Player, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(moved) = state.0.latest(&cursor_events) {
        for (_player, mut transform) in &mut players_pos.iter() {
            let pos =
                moved.position - Vec2::new(window.width as f32 / 2.0, window.height as f32 / 2.0);
            transform.set_translation(pos.extend(0.0));
        }
    }
}
