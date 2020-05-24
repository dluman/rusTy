#[path = "lib/spacy.rs"] mod spacy;

use cpython::*;

fn main() {
  // Need better syntax
  let spacy = spacy::module::load("en_core_web_lg");
  let nlp = spacy.nlp("With tenure, Suzie’d have all the more leisure for yachting, but her publications are no good.");
  let doc = spacy::module::map(nlp);
  println!("{}",doc.fields["tokens"]);
}
