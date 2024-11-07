pub mod bundles;
pub mod components;
mod despawn;
pub mod observers;
pub mod render;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::components::*;
    pub use crate::observers::component_observer;
    pub use crate::StatBarsPlugin;
}

use bevy::prelude::*;

pub struct StatBarsPlugin;

impl Plugin for StatBarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            render::RenderStatBarsPlugin,
            observers::StatBarObserverPlugin,
        ));
        app.add_systems(PostUpdate, despawn::despawn_if_subject_not_found);
    }
}
