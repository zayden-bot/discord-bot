use std::collections::HashMap;

use serenity::prelude::TypeMapKey;

type CooldownCondition = fn(&str) -> bool;

#[derive(Debug, Default, Clone)]
pub struct State {
    pub cooldown_conditions: HashMap<String, CooldownCondition>,
}

impl State {
    pub fn new() -> Self {
        Self {
            cooldown_conditions: HashMap::new(),
        }
    }
}

impl TypeMapKey for State {
    type Value = State;
}
