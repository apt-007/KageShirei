//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, ActiveValue::Set};
use serde::{Deserialize, Serialize};

use crate::{
    active_enums::{AgentField, FilterOperation, LogicalOperator},
    helpers::CUID2,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "filter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id:                String,
    pub agent_profile_id:  String,
    pub agent_field:       AgentField,
    pub filter_op:         FilterOperation,
    pub value:             Json,
    pub sequence:          i32,
    pub next_hop_relation: Option<LogicalOperator>,
    pub grouping_start:    bool,
    pub grouping_end:      bool,
    pub created_at:        DateTime,
    pub updated_at:        DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::agent_profile::Entity",
        from = "Column::AgentProfileId",
        to = "super::agent_profile::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AgentProfile,
}

impl Related<super::agent_profile::Entity> for Entity {
    fn to() -> RelationDef { Relation::AgentProfile.def() }
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

    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Clone the model to avoid moving it
        let mut model = self;

        if insert {
            // Update the `created_at` field with the current time
            model.created_at = Set(Utc::now().naive_utc());
        }

        // Update the `updated_at` field with the current time
        model.updated_at = Set(Utc::now().naive_utc());
        Ok(model)
    }
}
