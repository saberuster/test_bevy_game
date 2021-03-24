use bevy::core::FixedTimestep;
use bevy::prelude::*;
pub struct ElapsedTimePlugin;

#[derive(Default)]
struct ElapsedSeconds(usize);

pub struct ElapsedSecondChangedEvent {
    pub seconds: usize,
}

impl Plugin for ElapsedTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ElapsedSeconds>()
            .add_event::<ElapsedSecondChangedEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(elapsed_time_update_system.system()),
            );
    }
}

fn elapsed_time_update_system(
    mut elapsed_time: ResMut<ElapsedSeconds>,
    mut changed_event: EventWriter<ElapsedSecondChangedEvent>,
) {
    elapsed_time.0 += 1;
    //debug!("elapsed_time update, {}s now!", elapsed_time.0);
    changed_event.send(ElapsedSecondChangedEvent {
        seconds: elapsed_time.0,
    });
}
