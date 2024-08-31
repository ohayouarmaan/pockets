mod pocket_state;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut pocket = pocket_state::Pocket::default();
    pocket.listen("127.0.0.1", 8000).await;
    dbg!(&pocket);
    Ok(())
}
