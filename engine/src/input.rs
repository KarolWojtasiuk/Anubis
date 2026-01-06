use crate::prelude::*;
use bevy::input::mouse::AccumulatedMouseScroll;

#[derive(Default)]
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuInput>();
        app.init_resource::<GameplayInput>();

        app.add_systems(
            PreUpdate,
            process_main_menu_input.in_set(MainMenuPreUpdateSystems::InputProcessing),
        );
        app.add_systems(
            PreUpdate,
            process_gameplay_input.in_set(GameplayPreUpdateSystems::InputProcessing),
        );
    }
}

#[derive(Resource, Default)]
pub struct MainMenuInput {
    pub exit: bool,
    pub start: bool,
}

#[derive(Resource, Default)]
pub struct GameplayInput {
    pub exit: bool,
    pub sprint: bool,
    pub movement: Vec2,
    pub zoom: i8,
}

fn process_main_menu_input(mut input: ResMut<MainMenuInput>, keys: Res<ButtonInput<KeyCode>>) {
    input.exit = keys.just_pressed(KeyCode::Escape);
    input.start = keys.just_pressed(KeyCode::Enter);
}

fn process_gameplay_input(
    mut input: ResMut<GameplayInput>,
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    input.exit = keys.just_pressed(KeyCode::Escape);
    input.sprint = keys.pressed(KeyCode::ShiftLeft);
    input.movement = {
        let mut value = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            value.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            value.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            value.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            value.y -= 1.0;
        }
        value.normalize_or_zero()
    };
    input.zoom = scroll.delta.y as i8;
}
