use std::collections::HashMap;

use super::coin::CoinPickedupEvent;
use super::game::GameRules;
use super::{coin::Coin, input_ext::PlayerOperate};
use super::{coin::CoinInfo, input_ext::PlayerInputSettings};
use bevy::{prelude::*, sprite::collide_aabb::collide};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerInputSettings::from_array(&vec![
            (KeyCode::W, PlayerOperate::MoveFrond),
            (KeyCode::S, PlayerOperate::MoveBack),
            (KeyCode::A, PlayerOperate::MoveLeft),
            (KeyCode::D, PlayerOperate::MoveRight),
        ]));
        app.add_event::<IncreasePlayerScoreEvent>()
            .add_event::<TeamScoreChangedEvent>()
            .add_startup_system(setup.system())
            .add_system(player_score_update_system.system())
            .add_system(
                player_movement_system
                    .system()
                    .label(PlayerSystem::PlayerMoving),
            )
            .add_system(
                player_input_system
                    .system()
                    .before(PlayerSystem::PlayerMoving),
            )
            .add_system(
                player_collision_system
                    .system()
                    .after(PlayerSystem::PlayerMoving),
            );
    }
}

pub struct IncreasePlayerScoreEvent {
    pub player: Entity,
    pub score_to_increase: usize,
}

pub struct TeamScoreChangedEvent {
    pub team_score: usize,
    pub team_id: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
enum PlayerSystem {
    PlayerMoving,
}

pub struct Player;

struct PlayerInfo {
    name: String, // 玩家名称
}

struct Team {
    id: usize,
}

struct Score {
    val: usize,
}

struct Movement {
    speed: f32,
    pendding_offset: Vec3,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rules: Res<GameRules>,
) {
    let player_color = Color::rgb(0.5, 0.5, 1.0);
    commands
        .spawn()
        .insert_bundle((Player,))
        .insert_bundle((
            PlayerInfo {
                name: "Player 0".to_string(),
                // color: player_color,
            },
            Team { id: 1 },
            Score { val: 0 },
            Movement {
                speed: 500.0,
                pendding_offset: Default::default(),
            },
        ))
        .insert_bundle(SpriteBundle {
            material: materials.add(player_color.into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(rules.player_brick_size),
            ..Default::default()
        });
}

fn player_input_system(
    input: Res<Input<PlayerOperate>>,
    time: Res<Time>,
    mut query: Query<&mut Movement, With<Player>>,
) {
    if let Ok(mut movement) = query.single_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if input.pressed(PlayerOperate::MoveFrond) {
            direction.y += 1.0;
        }
        if input.pressed(PlayerOperate::MoveBack) {
            direction.y += -1.0;
        }
        if input.pressed(PlayerOperate::MoveLeft) {
            direction.x += -1.0;
        }
        if input.pressed(PlayerOperate::MoveRight) {
            direction.x += 1.0;
        }
        let speed = movement.speed;

        movement.pendding_offset +=
            Vec3::new(direction.x, direction.y, 0.0) * speed * time.delta_seconds();
    }
}

fn player_movement_system(mut query: Query<(&mut Movement, &mut Transform), With<Player>>) {
    if let Ok((mut movement, mut transform)) = query.single_mut() {
        transform.translation += movement.pendding_offset;
        movement.pendding_offset = Vec3::new(0.0, 0.0, 0.0);
    }
}

fn player_collision_system(
    player_query: Query<(Entity, &PlayerInfo, &Transform, &Team, &Sprite), With<Player>>,
    collision_query: Query<(Entity, &Transform, &Sprite, &CoinInfo), With<Coin>>,
    mut coin_pickedup_event: EventWriter<CoinPickedupEvent>,
    mut increase_score_event: EventWriter<IncreasePlayerScoreEvent>,
) {
    collision_query.for_each(
        |(coin_entity, pickup_transform, pickup_sprite, coin_info)| {
            for (player_entity, player, player_transform, _, player_sprite) in player_query.iter() {
                let collision = collide(
                    player_transform.translation,
                    player_sprite.size,
                    pickup_transform.translation,
                    pickup_sprite.size,
                );

                if let Some(_) = collision {
                    debug!("{} collect the coin({:?})!", player.name, coin_entity);

                    increase_score_event.send(IncreasePlayerScoreEvent {
                        player: player_entity,
                        score_to_increase: coin_info.score_value,
                    });

                    coin_pickedup_event.send(CoinPickedupEvent { coin: coin_entity });
                    break;
                }
            }
        },
    );
}

fn player_score_update_system(
    mut query: Query<(Entity, &mut Score, &PlayerInfo, &Team), With<Player>>,
    mut events: EventReader<IncreasePlayerScoreEvent>,
    mut team_score_changed_event: EventWriter<TeamScoreChangedEvent>,
) {
    let mut team_score_map = HashMap::new();

    events.iter().for_each(
        |IncreasePlayerScoreEvent {
             player,
             score_to_increase,
         }| {
            match query.get_mut(*player) {
                Ok((_, mut score, _, team)) => {
                    score.val += score_to_increase;
                    team_score_map.insert(team.id, 0);
                }
                Err(e) => error!("{}", e),
            }
        },
    );

    query.for_each_mut(|(_, score, player_info, team)| {
        if let Some(team_score) = team_score_map.get_mut(&team.id) {
            *team_score += score.val;
            info!("{} get coin, {} score now!", player_info.name, score.val);
        }
    });

    team_score_map.iter().for_each(|(team_id, team_score)| {
        team_score_changed_event.send(TeamScoreChangedEvent {
            team_id: *team_id,
            team_score: *team_score,
        });
    });
}
