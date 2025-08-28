use ankiconnect::Client;
use ankiconnect::endpoints::{GetDeckStats};

#[tokio::test]
async fn test_get_deck_stats_default() {
    let client = Client::default();
    let result = client.invoke_default::<GetDeckStats>().await.unwrap();
    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_get_deck_stats_single() {
    let client = Client::default();
    let request = GetDeckStats::single("Default");
    client.invoke(&request).await.unwrap();
}

#[tokio::test]
async fn test_get_deck_stats_many() {
    // This doesn't actually test getting multiple decks, just the act of passing a container to
    // the constructor.
    // Getting stats for deck names that don't exist will cause them to be created(!)
    let client = Client::default();
    let decks = [
        "Default".into(),
    ];
    let request = GetDeckStats::new(decks);
    client.invoke(&request).await.unwrap();
}
