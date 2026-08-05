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
rusty = "0.6"
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
| `pipe_names()` | List active pipeline component names |
| `pipeline()` | Active pipeline as `(name, component)` tuples |
| `disabled()` | Names of currently disabled components |
| `add_pipe(name, config, last)` | Add a pipeline component |
| `remove_pipe(name)` | Remove a pipeline component |
| `replace_pipe(name, factory, config)` | Replace a pipeline component |
| `rename_pipe(old, new)` | Rename a pipeline component |
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
| `from_json(lang, json)` | Import from JSON |
| `to_bytes()` | Serialize to bytes |
| `from_bytes(lang, bytes)` | Deserialize from bytes |
| `to_disk(path)` | Save `Doc` to disk |
| `from_disk(lang, path)` | Load `Doc` from disk |

### `Token`

Represents a single token.

**Text & Morphology:** `text()`, `orth_()`, `lemma_()`, `norm_()`, `lower_()`, `shape_()`, `prefix_()`, `suffix_()`, `morph_()`, `whitespace_()`

**POS & Dependencies:** `pos_()`, `tag_()`, `dep_()`, `head()`, `children()`, `lefts()`, `rights()`, `ancestors()`, `subtree()`, `nbor(offset)`

**Entities:** `ent_type_()`, `ent_iob_()`, `ent_kb_id_()`, `ent_id_()`

**Lexeme:** `lexeme()`, `rank()`, `prob()`, `cluster()`

**Flags:** `is_alpha()`, `is_ascii()`, `is_digit()`, `is_lower()`, `is_upper()`, `is_title()`, `is_punct()`, `is_space()`, `is_stop()`, `like_num()`, `like_email()`, `like_url()`

**Vectors:** `vector()`, `vector_norm()`, `has_vector()`, `similarity(other)`

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
| `vocab.vectors()` | Access the vocab's `Vectors` |
| `strings.add(string)` | Add a string, get its hash |
| `strings.get_hash(string)` | Get hash for a string |
| `strings.get_string(hash)` | Look up string by hash |

### `Vectors`

Word vector table backed by numpy. Keys are 64-bit hashes from the vocab's string store.

```rust
use rusty::{Language, Vectors};

let nlp = Language::load("en_core_web_sm")?;
let vocab = nlp.vocab()?;
let vectors = Vectors::new_table(&vocab, Some((10, 4)))?;
vectors.add("hello", Some(&[1.0, 2.0, 3.0, 4.0]), None)?;
let vec = vectors.get("hello")?;
```

| Method | Description |
|--------|-------------|
| `new_table(vocab, shape)` | Create a new vector table (`shape`: `(rows, dims)`) |
| `add(key, vector, row)` | Add a vector; returns assigned row |
| `get(key)` | Retrieve vector by string key |
| `find(key, row)` | Look up row by key or key by row |
| `contains(hash)` | Check if a key hash exists |
| `keys()` | All key hashes in the table |
| `len()` | Number of rows |
| `shape()` | `(rows, dims)` |
| `most_similar(queries, n, batch_size)` | Find `n` most similar vectors |
| `to_bytes()` / `from_bytes(vocab, bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(vocab, path)` | Disk serialization |

### `Lexeme`

Represents a word type independent of context.

```rust
let token = doc.token(0)?;
let lexeme = token.lexeme()?;
```

| Method | Description |
|--------|-------------|
| `orth_()` | Canonical string form |
| `lower_()` | Lowercased form |
| `norm_()` | Normalized form |
| `shape_()` | Word shape |
| `prefix_()` / `suffix_()` | Prefix / suffix |
| `lang_()` | Language code |
| `is_alpha()` / `is_ascii()` / `is_digit()` / `is_lower()` / `is_upper()` / `is_title()` / `is_punct()` / `is_space()` / `is_stop()` | Boolean flags |
| `like_num()` / `like_email()` / `like_url()` | Pattern matches |
| `prob()` | Log probability |
| `cluster()` | Brown cluster ID |
| `rank()` | Frequency rank |
| `vector()` / `vector_norm()` / `has_vector()` | Vector access |

