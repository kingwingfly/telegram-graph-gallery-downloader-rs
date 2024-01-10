mod downloader;
mod parser;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Download images from telegram graph gallery
/// # Example
/// ```no_run
/// use telegram_photo_gallery_downloader::download;
///
/// # async fn run() {
/// download("dir_to_save", "https://telegra.ph/xxx").await.unwrap();
/// # }
/// ```
pub async fn download<P, S>(dir: P, url: S) -> BoxResult<()>
where
    P: AsRef<str>,
    S: AsRef<str>,
{
    let html = downloader::get_html(url).await?;
    let info = parser::parse(html)?;
    downloader::download_imgs(dir, info).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn download_test() {
        let url_str = "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93-%E7%88%B1%E8%8E%89%E5%B8%8C%E9%9B%85-33P-352MB-08-28";
        download(std::env::var("TEST_SAVE_DIR").unwrap(), url_str)
            .await
            .unwrap();
    }
}
