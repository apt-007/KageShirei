// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `agent_profiles` table.
    ///
    /// (Automatically generated by Diesel.)
    agent_profiles (id) {
        /// The `id` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `name` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        name -> Varchar,
        /// The `kill_date` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        kill_date -> Nullable<Int8>,
        /// The `working_hours` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Nullable<Array<Nullable<Int8>>>`.
        ///
        /// (Automatically generated by Diesel.)
        working_hours -> Nullable<Array<Nullable<Int8>>>,
        /// The `polling_interval` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        polling_interval -> Nullable<Int8>,
        /// The `polling_jitter` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        polling_jitter -> Nullable<Int8>,
        /// The `created_at` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `agent_profiles` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `agents` table.
    ///
    /// (Automatically generated by Diesel.)
    agents (id) {
        /// The `id` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `operative_system` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        operative_system -> Varchar,
        /// The `hostname` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        hostname -> Varchar,
        /// The `domain` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        domain -> Varchar,
        /// The `username` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        username -> Varchar,
        /// The `network_interfaces` column of the `agents` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        network_interfaces -> Text,
        /// The `process_id` column of the `agents` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        process_id -> Int8,
        /// The `parent_process_id` column of the `agents` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        parent_process_id -> Int8,
        /// The `process_name` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        process_name -> Varchar,
        /// The `integrity_level` column of the `agents` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        integrity_level -> Int2,
        /// The `cwd` column of the `agents` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        cwd -> Text,
        /// The `server_secret_key` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        server_secret_key -> Varchar,
        /// The `secret_key` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        secret_key -> Varchar,
        /// The `signature` column of the `agents` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        signature -> Varchar,
        /// The `terminated_at` column of the `agents` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        terminated_at -> Nullable<Timestamptz>,
        /// The `created_at` column of the `agents` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `agents` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema_extension::AgentCommandStatus;

    /// Representation of the `agents_command_requests` table.
    ///
    /// (Automatically generated by Diesel.)
    agents_command_requests (id) {
        /// The `id` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `agent_id` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        agent_id -> Varchar,
        /// The `command` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        command -> Jsonb,
        /// The `output` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        output -> Nullable<Text>,
        /// The `status` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `AgentCommandStatus`.
        ///
        /// (Automatically generated by Diesel.)
        status -> AgentCommandStatus,
        /// The `retrieved_at` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        retrieved_at -> Nullable<Timestamptz>,
        /// The `completed_at` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        completed_at -> Nullable<Timestamptz>,
        /// The `failed_at` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        failed_at -> Nullable<Timestamptz>,
        /// The `created_at` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `agents_command_requests` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `commands` table.
    ///
    /// (Automatically generated by Diesel.)
    commands (id) {
        /// The `id` column of the `commands` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `ran_by` column of the `commands` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        ran_by -> Varchar,
        /// The `command` column of the `commands` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        command -> Text,
        /// The `session_id` column of the `commands` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        session_id -> Varchar,
        /// The `output` column of the `commands` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        output -> Nullable<Text>,
        /// The `exit_code` column of the `commands` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        exit_code -> Nullable<Int4>,
        /// The `sequence_counter` column of the `commands` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        sequence_counter -> Nullable<Int8>,
        /// The `deleted_at` column of the `commands` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        deleted_at -> Nullable<Timestamptz>,
        /// The `restored_at` column of the `commands` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        restored_at -> Nullable<Timestamptz>,
        /// The `created_at` column of the `commands` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `commands` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema_extension::AgentFields;
    use crate::schema_extension::FilterOperator;
    use crate::schema_extension::LogicalOperator;

    /// Representation of the `filters` table.
    ///
    /// (Automatically generated by Diesel.)
    filters (id) {
        /// The `id` column of the `filters` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `agent_profile_id` column of the `filters` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        agent_profile_id -> Varchar,
        /// The `agent_field` column of the `filters` table.
        ///
        /// Its SQL type is `AgentFields`.
        ///
        /// (Automatically generated by Diesel.)
        agent_field -> AgentFields,
        /// The `filter_op` column of the `filters` table.
        ///
        /// Its SQL type is `FilterOperator`.
        ///
        /// (Automatically generated by Diesel.)
        filter_op -> FilterOperator,
        /// The `value` column of the `filters` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        value -> Text,
        /// The `sequence` column of the `filters` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        sequence -> Int4,
        /// The `next_hop_relation` column of the `filters` table.
        ///
        /// Its SQL type is `Nullable<LogicalOperator>`.
        ///
        /// (Automatically generated by Diesel.)
        next_hop_relation -> Nullable<LogicalOperator>,
        /// The `grouping_start` column of the `filters` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        grouping_start -> Bool,
        /// The `grouping_end` column of the `filters` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        grouping_end -> Bool,
        /// The `created_at` column of the `filters` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `filters` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema_extension::LogLevel;

    /// Representation of the `logs` table.
    ///
    /// (Automatically generated by Diesel.)
    logs (id) {
        /// The `id` column of the `logs` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `level` column of the `logs` table.
        ///
        /// Its SQL type is `LogLevel`.
        ///
        /// (Automatically generated by Diesel.)
        level -> LogLevel,
        /// The `message` column of the `logs` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        message -> Nullable<Text>,
        /// The `title` column of the `logs` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Nullable<Text>,
        /// The `extra` column of the `logs` table.
        ///
        /// Its SQL type is `Nullable<Jsonb>`.
        ///
        /// (Automatically generated by Diesel.)
        extra -> Nullable<Jsonb>,
        /// The `created_at` column of the `logs` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `logs` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema_extension::LogLevel;

    /// Representation of the `notifications` table.
    ///
    /// (Automatically generated by Diesel.)
    notifications (id) {
        /// The `id` column of the `notifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `level` column of the `notifications` table.
        ///
        /// Its SQL type is `LogLevel`.
        ///
        /// (Automatically generated by Diesel.)
        level -> LogLevel,
        /// The `message` column of the `notifications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        message -> Text,
        /// The `title` column of the `notifications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `created_at` column of the `notifications` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `notifications` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 32]
        id -> Varchar,
        /// The `username` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        username -> Varchar,
        /// The `password` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        password -> Varchar,
        /// The `created_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(agents_command_requests -> agents (agent_id));
diesel::joinable!(commands -> users (ran_by));
diesel::joinable!(filters -> agent_profiles (agent_profile_id));

diesel::allow_tables_to_appear_in_same_query!(
    agent_profiles,
    agents,
    agents_command_requests,
    commands,
    filters,
    logs,
    notifications,
    users,
);
