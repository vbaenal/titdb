# TITDB - Trails in the Database

titdb is a library that calls [Trails in the Database](https://trailsinthedatabase.com/)'s API. It also includes a script randomizer as an optional feature.

## Randomizer

```toml
[dependencies]
titdb = { version = "0.1", features = ["rand"] }
```

## Examples

```rust
use titdb::rand::SceneRandomizer;

#[tokio::main]
async fn main() -> Result<()> {
    // Get 500 characters of Trails from Zero
    let characters = get_chr_detail(Some(4), None, None, None, Some(500))
        .await
        .unwrap();
    // Randomize any script from any game with characters from the previous list
    let random_scene: SceneRandomizer = SceneRandomizer::new(Some(characters), None, None);
    let scene = random_scene.randomize().await;
    println!(scene.unwrap());
    Ok(())
}
```

