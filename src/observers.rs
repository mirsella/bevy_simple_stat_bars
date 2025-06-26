use bevy::ecs::world::EntityRef;
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::despawn;
use crate::prelude::StatBarSubject;
use crate::prelude::StatBarValue;

#[derive(Component)]
pub struct StatBarObserver {
    inner: Box<dyn ComponentObserver + 'static + Send + Sync>,
}

pub fn component_observer<C, F>(f: F) -> StatBarObserver
where
    C: Component,
    F: Fn(&C) -> f32 + 'static + Send + Sync,
{
    StatBarObserver {
        inner: Box::new(ObserveComponent {
            function: f,
            phantom: PhantomData,
        }),
    }
}

#[derive(Default)]
struct ObserveComponent<C, F>
where
    F: Fn(&C) -> f32,
{
    pub phantom: PhantomData<C>,
    pub function: F,
}

impl<C, F> ComponentObserver for ObserveComponent<C, F>
where
    C: Component,
    F: Fn(&C) -> f32,
{
    fn observe(&self, entity_ref: EntityRef<'_>) -> Option<f32> {
        entity_ref
            .get::<C>()
            .map(|component| (self.function)(component))
    }
}

trait ComponentObserver {
    fn observe(&self, entity_ref: EntityRef<'_>) -> Option<f32>;
}

#[derive(Default, Resource, Deref, DerefMut)]
struct ValuesBuffer(Vec<(Entity, f32)>);

fn observe_components(world: &mut World) {
    world.resource_scope(|world: &mut World, mut b: Mut<ValuesBuffer>| {
        for (bar, subject, observer) in world
            .query::<(Entity, &StatBarSubject, &StatBarObserver)>()
            .iter(world)
        {
            if let Ok(entity_ref) = world.get_entity(**subject) {
                if let Some(value) = observer.inner.observe(entity_ref) {
                    (*b).push((bar, value));
                }
            }
        }
        for (entity, value) in (*b).drain(..) {
            if let Some(mut sb_value) = world.entity_mut(entity).get_mut::<StatBarValue>() {
                **sb_value = value;
            }
        }
    });
}

pub struct StatBarObserverPlugin;

impl Plugin for StatBarObserverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ValuesBuffer>().add_systems(
            PostUpdate,
            observe_components.after(despawn::despawn_if_subject_not_found),
        );
    }
}
