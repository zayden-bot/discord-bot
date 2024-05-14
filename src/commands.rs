use std::collections::HashMap;

use serenity::{
    all::{CommandInteraction, Context},
    prelude::TypeMapKey,
};

use crate::Result;

type SlashCommand = fn(&Context, &CommandInteraction) -> Result<()>;

#[derive(Debug, Default, Clone)]
pub struct Commands {
    pub slash_commands: HashMap<String, SlashCommand>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            slash_commands: HashMap::new(),
        }
    }
}

impl TypeMapKey for Commands {
    type Value = Commands;
}
