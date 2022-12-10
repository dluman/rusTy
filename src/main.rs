#[path = "lib/phrasematcher.rs"]
mod phrasematcher;
#[path = "lib/spacy.rs"]
mod spacy;

#[path = "lib/doc.rs"]
mod doc;

use crate::spacy::doc::Callable;

fn main() {
    let spacy = spacy::Module::init();
    spacy.load("en_core_web_sm");
    let pangram1 = spacy::nlp("This is good.");
    let pangram2 = spacy::nlp("This is bad.");
    pangram1
        .call("similarity")
        .args(pangram2)
        .kwargs(None)
        .invoke();

    phrasematcher::match_phrases();
}
