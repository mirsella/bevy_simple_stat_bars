use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn despawn_if_subject_not_found(
    mut commands: Commands,
    subjects: Query<Entity>,
    bars: Query<(Entity, &StatBarSubject)>,
) {
    for (bar, &StatBarSubject(subject)) in bars.iter() {
        if subjects.get(subject).is_err() {
            commands.entity(bar).despawn_recursive();
        }
    }
}
