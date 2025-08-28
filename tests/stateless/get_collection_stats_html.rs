use ankiconnect::Client;
use ankiconnect::endpoints::{GetCollectionStatsHtml};

#[tokio::test]
async fn test_get_collection_stats_default() {
    let client = Client::default();
    client.invoke_default::<GetCollectionStatsHtml>().await.unwrap();
}

#[tokio::test]
async fn test_get_collection_stats_whole_collection() {
    let client = Client::default();
    let request = GetCollectionStatsHtml::new(true);
    client.invoke(&request).await.unwrap();
}

#[tokio::test]
async fn test_get_collection_stats_not_whole_collection() {
    let client = Client::default();
    let request = GetCollectionStatsHtml::new(false);
    client.invoke(&request).await.unwrap();
}
