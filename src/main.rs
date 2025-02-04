use rust_news::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}