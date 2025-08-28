use ankiconnect::Client;
use ankiconnect::endpoints::{CardReviews, DeckNames};

#[tokio::test]
async fn test_card_reviews_on_default() {
    let client = Client::default();
    let request = CardReviews::new("Default", 0);
    client.invoke(&request).await.unwrap();
}

#[tokio::test]
async fn test_card_reviews_on_every_deck() {
    let client = Client::default();

    let decks = client.invoke_default::<DeckNames>().await.unwrap();

    for deck in decks {
        let request = CardReviews::new(deck, 0);
        client.invoke(&request).await.unwrap();
    }
}
