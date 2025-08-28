use ankiconnect::Client;
use ankiconnect::endpoints::{
    FindCards,
    GetReviewsOfCards,
};

#[tokio::test]
async fn test_get_reviews_of_cards_on_empty() {
    let client = Client::default();
    let request = GetReviewsOfCards::new(&[]);
    client.invoke(&request).await.unwrap();
}

#[tokio::test]
async fn test_get_reviews_of_cards_on_a_few_cards() {
    let client = Client::default();

    let cards = client.invoke_default::<FindCards>().await.unwrap();

    let cards = &cards[..cards.len().min(3)];

    let request = GetReviewsOfCards::new(cards);
    client.invoke(&request).await.unwrap();
}
