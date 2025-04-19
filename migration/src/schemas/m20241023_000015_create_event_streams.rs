// use sea_orm::Statement;
// use sea_orm_migration::prelude::*;

// #[derive(DeriveMigrationName)]
// pub struct Migration;

// #[async_trait::async_trait]
// impl MigrationTrait for Migration {
//     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
//         let db = manager.get_connection();

//         // 创建 event_streams 表
//         manager.create_table(
//             Table::create()
//                 .table(Alias::new("event_streams"))
//                 .if_not_exists()
//                 .col(ColumnDef::new(Alias::new("event_stream_id")).text().not_null().primary_key())
//                 .col(
//                     ColumnDef::new(Alias::new("version"))
//                         .integer()
//                         .not_null()
//                         .check(Expr::cust("\"version\" > 0"))
//                 )
//                 .to_owned()
//         ).await?;

//         // 创建 events 表
//         manager.create_table(
//             Table::create()
//                 .table(Alias::new("events"))
//                 .if_not_exists()
//                 .col(ColumnDef::new(Alias::new("event_stream_id")).text().not_null())
//                 .col(ColumnDef::new(Alias::new("type")).text().not_null())
//                 .col(
//                     ColumnDef::new(Alias::new("version"))
//                         .integer()
//                         .not_null()
//                         .check(Expr::cust("\"version\" > 0"))
//                 )
//                 .col(ColumnDef::new(Alias::new("event")).binary().not_null())
//                 .col(ColumnDef::new(Alias::new("metadata")).json_binary())
//                 .primary_key(
//                     Index::create()
//                         .name("pk_events")
//                         .col(Alias::new("event_stream_id"))
//                         .col(Alias::new("version"))
//                 )
//                 .foreign_key(
//                     ForeignKey::create()
//                         .name("fk_events_event_stream_id")
//                         .from(Alias::new("events"), Alias::new("event_stream_id"))
//                         .to(Alias::new("event_streams"), Alias::new("event_stream_id"))
//                         .on_delete(ForeignKeyAction::Cascade)
//                 )
//                 .to_owned()
//         ).await?;

//         // 创建 event_stream_id 索引
//         manager.create_index(
//             Index::create()
//                 .name("event_stream_id_idx")
//                 .table(Alias::new("events"))
//                 .col(Alias::new("event_stream_id"))
//                 .to_owned()
//         ).await?;

//         // 创建 aggregates 表
//         manager.create_table(
//             Table::create()
//                 .table(Alias::new("aggregates"))
//                 .if_not_exists()
//                 .col(ColumnDef::new(Alias::new("aggregate_id")).text().not_null().primary_key())
//                 .col(ColumnDef::new(Alias::new("type")).text().not_null())
//                 .col(
//                     ColumnDef::new(Alias::new("version"))
//                         .integer()
//                         .not_null()
//                         .check(Expr::cust("\"version\" > 0"))
//                 )
//                 .col(ColumnDef::new(Alias::new("state")).binary().not_null())
//                 .foreign_key(
//                     ForeignKey::create()
//                         .name("fk_aggregates_event_stream_id")
//                         .from(Alias::new("aggregates"), Alias::new("aggregate_id"))
//                         .to(Alias::new("event_streams"), Alias::new("event_stream_id"))
//                         .on_delete(ForeignKeyAction::Cascade)
//                 )
//                 .to_owned()
//         ).await?;

//         // 创建存储过程和函数
//         let procedures = vec![
//             // upsert_event_stream 存储过程
//             r#"
//             CREATE OR REPLACE PROCEDURE upsert_event_stream(
//                 _event_stream_id TEXT,
//                 _expected_version INTEGER,
//                 _new_version INTEGER
//             )
//             LANGUAGE PLPGSQL
//             AS $$
//             DECLARE
//                 current_event_stream_version INTEGER;
//             BEGIN
//                 -- Retrieve the latest version for the target Event Stream.
//                 SELECT es."version"
//                 INTO current_event_stream_version
//                 FROM event_streams es
//                 WHERE es.event_stream_id = _event_stream_id;

//                 IF (NOT FOUND AND _expected_version <> 0) OR (current_event_stream_version <> _expected_version)
//                 THEN
//                     RAISE EXCEPTION 'event stream version check failed, expected: %, got: %', _expected_version, current_event_stream_version;
//                 END IF;

