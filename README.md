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
rusty = "0.5"
```

## Quick Start

```rust
use rusty::Language;

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
| `char_span(start, end, label, kb_id, alignment_mode)` | Create `Span` from character offsets |
| `vector()` | Document vector (`Vec<f32>`) |
| `vector_norm()` | L2 norm of the document vector |
| `has_vector()` | Check if vector exists |
| `similarity(other)` | Cosine similarity with another `Doc` |
| `vocab()` | Access the document's `Vocab` |
| `cats()` | Text categorization scores (`HashMap<String, f64>`) |
| `has_annotation(attr, require_complete)` | Check if annotation is present |
| `count_by(attr_id)` | Attribute frequency counts |
| `retokenize()` | Returns a `RetokenizerGuard` for merge/split |
| `spans()` | Access named span groups (`SpanGroups`) |
| `to_json()` | Export as JSON (`serde_json::Value`) |
| `to_bytes()` | Serialize to bytes |
| `from_bytes(lang, bytes)` | Deserialize from bytes |

### `Token`

Represents a single token.

**Text & Morphology:** `text()`, `orth_()`, `lemma_()`, `norm_()`, `lower_()`, `shape_()`, `prefix_()`, `suffix_()`, `morph_()`, `whitespace_()`

**POS & Dependencies:** `pos_()`, `tag_()`, `dep_()`, `head()`, `children()`, `lefts()`, `rights()`, `ancestors()`, `subtree()`, `nbor(offset)`

**Entities:** `ent_type_()`, `ent_iob_()`, `ent_kb_id_()`, `ent_id_()`

**Lexeme:** `rank()`, `prob()`, `cluster()`

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
| `label_()` | Entity/predicate label (string) |
| `label()` | Entity/predicate label (int hash) |
| `kb_id_()` | Knowledge base ID |
| `tokens()` | Tokens in the span |
| `root()` | Root token of the span |
| `sent()` | Containing sentence |
| `doc()` | Parent document |
| `vector()` | Span vector |
| `vector_norm()` | L2 norm of the span vector |
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

### `Matcher`

spaCy's rule-based `Matcher` for finding token sequences.

```rust
use rusty::{Matcher, PatternValue, TokenPattern};

let vocab = doc.vocab()?;
let matcher = Matcher::new(&vocab, false)?;
matcher.add("GREETING", vec![
    TokenPattern::new().orth("Hello").is_punct(false),
])?;
let matches = matcher.call(&doc)?;
for m in &matches {
    println!("match {:?} at {}..{}", m.id, m.start, m.end);
}
```

| Method | Description |
|--------|-------------|
| `new(vocab, validate)` | Create a `Matcher` |
| `add(name, patterns)` | Add a pattern (one `Vec<TokenPattern>`) |
| `add_raw(name, json)` | Add a raw JSON pattern string |
| `call(doc)` | Find matches, returns `Vec<Match>` |
| `call_as_spans(doc)` | Find matches, returns `Vec<Span>` |
| `len()` | Number of rules |
| `is_empty()` | Whether no rules are registered |
| `contains(name)` | Check if rule exists |
| `remove(name)` | Remove a rule |

### `PhraseMatcher`

Efficient phrase-list matching using `Doc` objects as patterns.

```rust
use rusty::PhraseMatcher;

let vocab = doc.vocab()?;
let pattern_doc = nlp.nlp("Hello")?;
let matcher = PhraseMatcher::new(&vocab, None, false)?;
matcher.add("GREETING", &[pattern_doc])?;
let matches = matcher.call(&doc)?;
```

| Method | Description |
|--------|-------------|
| `new(vocab, attr, validate)` | Create a `PhraseMatcher` (`attr`: e.g. `"ORTH"`, `"LOWER"`) |
| `add(name, docs)` | Add `Doc` objects as patterns |
| `call(doc)` / `call_as_spans(doc)` | Find matches |
| `len()` / `is_empty()` / `contains(name)` / `remove(name)` | Same as `Matcher` |

### `RetokenizerGuard`

RAII guard around `Doc.retokenize()`. On drop, all pending changes are applied.

```rust
let mut span = doc.char_span(0, 8, None, None, None)?.unwrap();
{
    let mut retokenizer = doc.retokenize()?;
    retokenizer.merge(&span, None)?;
} // merged token is persisted
```

| Method | Description |
|--------|-------------|
| `merge(span, attrs_json)` | Merge tokens in `span` into one token |
| `split(token, orths, heads, attrs_json)` | Split `token` into subtokens |

### `SpanGroups` & `SpanGroup`

Named groups of potentially overlapping spans.

```rust
let spans = doc.spans()?;
spans.set("ents", &doc.ents()?)?;
let group = spans.get("ents")?.unwrap();
assert_eq!(group.len()?, doc.ents()?.len());
```

**SpanGroups:**
| Method | Description |
|--------|-------------|
| `get(name)` | Get a `SpanGroup` by name |
| `set(name, spans)` | Assign a list of `Span`s to a name |
| `names()` | List all group names |
| `has(name)` | Check if a group exists |

**SpanGroup:**
| Method | Description |
|--------|-------------|
| `len()` / `is_empty()` | Number of spans |
| `spans()` | Get all spans |
| `get(index)` / `set(index, span)` / `remove(index)` | Index access |
| `append(span)` / `extend(spans)` | Add spans |
| `has_overlap()` | Check for overlapping spans |
| `copy()` | Return a copy |

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
let doc2 = rusty::Doc::from_bytes(&nlp, &bytes)?;
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
