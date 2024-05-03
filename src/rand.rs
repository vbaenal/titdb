use crate::api::*;
use rand::seq::{IteratorRandom, SliceRandom};
use reqwest::Result;

#[derive(Clone, Default)]
pub struct SceneRandomizer {
    game_id: Option<u8>,
    fname: Option<String>,
    characters: Option<Vec<Character>>,
}

impl SceneRandomizer {
    pub fn new(
        characters: Option<Vec<Character>>,
        game_id: Option<u8>,
        fname: Option<String>,
    ) -> Self {
        Self {
            characters,
            game_id,
            fname,
        }
    }

    pub async fn randomize(self) -> Result<Vec<Script>> {
        let gid = match self.game_id {
            Some(g) => g,
            None => {
                get_games()
                    .await
                    .unwrap()
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .id
            }
        };
        let meta: Metadata = match self.fname {
            Some(f) => {
                let file = get_file(gid).await.unwrap();
                file.iter().find(|&fi| fi.fname == f).unwrap().clone()
            }
            None => {
                let file = get_file(gid).await.unwrap();
                file.iter()
                    .filter(|&fi| !fi.eng_chr_names.is_empty() || !fi.jpn_chr_names.is_empty())
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone()
            }
        };
        let chars: Vec<Character> = match self.characters {
            Some(cs) => cs,
            None => get_chr_detail(Some(gid), None, None, None, Some(500))
                .await
                .unwrap(),
        };

        let random_scene_characters: &mut Vec<(String, String)> = &mut vec![];
        let original_scene_characters = if !meta.eng_chr_names.is_empty() {
            meta.eng_chr_names
        } else {
            meta.jpn_chr_names
        };
        original_scene_characters.iter().for_each(|_| {
            let character = chars.choose(&mut rand::thread_rng()).unwrap();
            random_scene_characters.push((
                character.eng_chr_name.clone(),
                character.jpn_chr_name.clone(),
            ));
        });

        let scene: &mut Vec<Script> = &mut get_script_detail(gid, &meta.fname).await.unwrap();
        scene.iter_mut().for_each(|script| {
            let chr_pos = original_scene_characters
                .iter()
                .position(|chr| chr == &script.eng_chr_name || chr == &script.jpn_chr_name)
                .unwrap();
            let rsc = random_scene_characters.get(chr_pos).unwrap();
            script.eng_chr_name = rsc.0.to_string();
            script.jpn_chr_name = rsc.1.to_string();
        });
        Ok(scene.clone())
    }
}
