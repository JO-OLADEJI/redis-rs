use mini_redis::{client, Result};

async fn greet() {
    println!("tokio!");
}

#[tokio::main]
async fn main() -> Result<()> {
    let greeting = greet();

    print!("Hello, ");
    greeting.await;

    Ok(())
}
