use serenity::all::UserId;
use sqlx::{Database, Pool};

use crate::GamblingGoalsRow;
use crate::GoalsManager;
use crate::events::Event;
use crate::events::EventRow;

use super::GOAL_REGISTRY;

pub struct GoalHandler;

impl GoalHandler {
    pub async fn daily_reset<Db: Database, Manager: GoalsManager<Db>>(
        pool: &Pool<Db>,
        id: impl Into<UserId>,
        row: &dyn EventRow,
    ) -> sqlx::Result<Vec<GamblingGoalsRow>> {
        let id = id.into();

        let selected_goal_definitions = GOAL_REGISTRY.select_daily_goal();

        let goals = selected_goal_definitions
            .into_iter()
            .map(|goal| {
                let target_value = (goal.target)(row);
                (goal.id, target_value)
            })
            .map(|(goal_id, target)| GamblingGoalsRow::new(id, goal_id, target))
            .collect::<Vec<_>>();

        let rows = Manager::update(pool, &goals).await?;

        Ok(rows)
    }

    pub async fn get_user_progress<Db: Database, Manager: GoalsManager<Db>>(
        pool: &Pool<Db>,
        user_id: impl Into<UserId>,
        row: &dyn EventRow,
    ) -> sqlx::Result<Vec<GamblingGoalsRow>> {
        let user_id = user_id.into();

        let mut goals = Manager::full_rows(pool, user_id).await?;

        if goals.is_empty() || !goals[0].is_today() {
            goals = Self::daily_reset::<Db, Manager>(pool, user_id, row).await?;
        }

        Ok(goals)
    }

    pub async fn process_goals<Db: Database, Manager: GoalsManager<Db>>(
        pool: &Pool<Db>,
        row: &mut dyn EventRow,
        event: Event,
    ) -> sqlx::Result<Event> {
        let user_id = event.user_id();

        let mut all_goals = Self::get_user_progress::<Db, Manager>(pool, user_id, row).await?;

        let changed = all_goals
            .iter_mut()
            .filter(|goal| !goal.is_complete())
            .filter_map(|goal| {
                GOAL_REGISTRY
                    .get_definition(goal.goal_id())
                    .map(|definition| (goal, definition))
            })
            .fold(Vec::new(), |mut acc, (goal, definition)| {
                let changed = (definition.update_fn)(goal, &event);

                if changed {
                    acc.push(goal);
                }

                acc
            });

        changed
            .iter()
            .filter(|goal| goal.is_complete())
            .for_each(|_| row.add_coins(5_000));

        if !changed.is_empty() {
            if all_goals.iter().all(|row| row.is_complete()) {
                row.add_gems(1);
            }

            Manager::update(pool, &all_goals).await.unwrap();
        }

        Ok(event)
    }
}
