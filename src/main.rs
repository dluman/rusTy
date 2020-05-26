#[path = "lib/spacy.rs"] mod spacy;

use cpython::*;

fn main() {
  let spacy = spacy::Module::init();
  let model = spacy.load("en_core_web_lg");
  let pangram1 = spacy::nlp("With tenure, Suzie’d have all the more leisure for yachting, but her publications are no good.");
  let pangram2 = spacy::nlp("Amazingly few discotheques provide jukeboxes.");
  println!("{:?}",pangram1);
  println!("{:?}",pangram2);
}
