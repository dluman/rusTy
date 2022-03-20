#[path = "lib/spacy.rs"] mod spacy;

use crate::spacy::doc::Callable;

fn main() {
  let spacy = spacy::Module::init();
  spacy.load("en_core_web_lg");
  let pangram1 = spacy::nlp("With tenure, Suzie’d have all the more leisure for yachting, but her publications are no good.");
  let pangram2 = spacy::nlp("Amazingly few discotheques provide jukeboxes.");
  pangram1
    .call("similarity")
    .args(pangram2)
    .kwargs(None)
    .invoke();
}
