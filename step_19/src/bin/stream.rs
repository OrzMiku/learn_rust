use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec![
        "Hello",
        "World",
        "from",
        "tokio_stream",
        "with",
        "async",
    ]).map(|s| s.to_uppercase());
    
    while let Some(item) = stream.next().await {
        print!("{} ", item);
    }

    println!();
}
