use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

use super::coin::CoinPlugin;
use super::elapsed_time::ElapsedTimePlugin;
use super::player::*;
use super::ui::UiPlugin;

#[derive(Clone, PartialEq, Eq)]
pub enum MatchState {
    WaitingForBegin,
    Playing,
    GameOver,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, StageLabel)]
pub enum GameStage {
    EventHandle, // 事件处理统一注册Stage
}

pub struct TestNetGamePlugins;

impl PluginGroup for TestNetGamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GameCorePlugin);
        group.add(PlayerPlugin);
        group.add(CoinPlugin);
        group.add(ElapsedTimePlugin);
        group.add(UiPlugin);
    }
}
pub struct GameRules {
    // Gameplay
    pub max_coin_num: usize,         // 可同时存在的最大硬币数量
    pub min_player_num: usize,       // 可以开始游戏的最小玩家数量(小于该数量不会开始游戏)
    pub max_player_num: usize,       // 游戏最大容纳的玩家数量
    pub target_score: usize,         // 得到 target_score 分数以上游戏结束
    pub min_coin_score_value: usize, // 单枚金币最x小价值
    pub max_coin_score_value: usize, // 单枚金币最大价值
    pub delay_seconds: f32,          // 延迟开始游戏的秒数(enable_delay==true 时有效) 0 表示不延迟

    // 单位外观
    pub player_brick_size: Vec2, // 玩家方块大小
    pub coin_brick_size: Vec2,   // 金币方块大小

    // UI
    pub font_path: &'static str,
    pub font_size: f32,
}

struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::First,
            GameStage::EventHandle,
            SystemStage::parallel(),
        )
        .add_state(MatchState::WaitingForBegin)
        .init_resource::<GameRules>()
        .init_resource::<GameState>()
        .init_resource::<GameDelayStart>()
        .add_startup_system(game_init_system.system())
        .add_system(game_state_update_system.system())
        .add_system(delay_start_update_system.system());
    }
}

impl Default for GameRules {
    fn default() -> Self {
        GameRules {
            max_coin_num: 3,
            min_player_num: 1,
            max_player_num: 2,
            target_score: 30,
            min_coin_score_value: 1,
            max_coin_score_value: 5,
            delay_seconds: 0.0,
            player_brick_size: Vec2::new(50.0, 50.0),
            coin_brick_size: Vec2::new(25.0, 25.0),
            font_path: "fonts/FiraSans-Bold.ttf",
            font_size: 50.0,
        }
    }
}

#[derive(Default)]
pub struct GameState {
    win_team_id: usize,
}

impl GameState {
    pub fn get_win_team_id(&self) -> usize {
        self.win_team_id
    }
}

struct GameDelayStart(Timer);

impl FromWorld for GameDelayStart {
    fn from_world(world: &mut World) -> Self {
        let rules = world.get_resource_or_insert_with(GameRules::default);
        GameDelayStart(Timer::from_seconds(rules.delay_seconds, false))
    }
}

fn game_init_system() {}

fn game_state_update_system(
    mut state: ResMut<State<MatchState>>,
    mut game_state: ResMut<GameState>,
    rules: Res<GameRules>,
    mut team_score_events: EventReader<TeamScoreChangedEvent>,
) {
    for TeamScoreChangedEvent {
        team_score,
        team_id,
    } in team_score_events.iter()
    {
        if *team_score >= rules.target_score {
            game_state.win_team_id = *team_id;
            state
                .set_next(MatchState::GameOver)
                .expect("set match state gameover fail!");
            info!("team {} win game!", team_id);
        }
    }
}

fn delay_start_update_system(
    time: Res<Time>,
    mut state: ResMut<State<MatchState>>,
    mut delay_start_timer: ResMut<GameDelayStart>,
) {
    if delay_start_timer.0.tick(time.delta()).just_finished() {
        state
            .set_next(MatchState::Playing)
            .expect("set match state fail!");

        info!("game start!");
    }
}
