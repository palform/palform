use anyhow::anyhow;
use rand::{rngs, seq::SliceRandom, SeedableRng};

const PASSPHRASE_WORD_COUNT: usize = 16;

#[cfg(feature = "frontend-js")]
struct WordList {
    list: Vec<String>,
}

#[cfg(feature = "frontend-js")]
impl WordList {
    fn load() -> Result<Self, anyhow::Error> {
        let word_list_raw = include_bytes!("./word_list.tsv") as &[u8];
        let mut r = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(word_list_raw);
        let mut words = Vec::<String>::new();
        for row in r.records() {
            let row = row.map_err(|e| anyhow!(e.to_string()))?;
            let word = row.get(1);
            if let Some(word) = word {
                words.push(word.to_owned());
            }
        }

        Ok(Self { list: words })
    }

    fn choose_random(&self) -> Option<String> {
        self.list
            .choose(&mut rngs::StdRng::from_entropy())
            .map(|e| e.to_owned())
    }

    pub fn generate_passphrase() -> Result<Vec<String>, anyhow::Error> {
        let wl = Self::load()?;
        let mut words = Vec::<String>::new();
        for _ in 0..PASSPHRASE_WORD_COUNT {
            words.push(
                wl.choose_random()
                    .ok_or(anyhow!("choose random returned None"))?,
            );
        }
        Ok(words)
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn generate_passphrase_js() -> Result<Vec<String>, wasm_bindgen::JsValue> {
    WordList::generate_passphrase().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
