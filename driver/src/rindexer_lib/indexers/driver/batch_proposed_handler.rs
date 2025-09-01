#![allow(non_snake_case)]
use crate::rindexer_lib::typings::driver::events::mainnet_inbox::BatchProposedEvent;

use super::super::super::typings::driver::events::mainnet_inbox::{
    no_extensions, BatchProposedData, MainnetInboxEventType,
};
use alloy::primitives::{I256, U256, U64};
use rindexer::{
    event::callback_registry::EventCallbackRegistry, rindexer_error, rindexer_info,
    EthereumSqlTypeWrapper, PgType, RindexerColorize,
};
use std::path::PathBuf;
use std::sync::Arc;

async fn on_batch_proposed(manifest_path: &PathBuf, registry: &mut EventCallbackRegistry) {
    MainnetInboxEventType::BatchProposed(
        BatchProposedEvent::handler(
            |results, context| async move {
                // logic here
                rindexer_info!("batch proposed handler");
                Ok(())
            },
            no_extensions(),
        )
        .await,
    )
    .register(manifest_path, registry)
    .await;
}
pub async fn mainnet_inbox_handlers(manifest_path: &PathBuf, registry: &mut EventCallbackRegistry) {
    on_batch_proposed(manifest_path, registry).await;
}
