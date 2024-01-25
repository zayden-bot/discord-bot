#![allow(dead_code)]

use std::fmt::Display;

pub enum InfractionType {
    Warn,
    Mute,
    Kick,
    SoftBan,
    Ban,
}

impl Display for InfractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            InfractionType::Warn => "Warn",
            InfractionType::Mute => "Mute",
            InfractionType::Kick => "Kick",
            InfractionType::SoftBan => "SoftBan",
            InfractionType::Ban => "Ban",
        };
        write!(f, "{}", str)
    }
}
