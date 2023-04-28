use reqwest::Result;
use rss::Channel;
use std::error::Error;

async fn example_feed() -> std::result::Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.nasa.gov/rss/dyn/breaking_news.rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let c = example_feed().await?;
    // store the feed into a database
    dbg!(&c);
    for item in c.items() {
        dbg!(item.guid()); // guid is supposed to be unique for an entry to determine if it has
                           // been read or not
    }
    Ok(())
}
