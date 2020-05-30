![Build status](https://travis-ci.com/dluman/rusTy.svg?branch=master)

# rusTy

Rust bindings for the [spaCy](https://spacy.io) Python NLP library. It's a work in progress. I'm at that part in another project where it's just easier to take a detour and write some bindings.

## Example

The following performs sentence similarity.

```rust
let spacy = spacy::Module::init();
  spacy.load("en_core_web_lg");
  let pangram1 = spacy::nlp("With tenure, Suzie’d have all the more leisure for yachting, but her publications are no good.");
  let pangram2 = spacy::nlp("Amazingly few discotheques provide jukeboxes.");
  pangram1
    .call("similarity")
    .args(pangram2)
    .kwargs("")
    .invoke();

```
