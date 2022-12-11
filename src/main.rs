use crate::lib::{phrasematcher, spacy};
pub mod lib;
// pub mod lib;

fn main() {
    let spacy = spacy::Module::init();
    spacy.load("en_core_web_sm");
    let pangram1 = spacy::nlp("This is good.");
    let pangram2 = spacy::nlp("This is bad.");

    let sim = pangram1.similarity(pangram2);
    println!("{:?}", sim);

    phrasematcher::match_phrases();
}
