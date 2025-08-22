use ankiconnect::Client;
use ankiconnect::endpoints::{FindCards, GetEaseFactors};

#[tokio::test]
async fn test_get_ease_factors() {
    let client = Client::default();
    client.invoke_default::<GetEaseFactors>().await.unwrap();
}

#[tokio::test]
async fn test_get_ease_factors_for_many_cards() {
    let client = Client::default();

    let cards = client.invoke_default::<FindCards>().await.unwrap();

    let cards = &cards[..3];

    let request = GetEaseFactors::new(cards);
    client.invoke(&request).await.unwrap();
}
