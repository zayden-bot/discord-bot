#![allow(dead_code)]
pub enum InfractionType {
    Warn,
    Mute,
    Kick,
    SoftBan,
    Ban,
}

impl ToString for InfractionType {
    fn to_string(&self) -> String {
        match self {
            InfractionType::Warn => "Warn",
            InfractionType::Mute => "Mute",
            InfractionType::Kick => "Kick",
            InfractionType::SoftBan => "SoftBan",
            InfractionType::Ban => "Ban",
        }.to_string()
    }
}