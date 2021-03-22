use bevy::core::FixedTimestep;
use bevy::prelude::*;
pub struct ElapsedTimePlugin;

struct ElapsedSeconds(usize);

pub struct OnElapsedSecondChangedEvent {
    pub seconds: usize,
}

impl Plugin for ElapsedTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ElapsedSeconds(0))
            .add_event::<OnElapsedSecondChangedEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(elapsed_time_update_system.system()),
            );
    }
}

fn elapsed_time_update_system(
    mut elapsed_time: ResMut<ElapsedSeconds>,
    mut changed_event: EventWriter<OnElapsedSecondChangedEvent>,
) {
    elapsed_time.0 += 1;
    debug!("elapsed_time update, {}s now!", elapsed_time.0);
    changed_event.send(OnElapsedSecondChangedEvent {
        seconds: elapsed_time.0,
    });
}
