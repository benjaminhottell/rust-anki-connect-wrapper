use ankiconnect::client::Client;
use ankiconnect::endpoints::{ModelStyling, ModelNames};

#[tokio::test]
async fn test_model_styling() {
    let client = Client::default();

    let model_names = client.invoke_default::<ModelNames>().await.unwrap();

    for model_name in model_names {
        let request = ModelStyling::new(model_name);
        client.invoke(&request).await.unwrap();
    }

}
