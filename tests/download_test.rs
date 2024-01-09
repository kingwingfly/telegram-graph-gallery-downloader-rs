#[tokio::test]
async fn download_test() {
    let url_str = "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93%E7%BA%B3%E8%A5%BF%E5%A6%B2-37P-409MB-08-30";
    telegram_graph_downloader::download(std::env::var("TEST_SAVE_DIR").unwrap(), url_str)
        .await
        .unwrap();
}
