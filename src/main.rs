use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, log::Level};
mod plugins;

use plugins::TestNetGamePlugins;
// 游戏窗口 Title 名字
const GAME_WINDOW_TITLE: &str = "openra-rs bevy example";

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: GAME_WINDOW_TITLE.to_string(),
        ..Default::default()
    })
    .insert_resource(AssetServerSettings {
        // 这里重置 assets 目录防止 ide debug 的时候程序是在生成 target 文件夹里跑导致 assets 文件夹找不到
        asset_folder: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
    })
    .insert_resource(LogSettings {
        level: Level::DEBUG,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(TestNetGamePlugins);

    app.run();
}
