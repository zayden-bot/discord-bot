use std::{fmt::Display, time::Duration};

pub type Result<T> = std::result::Result<T, Error>;

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 3600;

#[derive(Debug)]
pub enum Error {
    SelfStar,
    NoStars(String),
}

impl Error {
    pub fn no_stars(t: Duration) -> Self {
        let hours = t.as_secs() / SECS_PER_HOUR;
        let minutes = (t.as_secs() % SECS_PER_HOUR) / SECS_PER_MINUTE;
        let seconds = t.as_secs() % SECS_PER_MINUTE;

        Error::NoStars(format!(
            "You don't have any stars to give.\nNext free star in: {}h {}m {}s.",
            hours, minutes, seconds
        ))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfStar => write!(f, "You can't give yourself a star."),
            Self::NoStars(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}