### `SpanRuler`

Pipeline component for rule-based span recognition using `SpanPattern`.

```rust
use rusty::{SpanRuler, SpanPattern};

let ruler = SpanRuler::new(&nlp, None, false, false, None)?;
let patterns = vec![
    SpanPattern::phrase("ORG", "Apple Inc."),
    SpanPattern::tokens("ORG", vec![TokenPattern::new().orth("Apple")]),
];
ruler.add_patterns(&patterns)?;
let doc = ruler.call(&nlp.nlp("Apple Inc. is hiring.")?)?;
```

| Method | Description |
|--------|-------------|
| `new(language, phrase_matcher_attr, validate, overwrite_ents, patterns)` | Create a `SpanRuler` |
| `add_patterns(patterns)` | Add typed `SpanPattern`s |
| `add_patterns_raw(json)` | Add raw JSON patterns |
| `call(doc)` | Process a `Doc` and add spans |
| `labels()` | All span labels |
| `spans_key()` | Default span group key |
| `len()` / `is_empty()` | Number of patterns |
| `to_bytes()` / `from_bytes(bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(path)` | Disk serialization |

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

### `DocBin`

Efficient binary serializer for collections of `Doc` objects. Produces smaller files than pickle and deserializes without executing arbitrary code.

```rust
use rusty::DocBin;

let docs = vec![nlp.nlp("Hello.")?, nlp.nlp("World.")?];
let bin = DocBin::from_docs(None, false, &docs)?;
let bytes = bin.to_bytes()?;

// Later…
let bin2 = DocBin::from_bytes(&bytes)?;
let recovered = bin2.get_docs(&nlp.vocab()?)?;
```

| Method | Description |
|--------|-------------|
| `new(attrs, store_user_data)` | Create an empty `DocBin` |
| `from_docs(attrs, store_user_data, docs)` | Create and populate in one call |
| `add(doc)` | Add a `Doc` |
| `len()` / `is_empty()` | Number of stored docs |
| `merge(other)` | Merge another `DocBin` into this one |
| `to_bytes()` / `from_bytes(bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(path)` | Disk serialization (`.spacy` extension) |
| `get_docs(vocab)` | Recover `Doc` objects |

### `EntityRuler`

Pipeline component for rule-based named entity recognition. Can be used alone or combined with the statistical `EntityRecognizer`.

```rust
use rusty::{EntityRuler, EntityPattern, TokenPattern};

let ruler = EntityRuler::new(&nlp, None, false, false, None)?;
let patterns = vec![
    EntityPattern::phrase("ORG", "Apple"),
    EntityPattern::tokens("ORG", vec![TokenPattern::new().lower("apple")]),
];
ruler.add_patterns(&patterns)?;

let doc = ruler.call(&nlp.nlp("Apple is great.")?)?;
for ent in doc.ents()? {
    println!("{} -> {}", ent.text()?, ent.label_()?);
}
```

| Method | Description |
|--------|-------------|
| `new(language, phrase_matcher_attr, validate, overwrite_ents, patterns)` | Create an `EntityRuler` |
| `add_patterns(patterns)` | Add typed patterns |
| `add_patterns_raw(json)` | Add raw JSON patterns |
| `call(doc)` | Process a `Doc` and add matches to `doc.ents` |
| `labels()` | All labels in the patterns |
| `ent_ids()` | All entity IDs in the patterns |
| `patterns()` | All patterns as `serde_json::Value` |
| `contains(label)` | Check if a label exists |
| `len()` / `is_empty()` | Number of patterns |
| `to_bytes()` / `from_bytes(bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(path)` | Disk serialization |

### `KnowledgeBase` & `Candidate`

In-memory knowledge base for entity linking. Entities have vectors and surface-form aliases map to candidates.

