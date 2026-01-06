pub mod camera;
pub mod character;
pub mod input;
pub mod prelude;
pub mod state;
pub mod systems;

bevy::app::plugin_group! {
    pub struct EnginePlugins {
        state:::GameStatePlugin,
        systems:::GameSystemsPlugin,
        input:::GameInputPlugin,
        camera:::GameCameraPlugin,
        character:::CharacterPlugin
    }
}
