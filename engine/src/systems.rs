use crate::prelude::*;

#[derive(Default)]
pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (
                MainMenuPreUpdateSystems::InputProcessing.after(bevy::input::InputSystems),
                MainMenuPreUpdateSystems::General,
            )
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        );
        app.configure_sets(
            Update,
            (
                MainMenuUpdateSystems::StateTransition,
                MainMenuUpdateSystems::General,
            )
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        );
        app.configure_sets(
            PostUpdate,
            MainMenuPostUpdateSystems::General.run_if(in_state(GameState::MainMenu)),
        );

        app.configure_sets(
            PreUpdate,
            (
                GameplayPreUpdateSystems::InputProcessing.after(bevy::input::InputSystems),
                GameplayPreUpdateSystems::General,
            )
                .chain()
                .run_if(in_state(GameState::Gameplay)),
        );
        app.configure_sets(
            Update,
            (
                GameplayUpdateSystems::StateTransition,
                GameplayUpdateSystems::General,
            )
                .chain()
                .run_if(in_state(GameState::Gameplay)),
        );
        app.configure_sets(
            PostUpdate,
            (
                GameplayPostUpdateSystems::CameraUpdate
                    .before(bevy::transform::TransformSystems::Propagate),
                GameplayPostUpdateSystems::General,
            )
                .chain()
                .run_if(in_state(GameState::Gameplay)),
        );
        app.configure_sets(
            FixedUpdate,
            GameplayFixedUpdateSystems::General.run_if(in_state(GameState::Gameplay)),
        );
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MainMenuPreUpdateSystems {
    InputProcessing,
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MainMenuUpdateSystems {
    StateTransition,
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MainMenuPostUpdateSystems {
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameplayPreUpdateSystems {
    InputProcessing,
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameplayUpdateSystems {
    StateTransition,
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameplayPostUpdateSystems {
    CameraUpdate,
    General,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameplayFixedUpdateSystems {
    General,
}