```rust
use rusty::{KnowledgeBase, EntityLinker};

let kb = KnowledgeBase::new(&vocab, 4)?;
kb.add_entity("Q1", 100, &[1.0, 2.0, 3.0, 4.0])?;
kb.add_alias("Apple", &["Q1"], &[0.9])?;

let candidates = kb.get_candidates("Apple")?;
for c in &candidates {
    println!("{} -> {} (prob: {})", c.alias_()?, c.entity_()?, c.prior_prob()?);
}
```

**KnowledgeBase:**
| Method | Description |
|--------|-------------|
| `new(vocab, entity_vector_length)` | Create an empty `InMemoryLookupKB` |
| `add_entity(entity, freq, vector)` | Add an entity with a vector |
| `add_alias(alias, entities, probabilities)` | Add a surface-form alias |
| `contains_entity(entity)` / `contains_alias(alias)` | Existence checks |
| `get_candidates(alias)` | Get `Candidate`s for an alias |
| `get_vector(entity)` | Retrieve entity vector |
| `get_prior_prob(alias, entity)` | Alias→entity prior probability |
| `entity_vector_length()` | Vector dimensionality |
| `is_empty()` / `get_size_entities()` / `get_size_aliases()` | Size queries |
| `get_entity_strings()` / `get_alias_strings()` | All keys |
| `to_bytes()` / `from_bytes(vocab, bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(vocab, path)` | Disk serialization |

**Candidate:**
| Method | Description |
|--------|-------------|
| `entity_()` / `alias_()` | String IDs |
| `prior_prob()` | Probability |
| `entity_vector()` | Entity vector |
| `entity_freq()` | Entity frequency |

### `EntityLinker`

Pipeline component for linking named entities to a `KnowledgeBase`. Must be initialized before running inference.

```rust
let el = EntityLinker::new(&nlp, "entity_linker", 4)?;
el.set_kb(&kb)?;
```

| Method | Description |
|--------|-------------|
| `new(language, name, entity_vector_length)` | Create and add to pipeline |
| `from_pipe(language, name)` | Retrieve existing linker |
| `set_kb(knowledge_base)` | Attach a `KnowledgeBase` |
| `call(doc)` | Process a `Doc` (requires init) |
| `labels()` | Configured labels |
| `cfg()` | Component config as JSON |
| `name()` | Component name |
| `to_bytes()` / `from_bytes(language, name, bytes)` | Byte serialization |
| `to_disk(path)` / `from_disk(language, name, path)` | Disk serialization |

### `Example`

A training example pairing a predicted `Doc` with a reference `Doc` containing gold annotations.

```rust
use rusty::Example;

let annotations = r#"{"entities": [[0, 5, "ORG"]]}"#;
let example = Example::from_text_and_annotations(&nlp, "Apple is great.", annotations)?;

let tags = example.get_aligned_ner()?;
assert!(tags.iter().any(|t| t == "U-ORG"));
```

| Method | Description |
|--------|-------------|
| `from_dict(doc, annotations_json)` | Create from a `Doc` + JSON annotations |
| `from_text_and_annotations(language, text, annotations_json)` | Create from text + annotations |
| `predicted()` / `reference()` | Predicted and gold `Doc`s |
| `text()` | Text of the example |
| `to_dict()` | Export as spaCy annotation dict |
| `get_aligned_ner()` | Aligned BILUO NER tags |
| `split_sents()` | Split into one example per sentence |

### `Training Utilities`

Helper functions for preparing training data.

```rust
use rusty::offsets_to_biluo_tags;

let doc = nlp.nlp("Apple is great.")?;
let tags = offsets_to_biluo_tags(&doc, &[(0, 5, "ORG")])?;
```

| Function | Description |
|----------|-------------|
| `offsets_to_biluo_tags(doc, entities)` | Convert `(start, end, label)` offsets to BILUO tags |

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
| `remove_span(name, index)` | Safely remove a span by rebuilding the group |
| `names()` | List all group names |
| `has(name)` | Check if a group exists |

