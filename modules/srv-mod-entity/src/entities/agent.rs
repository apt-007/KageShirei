//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use kageshirei_communication_protocol::network_interface::NetworkInterface;
use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, ActiveValue::Set, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::{active_enums::AgentIntegrity, helpers::CUID2};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "agent")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id:                 String,
    pub operating_system:   String,
    pub hostname:           String,
    pub domain:             Option<String>,
    pub username:           String,
    pub network_interfaces: Vec<NetworkInterface>,
    pub pid:                i64,
    pub ppid:               i64,
    pub process_name:       String,
    pub integrity:          AgentIntegrity,
    pub cwd:                String,
    pub server_secret:      String,
    pub secret:             String,
    #[sea_orm(unique)]
    pub signature:          String,
    pub terminated_at:      Option<DateTime>,
    pub created_at:         DateTime,
    pub updated_at:         DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::agent_command::Entity")]
    AgentCommand,
    #[sea_orm(has_many = "super::terminal_history::Entity")]
    TerminalHistory,
}

impl Related<super::agent_command::Entity> for Entity {
    fn to() -> RelationDef { Relation::AgentCommand.def() }
}

impl Related<super::terminal_history::Entity> for Entity {
    fn to() -> RelationDef { Relation::TerminalHistory.def() }
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