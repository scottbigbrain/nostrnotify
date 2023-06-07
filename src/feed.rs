async fn check_feed(feed_url: &str, last_feed_len: usize, client: &Client) -> Result<usize, Box<dyn Error>> {
    let feed = get_feed(feed_url).await?;
    if feed.items.len() > last_feed_len {
        let message = format!("Episode #{number} published", number = feed.items.len());
        client.publish_text_note(message, &[]).await?;
        println!("Increased");
    } else {
        println!("No change");
    }
    
    Ok(feed.items.len())
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}