**SpanGroup:**
| Method | Description |
|--------|-------------|
| `len()` / `is_empty()` | Number of spans |
| `spans()` | Get all spans |
| `get(index)` / `set(index, span)` | Index access |
| `append(span)` / `extend(spans)` | Add spans |
| `has_overlap()` | Check for overlapping spans |
| `copy()` | Return a copy |

> **Note:** `SpanGroup::remove` is intentionally absent because spaCy's `SpanGroup.__delitem__` contains an off-by-one bug that corrupts the heap. Use `SpanGroups::remove_span` instead.

### Custom Extension Attributes

Register custom attributes, properties, and methods on `Doc`, `Span`, and `Token` via the `._` namespace.

```rust
use rusty::{Doc, ExtensionDefinition};
use serde_json::json;

// Attribute extension
Doc::set_extension(
    "doc_id",
    ExtensionDefinition::Attribute { default: json!(0) },
    false,
)?;

// Property extension
Doc::set_extension(
    "upper_text",
    ExtensionDefinition::Property {
        getter: "lambda doc: doc.text.upper()".to_string(),
        setter: None,
    },
    false,
)?;

// Method extension with kwargs
Doc::set_extension(
    "has_word",
    ExtensionDefinition::Method {
        method: "lambda doc, word: word in doc.text".to_string(),
    },
    false,
)?;

let doc = nlp.nlp("Hello world.")?;
doc.set_underscore("doc_id", json!(42))?;
assert_eq!(doc.get_underscore("doc_id")?, json!(42));
assert_eq!(doc.get_underscore("upper_text")?, "HELLO WORLD.");

let mut kwargs = std::collections::HashMap::new();
kwargs.insert("word".to_string(), json!("world"));
assert_eq!(doc.call_underscore("has_word", &[], &kwargs)?, json!(true));
```

**Class methods (`Doc` / `Span` / `Token`):**
| Method | Description |
|--------|-------------|
| `set_extension(name, def, force)` | Register an extension |
| `has_extension(name)` | Check if an extension is registered |
| `remove_extension(name)` | Remove an extension (returns `ExtensionInfo`) |

**Instance methods (`Doc` / `Span` / `Token`):**
| Method | Description |
|--------|-------------|
| `get_underscore(name)` | Get a value from `._` |
| `set_underscore(name, value)` | Set a value on `._` |
| `has_underscore(name)` | Check if instance has the custom attr |
| `call_underscore(name, args, kwargs)` | Call a method extension |

### `DependencyMatcher`

Match dependency subtrees using Semgrex-style operators.

```rust
use rusty::{DependencyMatcher, DependencyPatternNode, TokenPattern};

let vocab = doc.vocab()?;
let matcher = DependencyMatcher::new(&vocab, false)?;
let pattern = vec![
    vec![
        DependencyPatternNode {
            left_id: None,
            rel_op: None,
            right_id: "like".to_string(),
            right_attrs: TokenPattern::new().lower("like"),
        },
        DependencyPatternNode {
            left_id: Some("like".to_string()),
            rel_op: Some(">".to_string()),
            right_id: "subject".to_string(),
            right_attrs: TokenPattern::new().dep("nsubj"),
        },
    ],
];
matcher.add("SUBJ", pattern)?;
let matches = matcher.call(&doc)?;
```

| Method | Description |
|--------|-------------|
| `new(vocab, validate)` | Create a `DependencyMatcher` |
| `add(name, patterns)` | Add dependency patterns |
| `call(doc)` | Find matches |
| `len()` / `is_empty()` / `contains(name)` / `remove(name)` | Rule management |

### `MorphAnalysis`

Structured access to token morphology.

```rust
let token = doc.token(0)?;
let morph = token.morph()?;
let features = morph.to_dict()?;
```

| Method | Description |
|--------|-------------|
| `to_string()` | UD FEATS string |
| `to_dict()` | `HashMap<String, String>` |
| `get(field)` | Values for a feature field |
| `contains(feature_value)` | Check feature/value pair |
| `len()` / `is_empty()` | Number of features |
| `features()` | All feature/value pairs as strings |

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
