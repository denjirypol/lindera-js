use lazy_static::lazy_static;
use lindera::tokenizer::{Token, Tokenizer};
use serde::Serialize;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref TOKENIZER: Mutex<Tokenizer> = Mutex::new(Tokenizer::new().unwrap());
}

#[derive(Serialize)]
pub struct KuromojiJSFormatToken<'a> {
    word_id: Option<u32>,
    word_type: &'a str,
    word_position: u32,
    surface_form: &'a str,
    pos: &'a str,
    pos_detail_1: &'a str,
    pos_detail_2: &'a str,
    pos_detail_3: &'a str,
    conjugated_type: &'a str,
    conjugated_form: &'a str,
    basic_form: &'a str,
    reading: &'a str,
    pronunciation: &'a str,
}

fn detail_to_kuromoji_js_format<'a>(position: u32, token: &'a Token) -> KuromojiJSFormatToken<'a> {
    if token.detail[0] != "UNK" {
        KuromojiJSFormatToken {
            word_id: None,
            word_type: { "KNOWN" },
            word_position: position,
            surface_form: token.text,
            pos: token.detail[0].as_str(),
            pos_detail_1: token.detail[1].as_str(),
            pos_detail_2: token.detail[2].as_str(),
            pos_detail_3: token.detail[3].as_str(),
            conjugated_type: token.detail[4].as_str(),
            conjugated_form: token.detail[5].as_str(),
            basic_form: token.detail[6].as_str(),
            reading: token.detail[7].as_str(),
            pronunciation: token.detail[8].as_str(),
        }
    } else {
        KuromojiJSFormatToken {
            word_id: None,
            word_type: "UNKNOWN",
            word_position: position,
            surface_form: token.text,
            pos: token.detail[0].as_str(),
            pos_detail_1: "＊",
            pos_detail_2: "＊",
            pos_detail_3: "＊",
            conjugated_type: "＊",
            conjugated_form: "＊",
            basic_form: "＊",
            reading: "＊",
            pronunciation: "＊",
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_KUROMOJI_JS_TOKEN: &'static str = r#"
interface KuromojiJSToken {
    word_id: number | null,
    word_type: string,
    word_position: number,
    surface_form: string,
    pos: string,
    pos_detail_1: string,
    pos_detail_2: string,
    pos_detail_3: string,
    conjugated_type: string,
    conjugated_form: string,
    basic_form: string,
    reading: string,
    pronunciation: string,
}

export function tokenize(input_text: string): KuromojiJSToken[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn tokenize(input_text: &str) -> JsValue {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let tokens = TOKENIZER.lock().unwrap().tokenize(input_text).unwrap();

    JsValue::from_serde(
        &tokens
            .iter()
            .enumerate()
            .map(|(i, x)| detail_to_kuromoji_js_format(i as u32, &x))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_tokenize() {
        let t = tokenize("関西国際空港限定トートバッグ");
        let tokens: Vec<Value> = t.into_serde().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].get("surface_form").unwrap(), "関西国際空港");
        assert_eq!(tokens[1].get("surface_form").unwrap(), "限定");
        assert_eq!(tokens[2].get("surface_form").unwrap(), "トートバッグ");
    }
}
