pub mod rindexer_lib;

use rindexer::{
    event::callback_registry::TraceCallbackRegistry, start_rindexer, GraphqlOverrideSettings,
    IndexingDetails, StartDetails,
};
use rindexer_lib::indexers::all_handlers::register_all_handlers;
use std::env;

pub async fn start_taiko_indexer(
    enable_graphql: bool,
    enable_indexer: bool,
    port: Option<u16>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let manifest_path = path.join("driver").join("rindexer.yaml");

    let result = start_rindexer(StartDetails {
        manifest_path: &manifest_path,
        indexing_details: if enable_indexer {
            Some(IndexingDetails {
                registry: register_all_handlers(&manifest_path).await,
                trace_registry: TraceCallbackRegistry { events: vec![] },
            })
        } else {
            None
        },
        graphql_details: GraphqlOverrideSettings { enabled: enable_graphql, override_port: port },
    })
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
