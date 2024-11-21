//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, ActiveValue::Set};
use serde::{Deserialize, Serialize};

use crate::helpers::CUID2;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "read_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id:      String,
    pub log_id:  String,
    pub read_by: String,
    pub read_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::logs::Entity",
        from = "Column::LogId",
        to = "super::logs::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Logs,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ReadBy",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::logs::Entity> for Entity {
    fn to() -> RelationDef { Relation::Logs.def() }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::User.def() }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            // Generate a new unique ID
            id: Set(CUID2.create_id()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Clone the model to avoid moving it
        let mut model = self;

        // Update the `updated_at` field with the current time
        model.read_at = Set(Utc::now().naive_utc());
        Ok(model)
    }
}