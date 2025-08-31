use ankiconnect::Client;
use ankiconnect::endpoints::{FindCards, AreDue};

#[tokio::test]
async fn test_are_due_default() {
    let client = Client::default();
    client.invoke_default::<AreDue>().await.unwrap();
}

#[tokio::test]
async fn test_due_cards_are_due() {

    let client = Client::default();

    let request = FindCards::new("is:due");
    let cards = client.invoke(&request).await.unwrap();

    let request = AreDue::new(&cards);
    let are_due = client.invoke(&request).await.unwrap();

    assert_eq!(are_due.len(), cards.len());

    for x in are_due {
        assert!(x)
    }

}
