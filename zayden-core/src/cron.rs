use std::str::FromStr;
use std::sync::Arc;
use std::{marker::PhantomData, pin::Pin};

use cron::Schedule;
use serenity::all::Context;
use serenity::prelude::TypeMapKey;
use sqlx::{Database, Pool};

pub type ActionFn<Db> =
    Arc<dyn Fn(Context, Pool<Db>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Clone)]
pub struct CronJob<Db: Database> {
    pub schedule: Schedule,
    pub action_fn: ActionFn<Db>,
}

impl<Db> CronJob<Db>
where
    Db: Database,
{
    pub fn new(source: &str) -> Self {
        Self {
            schedule: Schedule::from_str(source).unwrap(),
            action_fn: Self::action_fn(|_, _| async {}),
        }
    }

    fn action_fn<F, Fut>(f: F) -> ActionFn<Db>
    where
        F: Fn(Context, Pool<Db>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let action_closure = move |ctx, pool| {
            let future = f(ctx, pool);
            let boxed_dyn_future: Box<dyn Future<Output = ()> + Send> = Box::new(future);

            let pinned_future: Pin<Box<dyn Future<Output = ()> + Send>> =
                Box::into_pin(boxed_dyn_future);

            pinned_future
        };

        Arc::new(action_closure)
    }

    pub fn set_action<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Context, Pool<Db>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.action_fn = Self::action_fn(f);
        self
    }
}

pub struct CronJobs<Db: Database>(PhantomData<Db>);

impl<Db: Database> TypeMapKey for CronJobs<Db> {
    type Value = Vec<CronJob<Db>>;
}
