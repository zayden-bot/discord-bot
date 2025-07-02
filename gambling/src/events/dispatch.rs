use std::marker::PhantomData;

use sqlx::{Database, Pool};

use crate::GoalsManager;
use crate::goals::GoalHandler;

use super::{Event, EventRow};

pub struct Dispatch<'a, Db: Database, Manager: GoalsManager<Db>> {
    pool: &'a Pool<Db>,
    _manager: PhantomData<Manager>,
}

impl<'a, Db, Manager> Dispatch<'a, Db, Manager>
where
    Db: Database,
    Manager: GoalsManager<Db>,
{
    pub fn new(pool: &'a Pool<Db>) -> Self {
        Self {
            pool,
            _manager: PhantomData,
        }
    }

    pub async fn fire(&self, row: &mut dyn EventRow, event: Event) -> sqlx::Result<Event> {
        GoalHandler::process_goals::<Db, Manager>(self.pool, row, event).await
    }
}
