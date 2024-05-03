use titdb::api::*;

#[tokio::test]
async fn get_games_test() {
    let test: Result<Vec<Game>, reqwest::Error> = get_games().await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_file_test() {
    let test: Result<Vec<Metadata>, reqwest::Error> = get_file(1).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_script_detail_test() {
    let test: Result<Vec<Script>, reqwest::Error> = get_script_detail(1, "c0100").await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_test_none() {
    let test: Result<Vec<Character>, reqwest::Error> = get_chr(None).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_test_data() {
    let test: Result<Vec<Character>, reqwest::Error> = get_chr(Some("Estelle")).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_detail_test_none() {
    let test: Result<Vec<Character>, reqwest::Error> =
        get_chr_detail(None, None, None, None, None).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_detail_test_data() {
    let test: Result<Vec<Character>, reqwest::Error> = get_chr_detail(
        Some(1),
        Some(true),
        Some(Sort::EngChrName),
        Some(3),
        Some(4),
    )
    .await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_detail_stat_test_none() {
    let test: Result<Stat, reqwest::Error> = get_chr_detail_stat(None).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_chr_detail_stat_test_data() {
    let test: Result<Stat, reqwest::Error> = get_chr_detail_stat(Some(1)).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_script_search_stat_test_none() {
    let test: Result<Vec<Stat>, reqwest::Error> = get_script_search_stat(None, None, None, None).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_script_search_stat_test_data() {
    let test: Result<Vec<Stat>, reqwest::Error> =
        get_script_search_stat(Some("Why is my present a boy?"), Some(1), Some(false), Some(vec!["Estelle"])).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_script_search_test_none() {
    let test: Result<Vec<Script>, reqwest::Error> =
        get_script_search(None, None, None, None, None, None).await;
    assert!(test.is_ok())
}

#[tokio::test]
async fn get_script_search_test_data() {
    let test: Result<Vec<Script>, reqwest::Error> = get_script_search(
        Some("Why is my present a boy?"),
        Some(1),
        Some(false),
        Some(1),
        Some(1),
        Some(vec!["Estelle"])
    )
    .await;
    assert!(test.is_ok())
}
