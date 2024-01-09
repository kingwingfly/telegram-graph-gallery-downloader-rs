use crate::{parser::Info, BoxResult};
use std::{io::Write, path::PathBuf};

#[cfg(feature = "indicatif")]
use indicatif::ProgressBar;

pub(crate) async fn get_html<U>(url: U) -> BoxResult<String>
where
    U: AsRef<str>,
{
    let url = reqwest::Url::parse(url.as_ref())?;
    let client = reqwest::get(url).await?;
    Ok(client.text().await?)
}

pub(crate) async fn download_imgs<P>(dir: P, info: Info) -> BoxResult<()>
where
    P: AsRef<str>,
{
    let suffix = "https://telegra.ph";
    let Info { title, img_urls } = info;
    let title = title.unwrap_or("unknown".to_owned());
    let dir = PathBuf::from(dir.as_ref()).join(title.clone());
    std::fs::create_dir_all(&dir)?;
    let client = reqwest::Client::new();

    #[cfg(feature = "indicatif")]
    let pb = ProgressBar::new(img_urls.len() as u64);

    for (i, url) in img_urls
        .into_iter()
        .filter_map(|url| url.map(|url| format!("{suffix}{url}")))
        .enumerate()
    {
        let mut resp = client.get(&url).send().await?;
        let path = dir.join(format!("{}-{}.jpg", title, i));
        let mut file = std::fs::File::create(path)?;
        while let Some(chunk) = resp.chunk().await? {
            file.write_all(&chunk)?;
        }
        #[cfg(feature = "indicatif")]
        pb.inc(1);
    }
    #[cfg(feature = "indicatif")]
    pb.finish();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_html_test() {
        let url_str = "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93-%E7%88%B1%E8%8E%89%E5%B8%8C%E9%9B%85-33P-352MB-08-28";
        let _ = get_html(url_str).await;
    }

    #[tokio::test]
    async fn parse_test() {
        let url_str = "https://telegra.ph/Asagiriai%E6%84%9B%E3%81%A1%E3%82%83%E3%82%93-%E7%88%B1%E8%8E%89%E5%B8%8C%E9%9B%85-33P-352MB-08-28";
        let html = get_html(url_str).await.unwrap();
        let info = crate::parser::parse(html).unwrap();
        download_imgs(std::env::var("TEST_SAVE_DIR").unwrap(), info)
            .await
            .unwrap();
    }
}
