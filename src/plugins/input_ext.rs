use bevy::input::InputSystem;
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct InputExtSystem;

pub struct InputExtPlugin;

impl Plugin for InputExtPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Input<PlayerOperate>>()
            .init_resource::<PlayerInputSettings>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                input_ext_update_system
                    .system()
                    .label(InputExtSystem)
                    .after(InputSystem),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PlayerOperate {
    MoveFrond,
    MoveBack,
    MoveRight,
    MoveLeft,
}

#[derive(Default)]
pub struct PlayerInputSettings {
    input_map: HashMap<KeyCode, PlayerOperate>,
}

impl PlayerInputSettings {
    pub fn from_array(arr: &Vec<(KeyCode, PlayerOperate)>) -> Self {
        Self {
            input_map: arr.iter().cloned().collect(),
        }
    }
}

fn input_ext_update_system(
    settings: Res<PlayerInputSettings>,
    input_key: Res<Input<KeyCode>>,
    mut player_op: ResMut<Input<PlayerOperate>>,
) {
    input_key
        .get_just_pressed()
        .filter_map(|k| settings.input_map.get(k))
        .for_each(|op| player_op.press(*op));

    input_key
        .get_just_released()
        .filter_map(|k| settings.input_map.get(k))
        .for_each(|op| player_op.release(*op));
}
