use proto_did::DIDServer;

#[tokio::test]
async fn test_server() {
    DIDServer::build()
        .set_port(5000)
        .launch()
        .await;
}
