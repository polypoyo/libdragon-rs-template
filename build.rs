use libdragon_build::{Result, Build};

fn main() -> Result<()> {
    Build::new()
        .set_env_file(".libdragon-env")
        .set_just_file(".libdragon-just")
        .set_game_name("TEMPLATE")
        .build()
}
