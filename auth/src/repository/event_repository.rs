use sea_orm::{ ActiveModelTrait, ActiveValue::{ NotSet, Set }, DatabaseTransaction };
use anyhow::Result;

use crate::model::{entities::events, event::auth_event::Event};

#[tracing::instrument(skip(tx, event))]
pub async fn save(
    tx: &DatabaseTransaction,
    event:Event
) -> Result<()> {
    (events::ActiveModel {
        id: NotSet,
        source: Set(event.source),
        event_id: Set(event.id.to_string()),
        payload: Set(event.payload),
        event_type: Set(event.event_type),
        version: Set(1),
    }).insert(tx).await?;
    Ok(())
}
