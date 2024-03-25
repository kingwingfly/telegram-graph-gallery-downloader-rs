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
        let url_str = "https://telegra.ph/%E7%A5%9E%E6%B2%A2%E6%B0%B8%E8%8E%89-%E6%96%B0%E5%B9%B4%E5%A5%BD%E5%91%80%E6%96%B0%E5%B9%B4%E5%A5%BD%E5%91%80-03-22";
        download(std::env::var("TEST_SAVE_DIR").unwrap(), url_str)
            .await
            .unwrap();
    }
}
