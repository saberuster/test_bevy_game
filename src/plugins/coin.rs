use super::game::*;
use bevy::prelude::*;
use rand::Rng;
pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CoinPickedupEvent>()
            .add_event::<NewCoinSpawnedEvent>()
            .add_system_set_to_stage(
                GameStage::EventHandle,
                SystemSet::new()
                    .with_system(pickedup_event_listener_system.system())
                    .with_system(spawn_new_event_listener_system.system()),
            )
            .add_system_set(
                SystemSet::on_enter(MatchState::Playing).with_system(beginplay_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(MatchState::Playing).with_system(update_system.system()),
            )
            .add_system_set(
                SystemSet::on_enter(MatchState::GameOver).with_system(gameover_system.system()),
            );
    }
}

pub struct CoinPickedupEvent {
    pub coin: Entity,
}

pub struct NewCoinSpawnedEvent {}

pub struct Coin;

pub struct CoinInfo {
    pub score_value: usize,
}

fn beginplay_system(rules: Res<GameRules>, mut event: EventWriter<NewCoinSpawnedEvent>) {
    debug!("init coins!");
    (0..rules.max_coin_num).for_each(|_| {
        event.send(NewCoinSpawnedEvent {});
    });
}

fn spawn_new_event_listener_system(
    rules: Res<GameRules>,
    mut commands: Commands,
    mut events: EventReader<NewCoinSpawnedEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    events.iter().for_each(|_| {
        commands
            .spawn()
            .insert_bundle((Coin,))
            .insert_bundle((CoinInfo {
                score_value: rand::thread_rng()
                    .gen_range(rules.min_coin_score_value..rules.max_coin_score_value),
            },))
            .insert_bundle(SpriteBundle {
                material: materials.add(Color::GOLD.into()),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(-300.0..300.0),
                    rand::thread_rng().gen_range(-300.0..300.0),
                    0.0,
                ),
                sprite: Sprite::new(Vec2::new(50.0, 50.0)),
                ..Default::default()
            });
    });
}

fn update_system() {}

fn gameover_system(mut commands: Commands, query: Query<Entity, With<Coin>>) {
    debug!("game over!");
    query.for_each(|e| commands.entity(e).despawn());
}

fn pickedup_event_listener_system(
    mut commands: Commands,
    mut events: EventReader<CoinPickedupEvent>,
    mut spawn_coin_event: EventWriter<NewCoinSpawnedEvent>,
) {
    events.iter().for_each(|CoinPickedupEvent { coin }| {
        commands.entity(*coin).despawn();
        debug!("coin: {:?} despawn!", coin);
        spawn_coin_event.send(NewCoinSpawnedEvent {});
    });
}
