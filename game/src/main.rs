use engine::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugins)
        .add_systems(Startup, (spawn_info_text, setup_scene))
        .run()
}

fn spawn_info_text(mut commands: Commands) {
    commands.spawn((
        Text::new(format!(
            "{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        Node {
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            position_type: PositionType::Absolute,
            ..default()
        },
    ));
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
