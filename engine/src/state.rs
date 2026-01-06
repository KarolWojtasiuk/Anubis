use crate::prelude::*;

#[derive(Default)]
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        app.add_systems(
            Update,
            handle_main_menu_state.in_set(MainMenuUpdateSystems::StateTransition),
        );
        app.add_systems(
            Update,
            handle_gameplay_state.in_set(GameplayUpdateSystems::StateTransition),
        );
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    #[default]
    Gameplay,
}

fn handle_main_menu_state(
    main_menu_input: Res<MainMenuInput>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_writer: MessageWriter<AppExit>,
) {
    if main_menu_input.exit {
        info!("Quitting game");
        exit_writer.write(AppExit::Success);
    } else if main_menu_input.start {
        info!("Switching game state to Gameplay");
        next_state.set(GameState::Gameplay);
    }
}

fn handle_gameplay_state(
    gameplay_input: Res<GameplayInput>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if gameplay_input.exit {
        info!("Switching game state to MainMenu");
        next_state.set(GameState::MainMenu);
    }
}
