use clap::Parser;

/// A downloader for telegram graph gallery
#[derive(Parser)]
struct Args {
    /// Directory to save the imges
    #[arg(short, long)]
    dir: String,
    /// URL of telegram graph gallery
    urls: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    for url in args.urls {
        telegram_graph_downloader::download(args.dir.clone(), url)
            .await
            .unwrap();
    }
}