//                 INSERT INTO event_streams (event_stream_id, "version")
//                 VALUES (_event_stream_id, _new_version)
//                 ON CONFLICT (event_stream_id) DO
//                 UPDATE SET "version" = _new_version;
//             END;
//             $$;
//             "#,

//             // upsert_event_stream_with_no_version_check 函数
//             r#"
//             CREATE OR REPLACE FUNCTION upsert_event_stream_with_no_version_check(
//                 _event_stream_id TEXT,
//                 _new_version_offset INTEGER
//             )
//             RETURNS INTEGER
//             LANGUAGE PLPGSQL
//             AS $$
//             DECLARE
//                 current_event_stream_version INTEGER;
//                 new_event_stream_version INTEGER;
//             BEGIN
//                 -- Retrieve the latest version for the target Event Stream.
//                 SELECT es."version"
//                 INTO current_event_stream_version
//                 FROM event_streams es
//                 WHERE es.event_stream_id = _event_stream_id;

//                 IF NOT FOUND THEN
//                     current_event_stream_version := 0;
//                 END IF;

//                 new_event_stream_version := current_event_stream_version + _new_version_offset;

//                 INSERT INTO event_streams (event_stream_id, "version")
//                 VALUES (_event_stream_id, new_event_stream_version)
//                 ON CONFLICT (event_stream_id) DO
//                 UPDATE SET "version" = new_event_stream_version;

//                 RETURN new_event_stream_version;
//             END;
//             $$;
//             "#,

//             // upsert_aggregate 存储过程
//             r#"
//             CREATE OR REPLACE PROCEDURE upsert_aggregate(
//                 _aggregate_id TEXT,
//                 _type TEXT,
//                 _expected_version INTEGER,
//                 _new_version INTEGER,
//                 _state BYTEA
//             )
//             LANGUAGE PLPGSQL
//             AS $$
//             DECLARE
//                 current_aggregate_version INTEGER;
//             BEGIN
//                 -- Retrieve the latest version for the target aggregate.
//                 SELECT a."version"
//                 INTO current_aggregate_version
//                 FROM aggregates a
//                 WHERE a.aggregate_id = _aggregate_id;

//                 IF (NOT FOUND AND _expected_version <> 0) OR (current_aggregate_version <> _expected_version)
//                 THEN
//                     RAISE EXCEPTION 'aggregate version check failed, expected: %, got: %', _expected_version, current_aggregate_version;
//                 END IF;

//                 -- An Aggregate Root is also an Event Stream.
//                 INSERT INTO event_streams (event_stream_id, "version")
//                 VALUES (_aggregate_id, _new_version)
//                 ON CONFLICT (event_stream_id) DO
//                 UPDATE SET "version" = _new_version;

//                 INSERT INTO aggregates (aggregate_id, "type", "version", "state")
//                 VALUES (_aggregate_id, _type, _new_version, _state)
//                 ON CONFLICT (aggregate_id) DO
//                 UPDATE SET "version" = _new_version, "state" = _state;
//             END;
//             $$;
//             "#
//         ];

//         for sql in procedures {
//             db.execute(
//                 Statement::from_string(manager.get_database_backend(), sql.to_string())
//             ).await?;
//         }

//         Ok(())
//     }

//     async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
//         // 删除顺序需要反向，先删除依赖的表
//         manager.drop_table(Table::drop().table(Alias::new("aggregates")).to_owned()).await?;

//         manager.drop_table(Table::drop().table(Alias::new("events")).to_owned()).await?;

//         manager.drop_table(Table::drop().table(Alias::new("event_streams")).to_owned()).await?;

//         // 删除存储过程和函数
//         let drop_objects = vec![
//             "DROP PROCEDURE IF EXISTS upsert_event_stream",
//             "DROP FUNCTION IF EXISTS upsert_event_stream_with_no_version_check",
//             "DROP PROCEDURE IF EXISTS upsert_aggregate"
//         ];

//         for sql in drop_objects {
//             manager
//                 .get_connection()
//                 .execute(
//                     Statement::from_string(manager.get_database_backend(), sql.to_string())
//                 ).await?;
//         }

//         Ok(())
//     }
// }
