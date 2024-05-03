use titdb::api::*;
use titdb::rand::*;

#[tokio::test]
async fn randomize_scene_test_none() {
    let random_scene: SceneRandomizer = SceneRandomizer::default();
    let scene = random_scene.randomize().await;
    assert!(scene.is_ok());
}

#[tokio::test]
async fn randomize_scene_test_characters() {
    let characters = get_chr_detail(Some(4), None, None, None, Some(500))
        .await
        .unwrap();
    let random_scene: SceneRandomizer = SceneRandomizer::new(Some(characters), None, None);
    let scene = random_scene.randomize().await;
    assert!(scene.is_ok());
}

#[tokio::test]
async fn randomize_scene_test_game() {
    let random_scene: SceneRandomizer = SceneRandomizer::new(None, Some(4), None);
    let scene = random_scene.randomize().await;
    assert!(scene.is_ok());
}
