#[tokio::main]
async fn main() {
    let urls = [
        "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93-%E7%88%B1%E8%8E%89%E5%B8%8C%E9%9B%85-33P-352MB-08-28",
        "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93%E7%BA%B3%E8%A5%BF%E5%A6%B2-37P-409MB-08-30",
        "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93%E5%8E%9F%E7%A5%9EYae-Miko-35P-465MB-09-23",
        "https://telegra.ph/VIP%E5%A4%9C%E5%A4%9C---%E9%BA%BB%E8%A1%A3%E5%AD%A6%E5%A7%90-87P1V-306G-12-15",
        "https://telegra.ph/VIP%E5%A4%9C%E5%A4%9C-%E5%88%9D%E9%9F%B3%E5%B0%91%E5%A5%B3-R18-130P1V-575G-12-16",
        "https://telegra.ph/%E6%A1%9C%E4%BA%95%E5%AE%81%E5%AE%81-%E7%99%BD%E8%89%B2%E7%83%AD%E8%A3%A4%E6%AF%94%E5%9F%BA%E5%B0%BC-70P-271GB-12-26"
    ];
    for url in urls {
        telegram_photo_gallery_downloader::download(std::env::var("TEST_SAVE_DIR").unwrap(), url)
            .await
            .unwrap();
    }
    println!("Finished!")
}
