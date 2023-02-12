
use tokio::sync::RwLock;
use agent_config::ConfigStruct;

mod agent_config;
mod status_service;


#[tokio::main]
async fn main() {
    let global_config = Arc::new(RwLock::new(agent_config::initial_config()));

    tokio::spawn(start_web_server(global_config.clone()));

}
