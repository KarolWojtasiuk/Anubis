use engine::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EnginePlugins);
    app.add_plugins(PhysicsPlugins::default());

    if std::env::args().any(|a| a == "--debug-physics") {
        app.add_plugins(PhysicsDebugPlugin);
    }

    app.add_systems(Startup, spawn_info_text);
    app.add_systems(OnEnter(GameState::Gameplay), setup_scene);
    app.run()
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(GameState::Gameplay),
        ))
        .with_children(|commands| {
            commands.spawn((
                DirectionalLight {
                    illuminance: light_consts::lux::OVERCAST_DAY,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(Vec3::new(1.0, 5.0, -1.0))
                    .looking_at(Vec3::ZERO, Dir3::Y),
            ));
            commands.spawn(GameCamera::default());
            commands.spawn((
                Transform::from_xyz(0.0, -5.0, 0.0),
                Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(250.0)))),
                MeshMaterial3d(materials.add(Color::srgb(0.1, 0.6, 0.1))),
                RigidBody::Static,
                Collider::half_space(Vec3::Y),
            ));
            commands.spawn((
                Player,
                GameCameraTarget {
                    offset: Vec3::new(0.0, 15.0, 7.5),
                },
                Mesh3d(meshes.add(Capsule3d::new(1.0, 2.0))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.75, 0.0))),
                Collider::capsule(1.0, 2.0),
                LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            ));
        });
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
