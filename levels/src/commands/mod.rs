use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

mod levels;
mod rank;
mod xp;

pub struct Commands;

impl Commands {
    pub fn register() -> [CreateCommand; 3] {
        let levels = CreateCommand::new("levels").description("Get the leaderboard");

        let rank = CreateCommand::new("rank")
            .description("Get your rank or another member's rank")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to get the xp of",
            ))
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "ephemeral",
                "Whether the response should be ephemeral",
            ));

        let xp = CreateCommand::new("xp")
            .description("Get your current xp")
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "ephemeral",
                "Whether the response should be ephemeral",
            ));

        [levels, rank, xp]
    }
}
