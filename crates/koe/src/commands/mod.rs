//! Slash commands for the bot

mod dict;
mod help;
mod join;
mod leave;
mod skip;
mod voice;

pub use dict::dict;
pub use help::help;
pub use join::join;
pub use leave::leave;
pub use skip::skip;
pub use voice::voice;

/// Returns all commands for the bot
pub fn commands() -> Vec<poise::Command<crate::app_state::AppState, anyhow::Error>> {
    vec![join(), leave(), skip(), voice(), dict(), help()]
}
