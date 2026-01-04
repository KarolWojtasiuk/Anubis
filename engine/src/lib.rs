pub mod prelude {
    pub use super::EnginePlugins;
    // pub use bevy::ecs as bevy_ecs;
    pub use bevy::prelude::*;
    // pub use bevy::reflect as bevy_reflect;
}

bevy::app::plugin_group! {
    pub struct EnginePlugins {

    }
}
