use ankiconnect::Client;
use ankiconnect::endpoints::GetMediaFilesNames;

#[tokio::test]
async fn test_media_files_names_get_all() {
    let client = Client::default();
    client.invoke_default::<GetMediaFilesNames>().await.unwrap();
}

#[tokio::test]
async fn test_media_files_names_get_specific() {
    let client = Client::default();
    let request = GetMediaFilesNames::new("hello-world.txt");
    client.invoke(&request).await.unwrap();
}
