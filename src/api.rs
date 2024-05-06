use std::fmt::Display;

use reqwest::Result;
use serde::Deserialize;

const BASE_URL: &str = "https://trailsinthedatabase.com";
const API_URL: &str = "/api";

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u8,
    pub title_eng: String,
    pub title_jpn_roman: String,
    pub title_jpn: String,
    pub rows: Option<u32>,
}

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub game_id: u8,
    pub fname: String,
    pub rows: u32,
    pub eng_place_names: Option<Vec<String>>,
    pub jpn_place_names: Option<Vec<String>>,
    pub eng_chr_names: Vec<String>,
    pub jpn_chr_names: Vec<String>,
}

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub game_id: u8,
    pub fname: String,
    pub scene: Option<String>,
    pub row: u32,
    pub eng_chr_name: String,
    pub eng_search_text: String,
    pub jpn_chr_name: String,
    pub jpn_search_text: String,
    pub jpn_html_text: String,
    pub op_name: String,
    pub pc_icon_html: Option<String>,
    pub evo_icon_html: Option<String>,
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.eng_chr_name, self.eng_search_text)
    }
}

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub eng_chr_name: String,
    pub jpn_chr_name: String,
    pub game_id: Vec<u8>,
    pub rows: Option<u32>,
}

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stat {
    pub game: Option<Game>,
    pub rows: Option<u32>,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Sort {
    Rows,
    EngChrName,
    JpnChrName,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Rows => "rows".to_string(),
            Self::EngChrName => "engChrName".to_string(),
            Self::JpnChrName => "jpnChrName".to_string(),
        };
        write!(f, "{string}")
    }
}

fn option_to_string<T: ToString>(option: Option<T>) -> String {
    match option {
        None => "".to_string(),
        Some(o) => o.to_string(),
    }
}

fn option_vec_to_request_string(vec: Option<Vec<&str>>) -> String {
    match vec {
        None => "".to_string(),
        Some(v) => format!("&chr[]={}", v.join("&chr[]=")),
    }
}

pub async fn get_games() -> Result<Vec<Game>> {
    let url = format!("{BASE_URL}{API_URL}/game");
    let response = reqwest::get(url).await?;

    let games: Vec<Game> = response.json().await?;
    Ok(games)
}

pub async fn get_file(game_id: u8) -> Result<Vec<Metadata>> {
    let url = format!("{BASE_URL}{API_URL}/file?game_id={game_id}");
    let response = reqwest::get(url).await?;

    let meta: Vec<Metadata> = response.json().await?;
    Ok(meta)
}

pub async fn get_script_detail(game_id: u8, fname: &str) -> Result<Vec<Script>> {
    let url = format!("{BASE_URL}{API_URL}/script/detail/{game_id}/{fname}");
    let response = reqwest::get(&url).await?;

    let scripts: Vec<Script> = response.json().await?;
    Ok(scripts)
}

pub async fn get_chr(character: Option<&str>) -> Result<Vec<Character>> {
    let chr: &str = character.unwrap_or_default();
    let url = format!("{BASE_URL}{API_URL}/chr?chr={chr}");
    let response = reqwest::get(&url).await?;

    let characters: Vec<Character> = response.json().await?;
    Ok(characters)
}

pub async fn get_chr_detail(
    game_id: Option<u8>,
    asc: Option<bool>,
    sort: Option<Sort>,
    page_number: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Character>> {
    let game_id: String = option_to_string(game_id);
    let asc = asc.unwrap_or(false);
    let sort: String = sort.unwrap_or(Sort::Rows).to_string();
    let page_number: String = option_to_string(page_number);
    let page_size: String = option_to_string(page_size);

    let url = format!("{BASE_URL}{API_URL}/chr/detail?game_id={game_id}&asc={asc}&sort={sort}&page_numer={page_number}&page_size={page_size}");
    let response = reqwest::get(&url).await?;

    let characters: Vec<Character> = response.json().await?;
    Ok(characters)
}

pub async fn get_chr_detail_stat(game_id: Option<u8>) -> Result<Stat> {
    let game_id: String = option_to_string(game_id);
    let url = format!("{BASE_URL}{API_URL}/chr/detail/stat?game_id={game_id}");
    let response = reqwest::get(&url).await?;

    let stat: Stat = response.json().await?;
    Ok(stat)
}

pub async fn get_script_search_stat(
    q: Option<&str>,
    game_id: Option<u8>,
    strict_search: Option<bool>,
    chr: Option<Vec<&str>>,
) -> Result<Vec<Stat>> {
    let q: &str = q.unwrap_or_default();
    let game_id: String = option_to_string(game_id);
    let strict_search: bool = strict_search.unwrap_or(false);
    let chr: String = option_vec_to_request_string(chr);
    let url = format!(
        "{BASE_URL}{API_URL}/script/search/stat?q=\"{q}\"&game_id={game_id}&strict_search={strict_search}{chr}"
    );
    let response = reqwest::get(&url).await?;

    let stats: Vec<Stat> = response.json().await?;
    Ok(stats)
}

pub async fn get_script_search(
    q: Option<&str>,
    game_id: Option<u8>,
    strict_search: Option<bool>,
    page_number: Option<u32>,
    page_size: Option<u32>,
    chr: Option<Vec<&str>>,
) -> Result<Vec<Script>> {
    let q: &str = q.unwrap_or_default();
    let game_id: String = option_to_string(game_id);
    let strict_search: bool = strict_search.unwrap_or(false);
    let page_size: String = option_to_string(page_size);
    let page_number: String = option_to_string(page_number);
    let chr: String = option_vec_to_request_string(chr);

    let url = format!("{BASE_URL}{API_URL}/script/search?q=\"{q}\"&game_id={game_id}&strict_search={strict_search}&page_number={page_number}&page_size={page_size}{chr}");
    let response = reqwest::get(&url).await?;

    let scripts: Vec<Script> = response.json().await?;
    Ok(scripts)
}
