use ankiconnect::Client;
use ankiconnect::endpoints::Version;

#[tokio::test]
async fn test_version() {
    let client = Client::default();
    client.invoke_default::<Version>().await.unwrap();
}
