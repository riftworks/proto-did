use std::{collections::HashMap, env};
use proto_did::{cli::start_cli, DIDServer};

#[tokio::test]
async fn test_server() {
    let env_vars = env::vars().collect::<HashMap<String, String>>();
    let mode = env_vars.get("MODE");

    if mode.is_some() && mode.unwrap() == "cli" {
        start_cli().await;
    } else {
        DIDServer::build()
            .set_port(5000)
            .launch()
            .await;
    }
}
