use utils::download_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = download_input(1).await?;
    println!("Day 1 Input: \n\n{}", &puzzle_input);
    Ok(())
}
