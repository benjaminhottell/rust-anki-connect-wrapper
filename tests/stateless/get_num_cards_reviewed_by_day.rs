use ankiconnect::client::Client;
use ankiconnect::endpoints::GetNumCardsReviewedByDay;

#[tokio::test]
async fn test_get_num_cards_reviewed_by_day() {
    let client = Client::default();
    client.invoke_default::<GetNumCardsReviewedByDay>().await.unwrap();
}
