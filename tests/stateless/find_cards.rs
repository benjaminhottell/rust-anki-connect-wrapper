use ankiconnect::Client;
use ankiconnect::endpoints::FindCards;

#[tokio::test]
async fn test_find_all_cards() {
    let client = Client::default();
    client.invoke_default::<FindCards>().await.unwrap();
}

#[tokio::test]
async fn test_find_suspended_cards() {
    let client = Client::default();
    let request = FindCards::new("is:suspended");
    client.invoke(&request).await.unwrap();
}
