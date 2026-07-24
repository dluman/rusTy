# rusTy

Rust bindings for the [spaCy](https://spacy.io) Python NLP library. This crate provides idiomatic, strongly-typed Rust wrappers around spaCy's core data structures and functionality, built on [pyo3](https://github.com/PyO3/pyo3) and [rust-numpy](https://github.com/PyO3/rust-numpy) for zero-copy vector access.

## Requirements

- Python 3.8+
- spaCy installed in your Python environment
- A spaCy model downloaded (e.g., `en_core_web_sm`)

```bash
pip install spacy
python -m spacy download en_core_web_sm
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spacy_rs = "0.1"
```

## Quick Start

```rust
use spacy_rs::Language;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a spaCy model
    let nlp = Language::load("en_core_web_sm")?;

    // Process text
    let doc = nlp.nlp("Apple is looking at buying U.K. startup for $1 billion.")?;

    // Iterate tokens
    for token in doc.tokens()? {
        println!(
            "{:12} | lemma: {:10} | pos: {:6} | dep: {:10}",
            token.text()?,
            token.lemma_()?,
            token.pos_()?,
            token.dep_()?
        );
    }

    // Named entities
    for ent in doc.ents()? {
        println!("{:20} | label: {:10}", ent.text()?, ent.label_()?);
    }

    // Sentences
    for sent in doc.sents()? {
        println!("Sentence: {}", sent.text()?);
    }

    // Noun chunks
    for chunk in doc.noun_chunks()? {
        println!("Chunk: {}", chunk.text()?);
    }

    Ok(())
}
```

## API Overview

### `Language`

Represents a loaded spaCy language model.

| Method | Description |
|--------|-------------|
| `load(model)` | Load a spaCy model by name |
| `nlp(text)` | Process text into a `Doc` |
| `pipe(texts)` | Batch process multiple texts |
| `vocab()` | Access the model's `Vocab` |
| `component_names()` | List pipeline component names |
| `add_pipe(name, config, last)` | Add a pipeline component |
| `remove_pipe(name)` | Remove a pipeline component |
| `disable_pipes(names)` | Temporarily disable pipes (returns guard) |
| `to_disk(path)` | Save model to disk |
| `from_disk(path)` | Load model from disk |

### `Doc`

Represents a processed document.

| Method | Description |
|--------|-------------|
| `text()` | Full document text |
| `len()` | Number of tokens |
| `token(i)` | Get token at index |
| `tokens()` | Get all tokens |
| `ents()` | Named entities (as `Span`s) |
| `sents()` | Sentences (as `Span`s) |
| `noun_chunks()` | Noun chunks (as `Span`s) |
| `vector()` | Document vector (`Vec<f32>`) |
| `has_vector()` | Check if vector exists |
| `similarity(other)` | Cosine similarity with another `Doc` |
| `to_json()` | Export as JSON (`serde_json::Value`) |
| `to_bytes()` | Serialize to bytes |
| `from_bytes(lang, bytes)` | Deserialize from bytes |

### `Token`

Represents a single token.

**Text & Morphology:** `text()`, `orth_()`, `lemma_()`, `norm_()`, `lower_()`, `shape_()`, `prefix_()`, `suffix_()`

**POS & Dependencies:** `pos_()`, `tag_()`, `dep_()`, `head()`, `children()`, `lefts()`, `rights()`, `ancestors()`, `subtree()`, `nbor(offset)`

**Entities:** `ent_type_()`, `ent_iob_()`, `ent_kb_id_()`

**Flags:** `is_alpha()`, `is_ascii()`, `is_digit()`, `is_lower()`, `is_upper()`, `is_title()`, `is_punct()`, `is_space()`, `is_stop()`, `like_num()`, `like_email()`, `like_url()`

**Vectors:** `vector()`, `has_vector()`, `similarity(other)`

**Context:** `idx()`, `sent()`, `doc()`

### `Span`

Represents a contiguous slice of tokens.

| Method | Description |
|--------|-------------|
| `text()` | Span text |
| `start()`, `end()` | Token indices |
| `start_char()`, `end_char()` | Character indices |
| `label_()` | Entity/predicate label |
| `kb_id_()` | Knowledge base ID |
| `tokens()` | Tokens in the span |
| `root()` | Root token of the span |
| `sent()` | Containing sentence |
| `doc()` | Parent document |
| `vector()` | Span vector |
| `has_vector()` | Check if vector exists |
| `similarity(other)` | Similarity with another `Span` |
| `as_doc()` | Convert span to a standalone `Doc` |

### `Vocab` & `StringStore`

| Method | Description |
|--------|-------------|
| `vocab.strings()` | Access the string store |
| `strings.add(string)` | Add a string, get its hash |
| `strings.get_hash(string)` | Get hash for a string |
| `strings.get_string(hash)` | Look up string by hash |

## Similarity Example

```rust
let nlp = Language::load("en_core_web_lg")?;
let doc1 = nlp.nlp("I love apples.")?;
let doc2 = nlp.nlp("I enjoy fruit.")?;
let score = doc1.similarity(&doc2)?;
println!("Similarity: {}", score);
```

## Batch Processing

```rust
let nlp = Language::load("en_core_web_sm")?;
let texts = vec!["Hello world.", "This is a test."];
let docs = nlp.pipe(&texts)?;
for doc in &docs {
    println!("{}", doc.text()?);
}
```

## Serialization

```rust
let nlp = Language::load("en_core_web_sm")?;
let doc = nlp.nlp("Hello, world!")?;

// To bytes
let bytes = doc.to_bytes()?;

// From bytes
let doc2 = spacy_rs::Doc::from_bytes(&nlp, &bytes)?;
assert_eq!(doc.text()?, doc2.text()?);

// To JSON
let json = doc.to_json()?;
```

## Pipeline Control

```rust
let nlp = Language::load("en_core_web_sm")?;

// Temporarily disable parser and NER
{
    let _guard = nlp.disable_pipes(&["parser", "ner"])?;
    let doc = nlp.nlp("Just tokenization.")?;
} // Pipes automatically re-enabled when guard drops
```

## Error Handling

All operations return `Result<T, SpaCyError>`. The error enum covers:

- `Python(String)` — Python runtime errors
- `ModelNotLoaded` — Missing model
- `InvalidIndex(usize)` — Out-of-bounds token access
- `Io(std::io::Error)` — Disk I/O errors
- `Json(serde_json::Error)` — JSON serialization errors
- `Utf8(std::str::Utf8Error)` — Encoding errors
- `Numpy(String)` — Vector conversion errors

## License

See `LICENSE` file.
