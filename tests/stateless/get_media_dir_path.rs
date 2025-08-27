use ankiconnect::Client;
use ankiconnect::endpoints::GetMediaDirPath;

#[tokio::test]
async fn test_get_media_dir_path() {
    let client = Client::default();
    client.invoke_default::<GetMediaDirPath>().await.unwrap();
}
