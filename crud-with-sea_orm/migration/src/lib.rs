pub use sea_orm_migration::prelude::*;

use crate::migrations::m20220101_000001_post;
use crate::migrations::m20260121_020308_users;
use crate::seeds::m20220120_000002_seed_posts;
mod migrations;
mod seeds;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_post::Migration),
            Box::new(m20260121_020308_users::Migration),
            Box::new(m20220120_000002_seed_posts::Migration),
        ]
    }
}
