use ankiconnect::Client;
use ankiconnect::endpoints::{GetLatestReviewId};

#[tokio::test]
async fn test_get_latest_review_id() {
    let client = Client::default();
    let request = GetLatestReviewId::new("Default");
    client.invoke(&request).await.unwrap();
}
