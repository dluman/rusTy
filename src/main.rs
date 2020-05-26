#[path = "lib/spacy.rs"] mod spacy;

use cpython::*;

fn main() {
  let spacy = spacy::Module::init();
  let model = spacy.load("en_core_web_lg");
  let nlp = model.nlp("With tenure, Suzie’d have all the more leisure for yachting, but her publications are no good.");
  println!("{:?}",nlp);
}
