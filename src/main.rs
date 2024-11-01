use clap::Parser;
use std::error::Error;

mod crawler;
mod page;
mod analyzer;
mod storage;

use crawler::Crawler;
use analyzer::ContentAnalyzer;

#[derive(Parser)]
#[command(name = "web-crawler")]
#[command(about = "A web crawler that searches for specific content")]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value = "5")]
    depth: usize,

    #[arg(short, long, default_value = "1000")]
    timeout: u64,

    #[arg(short, long, default_value = "results")]
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    let mut crawler = Crawler::new(&args.url, args.depth, args.timeout)?;
    let pages = crawler.run().await?;

    let analyzer = ContentAnalyzer::new();
    let storage = storage::FileStorage::new(&args.output_dir)?;

    for page in pages {
        let analysis = analyzer.analyze(&page);
        storage.save(&page.url, &analysis)?;
        
        println!("Analyzed: {}", page.url);
        println!("Found patterns: {}", analysis.patterns_found());
        println!("Saved to: {}\n", args.output_dir);
    }

    Ok(())
}
