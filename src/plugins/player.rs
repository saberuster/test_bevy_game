use std::collections::HashMap;

use super::coin::Coin;
use super::coin::OnCoinPickedupEvent;
use super::game::GameRules;
use bevy::{prelude::*, sprite::collide_aabb::collide};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
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
                  // color: Color, // 玩家颜色
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
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Movement, With<Player>>,
) {
    if let Ok(mut movement) = query.single_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.y += -1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.x += -1.0;
        }
        if input.pressed(KeyCode::D) {
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
    collision_query: Query<(Entity, &Transform, &Sprite), With<Coin>>,
    mut coin_pickedup_event: EventWriter<OnCoinPickedupEvent>,
) {
    collision_query.for_each(|(coin_entity, pickup_transform, pickup_sprite)| {
        for (player_entity, player, player_transform, _, player_sprite) in player_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                pickup_transform.translation,
                pickup_sprite.size,
            );

            if let Some(_) = collision {
                debug!("{} collect the coin!", player.name);
                coin_pickedup_event.send(OnCoinPickedupEvent {
                    coin: coin_entity,
                    player: player_entity,
                });
                break;
            }
        }
    });
}

fn player_score_update_system(
    query: Query<(Entity, &mut Score, &PlayerInfo, &Team), With<Player>>,
    mut events: EventReader<IncreasePlayerScoreEvent>,
    mut team_score_changed_event: EventWriter<TeamScoreChangedEvent>,
) {
    let mut score_map = HashMap::new();
    let mut team_score_map = HashMap::new();

    events.iter().for_each(|event| {
        let waiting_increase = score_map.entry(event.player).or_insert(0usize);
        *waiting_increase += event.score_to_increase;
    });

    query.for_each_mut(|(player, mut score, player_info, team)| {
        let team_score = team_score_map.entry(team.id).or_insert((0usize, false));
        match score_map.get(&player) {
            Some(increas_score) => {
                score.val += increas_score;
                info!("{} get coin, {} score now!", player_info.name, score.val);
                *team_score = (team_score.0 + score.val, true);
            }
            None => {
                *team_score = (team_score.0 + score.val, team_score.1);
            }
        }
    });

    team_score_map
        .iter()
        .for_each(|(team_id, (team_score, is_changed))| {
            if *is_changed {
                team_score_changed_event.send(TeamScoreChangedEvent {
                    team_id: *team_id,
                    team_score: *team_score,
                });
            }
        });
}
