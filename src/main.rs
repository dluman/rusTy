pub mod utils;

use crate::utils::{doc, phrasematcher, spacy};

use cpython::*;

fn main() {
    let spacy: spacy::Module = spacy::Module::init();

    spacy.load("en_core_web_sm");
    let pangram1: doc::Doc = spacy::nlp("This is good.");
    let pangram2: doc::Doc = spacy::nlp("This is bad.");

    let sim: PyObject = pangram1.similarity(pangram2);
    println!("{:?}", sim);

    phrasematcher::match_phrases();
}
