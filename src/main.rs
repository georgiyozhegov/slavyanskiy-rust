mod translation;
mod alphabet;
#[macro_use]
mod macros;
mod tokenizer;

use translation::{TRANSLATION, KEYWORDS};
use tokenizer::tokenize;

use std::process;
use std::fs;
use std::env;


const BINARY_PATH: &str = "./славная_летопись";
const SAVE_PATH: &str = "./славная_летопись.rs";


fn get_path() -> String
{ 
    if let Some(path) = second_argument!(env::args()) {
        return path.to_string();
    }
    error!("Путь твой к летописи напиши: 'славянский <путь-твой>.слв'.");
}


fn read_source(path: String) -> String
{
    if let Ok(source) = fs::read_to_string(path) {
        if source.trim().is_empty() {
            error!("Летопись пустая, да скучная.");
        }
        return source;
    }
    error!("Летопись окаянная. Быть может ошибся ты?");
}


fn translate(source: String) -> String 
{
    let tokens = tokenize(source);
    let mut translated = Vec::new();
    for token in tokens.iter() {
        if KEYWORDS.contains(&token.as_str()) {
            let index = KEYWORDS
                .iter()
                .position(|t| t == &token)
                .unwrap();
            translated.push(TRANSLATION[index].1);
        } else {
            translated.push(&token);
        }
    }
    translated.join("")
}


fn save(translated: String) 
{
    if fs::write(SAVE_PATH, translated).is_err() {
        error!("Не сохраняется летопись, хоть и славная она.");
    }
}


fn run() 
{
    if let Err(err) = process::Command::new("rustc")
        .arg("-Awarnings")  // disable warnings
        .arg("-O")  // optimization level 2
        .arg(SAVE_PATH)
        .status() {
        error!(
            format!(
                "Батюшке летопись не понраву, хоть и славная она. Вот что молвит: {err}"
            )
        );
    }
    process::Command::new(BINARY_PATH).status().unwrap();
}


fn clear_files() 
{
    process::Command::new("rm")
        .arg(SAVE_PATH)
        .arg(BINARY_PATH)
        .status()
        .unwrap();
}


fn main() {
    let path = get_path();
    let source = read_source(path);
    let translated = translate(source);
    save(translated);
    run();
    clear_files();
}
