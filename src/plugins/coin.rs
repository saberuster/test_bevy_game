use super::{
    game::{GameRules, MatchState},
    player::IncreasePlayerScoreEvent,
};
use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OnCoinPickedupEvent>()
            .add_event::<NewCoinSpawnedEvent>()
            .add_system(pickedup_event_listener_system.system())
            .add_system(spawn_new_event_listener_system.system())
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

pub struct OnCoinPickedupEvent {
    pub coin: Entity,
    pub player: Entity,
}

pub struct NewCoinSpawnedEvent {}

pub struct Coin;

pub struct CoinInfo {
    pub score_value: usize,
}

fn beginplay_system(rules: Res<GameRules>, mut event: EventWriter<NewCoinSpawnedEvent>) {
    (0..rules.max_coin_num).for_each(|_| {
        event.send(NewCoinSpawnedEvent {});
    });
}

fn spawn_new_event_listener_system(
    mut commands: Commands,
    mut events: EventReader<NewCoinSpawnedEvent>,
    rules: Res<GameRules>,
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
    })
}

fn update_system() {}

fn gameover_system(mut commands: Commands, query: Query<Entity, With<Coin>>) {
    query.for_each(|e| {
        commands.entity(e).despawn();
    });
}

fn pickedup_event_listener_system(
    mut commands: Commands,
    query: Query<(Entity, &CoinInfo), With<Coin>>,
    mut events: EventReader<OnCoinPickedupEvent>,
    mut score_events: EventWriter<IncreasePlayerScoreEvent>,
    mut spawn_coin_event: EventWriter<NewCoinSpawnedEvent>,
) {
    // 因为每个金币只能被捡一次，所以这里直接用了 hashmap
    let waiting_despawns: HashMap<_, _> = events.iter().map(|event| (event.coin, event)).collect();
    if waiting_despawns.len() > 0 {
        query.for_each(|(coin, coin_info)| {
            if let Some(event) = waiting_despawns.get(&coin) {
                score_events.send(IncreasePlayerScoreEvent {
                    player: event.player,
                    score_to_increase: coin_info.score_value,
                });
                commands.entity(coin).despawn();
                spawn_coin_event.send(NewCoinSpawnedEvent {});
            }
        });
    }
}
