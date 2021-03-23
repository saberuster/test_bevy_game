use super::{
    elapsed_time::OnElapsedSecondChangedEvent,
    game::{GameRules, GameState, MatchState},
    player::TeamScoreChangedEvent,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(score_ui_system.system())
            .add_system(elapsed_time_ui_system.system())
            .add_system_set(
                SystemSet::on_enter(MatchState::GameOver).with_system(gameover_ui_system.system()),
            );
    }
}

struct GameOverUI;
struct ScoreUI;

struct ElapsedTimeUI;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, rules: Res<GameRules>) {
    let font = asset_server.load(rules.font_path);
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert_bundle(UiCameraBundle::default());
    commands
        .spawn()
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },

            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![TextSection {
                    value: "no team score".into(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: rules.font_size,
                        color: Color::RED,
                    },
                }],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreUI);

    commands
        .spawn()
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text::with_section(
                format!("elapsed time: {}", 0),
                TextStyle {
                    font: font.clone(),
                    font_size: rules.font_size,
                    color: Color::GREEN,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Right,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ElapsedTimeUI);
}

fn score_ui_system(
    mut events: EventReader<TeamScoreChangedEvent>,
    mut query: Query<&mut Text, With<ScoreUI>>,
    asset_server: Res<AssetServer>,
    rules: Res<GameRules>,
) {
    if let Ok(mut text) = query.single_mut() {
        let mut clear = false;
        events.iter().for_each(
            |TeamScoreChangedEvent {
                 team_score,
                 team_id,
             }| {
                if !clear {
                    text.sections.clear();
                    clear = true;
                }
                let font = asset_server.load(rules.font_path);
                text.sections.push(TextSection {
                    value: format!(" |team {}: {} ", team_id, team_score),
                    style: TextStyle {
                        font,
                        font_size: rules.font_size,
                        color: Color::RED,
                    },
                });
            },
        );
    }
}

fn gameover_ui_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    rules: Res<GameRules>,
) {
    let font = asset_server.load(rules.font_path);
    commands
        .spawn()
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(50.0),
                    left: Val::Percent(40.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text::with_section(
                format!("team {} win game", game_state.get_win_team_id()),
                TextStyle {
                    font: font.clone(),
                    font_size: rules.font_size,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Right,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(GameOverUI);
}

fn elapsed_time_ui_system(
    mut events: EventReader<OnElapsedSecondChangedEvent>,
    mut query: Query<&mut Text, With<ElapsedTimeUI>>,
    asset_server: Res<AssetServer>,
    rules: Res<GameRules>,
) {
    if let Ok(mut text) = query.single_mut() {
        let mut clear = false;
        events
            .iter()
            .for_each(|OnElapsedSecondChangedEvent { seconds }| {
                if !clear {
                    text.sections.clear();
                    clear = true;
                }
                let font = asset_server.load(rules.font_path);
                text.sections.push(TextSection {
                    value: format!("elapsed time: {}", seconds),
                    style: TextStyle {
                        font,
                        font_size: rules.font_size,
                        color: Color::GREEN,
                    },
                });
            });
    }
}
