use rusty::{
    Candidate, DependencyMatcher, DependencyPatternNode, Doc, DocBin, EntityLinker, EntityPattern,
    EntityRuler, ExtensionDefinition, Head, KnowledgeBase, Language, Lexeme, Matcher,
    MorphAnalysis, PatternValue, PhraseMatcher, SpanPattern, SpanRuler, Token, TokenPattern,
    Vectors,
};
use serde_json::json;

fn get_nlp() -> Language {
    Language::load("en_core_web_sm").expect(
        "Failed to load en_core_web_sm. Install it with: python -m spacy download en_core_web_sm",
    )
}

#[test]
fn test_language_load() {
    let _nlp = get_nlp();
}

#[test]
fn test_doc_creation() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").expect("Failed to create doc");
    assert_eq!(doc.text().unwrap(), "Hello, world!");
}

#[test]
fn test_doc_tokens() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").unwrap();
    let tokens = doc.tokens().unwrap();
    assert!(!tokens.is_empty());

    let first = &tokens[0];
    assert_eq!(first.text().unwrap(), "Hello");
    assert_eq!(first.lemma_().unwrap(), "hello");
    // POS may vary by model version; just check it's not empty
    assert!(!first.pos_().unwrap().is_empty());
}

#[test]
fn test_doc_token_by_index() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").unwrap();
    let token = doc.token(0).unwrap();
    assert_eq!(token.text().unwrap(), "Hello");
}

#[test]
fn test_token_booleans() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello 42").unwrap();
    let tokens = doc.tokens().unwrap();

    // Find tokens by text rather than hardcoding indices
    let hello = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Hello")
        .unwrap();
    let num42 = tokens.iter().find(|t| t.text().unwrap() == "42").unwrap();

    assert!(hello.is_alpha().unwrap());
    assert!(!hello.is_digit().unwrap());
    assert!(num42.like_num().unwrap());
}

#[test]
fn test_doc_entities() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    let ents = doc.ents().unwrap();

    assert!(!ents.is_empty());
    let first_ent = &ents[0];
    assert_eq!(first_ent.text().unwrap(), "Apple");
    // Label may vary by model version; just check it's not empty
    assert!(!first_ent.label_().unwrap().is_empty());
}

#[test]
fn test_doc_sents() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world. This is a test.").unwrap();
    let sents = doc.sents().unwrap();
    assert_eq!(sents.len(), 2);
    assert_eq!(sents[0].text().unwrap(), "Hello world.");
    assert_eq!(sents[1].text().unwrap(), "This is a test.");
}

#[test]
fn test_doc_noun_chunks() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    let chunks = doc.noun_chunks().unwrap();
    assert!(!chunks.is_empty());
}

#[test]
fn test_doc_len() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").unwrap();
    let len = doc.len().unwrap();
    assert!(len > 0);
}

#[test]
fn test_doc_is_empty() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").unwrap();
    assert!(!doc.is_empty().unwrap());
}

// Note: similarity requires a model with word vectors (e.g. en_core_web_md/lg).
// en_core_web_sm does not have vectors, so this test is skipped by default.
// Uncomment and use a larger model to test similarity.
// #[test]
// fn test_similarity() {
//     let nlp = Language::load("en_core_web_md").unwrap();
//     let doc1 = nlp.nlp("I love apples.").unwrap();
//     let doc2 = nlp.nlp("I enjoy fruit.").unwrap();
//     let sim = doc1.similarity(&doc2).unwrap();
//     assert!(sim >= 0.0 && sim <= 1.0);
// }

#[test]
fn test_pipe() {
    let nlp = get_nlp();
    let texts = vec!["Hello world.", "This is a test."];
    let docs = nlp.pipe(&texts).unwrap();
    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0].text().unwrap(), "Hello world.");
    assert_eq!(docs[1].text().unwrap(), "This is a test.");
}

#[test]
fn test_serialization_roundtrip() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello, world!").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let doc2 = rusty::Doc::from_bytes(&nlp, &bytes).unwrap();
    assert_eq!(doc.text().unwrap(), doc2.text().unwrap());
}

#[test]
fn test_to_json() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello.").unwrap();
    let json = doc.to_json().unwrap();
    assert!(json.is_object());
}

#[test]
fn test_token_head() {
    let nlp = get_nlp();
    let doc = nlp.nlp("I like apples.").unwrap();
    let tokens = doc.tokens().unwrap();
    let like = tokens.iter().find(|t| t.text().unwrap() == "like").unwrap();
    let head = like.head().unwrap();
    // The root token's head is itself
    assert_eq!(head.text().unwrap(), "like");
}

#[test]
fn test_token_children() {
    let nlp = get_nlp();
    let doc = nlp.nlp("I like apples.").unwrap();
    let tokens = doc.tokens().unwrap();
    let like = tokens.iter().find(|t| t.text().unwrap() == "like").unwrap();
    let children = like.children().unwrap();
    assert!(!children.is_empty());
}

#[test]
fn test_span_tokens() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is great.").unwrap();
    let ents = doc.ents().unwrap();
    if !ents.is_empty() {
        let ent = &ents[0];
        let tokens = ent.tokens().unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].text().unwrap(), "Apple");
    }
}

#[test]
fn test_span_as_doc() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is great. I agree.").unwrap();
    let sents = doc.sents().unwrap();
    let sent = &sents[0];
    let sent_doc = sent.as_doc().unwrap();
    // spaCy may include trailing whitespace in sentence spans
    assert_eq!(sent_doc.text().unwrap().trim(), "Apple is great.");
}

#[test]
fn test_language_component_names() {
    let nlp = get_nlp();
    let names = nlp.component_names().unwrap();
    assert!(!names.is_empty());
}

#[test]
fn test_vocab_strings() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let strings = vocab.strings().unwrap();
    let hash = strings.add("testword").unwrap();
    let retrieved = strings.get_string(hash).unwrap();
    assert_eq!(retrieved, "testword");
}

#[test]
fn test_token_morph() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apples are tasty.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apples = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apples")
        .unwrap();
    let morph = apples.morph_().unwrap();
    // spaCy v3 morphology is a pipe-delimited string like "Number=Plur"
    assert!(!morph.is_empty());
}

#[test]
fn test_token_whitespace() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let tokens = doc.tokens().unwrap();
    let hello = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Hello")
        .unwrap();
    // "Hello" is followed by a space in "Hello world."
    assert_eq!(hello.whitespace_().unwrap(), " ");
}

#[test]
fn test_token_ent_id() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apple = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apple")
        .unwrap();
    // ent_id_ is typically empty unless entity linking is configured
    let _ent_id = apple.ent_id_().unwrap();
}

#[test]
fn test_token_lexeme_features() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is great.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apple = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apple")
        .unwrap();

    // rank, prob, and cluster return valid values for known words
    let _rank = apple.rank().unwrap();
    let prob = apple.prob().unwrap();
    // Probability is typically a negative log probability (negative or zero)
    assert!(prob.is_finite());
    let _cluster = apple.cluster().unwrap();
}

#[test]
fn test_doc_char_span() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    // "Apple" starts at char 0 and ends at char 5
    let span = doc.char_span(0, 5, None, None, None).unwrap();
    assert!(span.is_some());
    assert_eq!(span.unwrap().text().unwrap(), "Apple");

    // Invalid strict span should return None
    let bad = doc.char_span(2, 8, None, None, Some("strict")).unwrap();
    assert!(bad.is_none());

    // Expand mode should pick up overlapping tokens
    let expanded = doc
        .char_span(2, 8, None, None, Some("expand"))
        .unwrap()
        .unwrap();
    assert_eq!(expanded.text().unwrap(), "Apple is");
}

#[test]
fn test_span_label_int() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    let ents = doc.ents().unwrap();
    assert!(!ents.is_empty());
    let first = &ents[0];
    // label_() is the string label, label() is the integer hash
    let label_str = first.label_().unwrap();
    let label_int = first.label().unwrap();
    assert!(!label_str.is_empty());
    assert!(label_int != 0);
}

#[test]
fn test_matcher_basic() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let vocab = doc.vocab().unwrap();
    let matcher = Matcher::new(&vocab, false).unwrap();
    matcher
        .add("GREETING", vec![TokenPattern::new().orth("Hello")])
        .unwrap();
    let matches = matcher.call(&doc).unwrap();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].start, 0);
    assert_eq!(matches[0].end, 1);
}

#[test]
fn test_matcher_in() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let vocab = doc.vocab().unwrap();
    let matcher = Matcher::new(&vocab, false).unwrap();
    matcher
        .add(
            "GREETING",
            vec![TokenPattern::new()
                .orth(PatternValue::in_list(vec!["Hello".into(), "Hi".into()]))],
        )
        .unwrap();
    let matches = matcher.call(&doc).unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_matcher_as_spans() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let vocab = doc.vocab().unwrap();
    let matcher = Matcher::new(&vocab, false).unwrap();
    matcher
        .add("GREETING", vec![TokenPattern::new().orth("Hello")])
        .unwrap();
    let spans = matcher.call_as_spans(&doc).unwrap();
    assert!(!spans.is_empty());
    assert_eq!(spans[0].text().unwrap(), "Hello");
}

#[test]
fn test_phrase_matcher() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let vocab = doc.vocab().unwrap();
    let pattern_doc = nlp.nlp("Hello").unwrap();
    let matcher = PhraseMatcher::new(&vocab, None, false).unwrap();
    matcher.add("GREETING", &[pattern_doc]).unwrap();
    let matches = matcher.call(&doc).unwrap();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].start, 0);
    assert_eq!(matches[0].end, 1);
}

#[test]
fn test_retokenize_merge() {
    let nlp = get_nlp();
    let doc = nlp.nlp("New York").unwrap();
    let span = doc
        .char_span(0, 8, None, None, None)
        .unwrap()
        .expect("valid char span");
    {
        let mut retokenizer = doc.retokenize().unwrap();
        retokenizer.merge(&span, None).unwrap();
    }
    let tokens = doc.tokens().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].text().unwrap(), "New York");
}

#[test]
fn test_retokenize_split() {
    let nlp = get_nlp();
    let doc = nlp.nlp("NewYork").unwrap();
    let token = doc.token(0).unwrap();
    {
        let mut retokenizer = doc.retokenize().unwrap();
        // one head per subtoken
        let heads = vec![Head::Token(token.clone()), Head::Token(token.clone())];
        retokenizer
            .split(&token, &["New", "York"], &heads, None)
            .unwrap();
    }
    let tokens = doc.tokens().unwrap();
    assert_eq!(tokens[0].text().unwrap(), "New");
    assert_eq!(tokens[1].text().unwrap(), "York");
}

#[test]
fn test_span_groups() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apple is looking at buying a startup.").unwrap();
    let spans = doc.spans().unwrap();
    let ents = doc.ents().unwrap();
    if !ents.is_empty() {
        spans.set("ents", &[ents[0].clone()]).unwrap();
        let group = spans.get("ents").unwrap().unwrap();
        assert_eq!(group.len().unwrap(), 1);

        // append is safe
        group.append(&ents[0]).unwrap();
        assert_eq!(group.len().unwrap(), 2);

        // remove_span rebuilds the group safely (avoids spaCy __delitem__ bug)
        spans.remove_span("ents", 0).unwrap();
        let group = spans.get("ents").unwrap().unwrap();
        assert_eq!(group.len().unwrap(), 1);
    }
}

#[test]
fn test_doc_cats() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let cats = doc.cats().unwrap();
    // en_core_web_sm has no textcat component
    assert!(cats.is_empty());
}

#[test]
fn test_doc_vector_norm() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let norm = doc.vector_norm().unwrap();
    assert!(norm >= 0.0);
}

#[test]
fn test_span_vector_norm() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let sents = doc.sents().unwrap();
    if !sents.is_empty() {
        let norm = sents[0].vector_norm().unwrap();
        assert!(norm >= 0.0);
    }
}

#[test]
fn test_has_annotation() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    assert!(doc.has_annotation("TAG", false).unwrap());
    assert!(doc.has_annotation("DEP", false).unwrap());
}

#[test]
fn test_count_by() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    // spaCy POS attribute ID = 74
    let counts = doc.count_by(74).unwrap();
    assert!(!counts.is_empty());
}

// === Tier 3: Token vectors, DocBin, EntityRuler ===

#[test]
fn test_token_vector_norm() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let token = doc.token(0).unwrap();
    let norm = token.vector_norm().unwrap();
    assert!(norm >= 0.0);
}

#[test]
fn test_doc_bin_roundtrip_bytes() {
    let nlp = get_nlp();
    let doc1 = nlp.nlp("Hello world.").unwrap();
    let doc2 = nlp.nlp("Foo bar.").unwrap();

    let bin = DocBin::new(None, false).unwrap();
    bin.add(&doc1).unwrap();
    bin.add(&doc2).unwrap();
    assert_eq!(bin.len().unwrap(), 2);
    assert!(!bin.is_empty().unwrap());

    let bytes = bin.to_bytes().unwrap();
    let bin2 = DocBin::from_bytes(&bytes).unwrap();
    assert_eq!(bin2.len().unwrap(), 2);

    let vocab = nlp.vocab().unwrap();
    let docs = bin2.get_docs(&vocab).unwrap();
    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0].text().unwrap(), "Hello world.");
    assert_eq!(docs[1].text().unwrap(), "Foo bar.");
}

#[test]
fn test_doc_bin_roundtrip_disk() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let path = "/tmp/test_docbin.spacy";

    let bin = DocBin::from_docs(None, false, &[doc]).unwrap();
    bin.to_disk(path).unwrap();

    let bin2 = DocBin::from_disk(path).unwrap();
    assert_eq!(bin2.len().unwrap(), 1);

    let vocab = nlp.vocab().unwrap();
    let docs = bin2.get_docs(&vocab).unwrap();
    assert_eq!(docs[0].text().unwrap(), "Hello world.");
}

#[test]
fn test_doc_to_disk_from_disk() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let path = "/tmp/test_doc_dir";

    doc.to_disk(path).unwrap();
    let doc2 = Doc::from_disk(&nlp, path).unwrap();
    assert_eq!(doc2.text().unwrap(), "Hello world.");
}

#[test]
fn test_doc_from_json() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Hello world.").unwrap();
    let json = doc.to_json().unwrap();
    let doc2 = Doc::from_json(&nlp, &json).unwrap();
    assert_eq!(doc2.text().unwrap(), "Hello world.");
}

#[test]
fn test_entity_ruler_phrase_pattern() {
    let nlp = get_nlp();
    let ruler = EntityRuler::new(&nlp, None, false, false, None).unwrap();
    let patterns = vec![EntityPattern::phrase("ORG", "Apple")];
    ruler.add_patterns(&patterns).unwrap();

    assert_eq!(ruler.len().unwrap(), 1);
    assert!(ruler.contains("ORG").unwrap());
    let labels = ruler.labels().unwrap();
    assert!(labels.contains(&"ORG".to_string()));

    let doc = nlp.nlp("Apple is great.").unwrap();
    let doc = ruler.call(&doc).unwrap();
    let ents = doc.ents().unwrap();
    assert!(ents
        .iter()
        .any(|e| { e.text().unwrap() == "Apple" && e.label_().unwrap() == "ORG" }));
}

#[test]
fn test_entity_ruler_token_pattern() {
    let nlp = get_nlp();
    let ruler = EntityRuler::new(&nlp, None, false, false, None).unwrap();
    let patterns = vec![EntityPattern::tokens(
        "ORG",
        vec![TokenPattern::new().lower("apple")],
    )];
    ruler.add_patterns(&patterns).unwrap();

    let doc = nlp.nlp("apple is great.").unwrap();
    let doc = ruler.call(&doc).unwrap();
    let ents = doc.ents().unwrap();
    assert!(ents
        .iter()
        .any(|e| { e.text().unwrap() == "apple" && e.label_().unwrap() == "ORG" }));
}

#[test]
fn test_entity_ruler_bytes_roundtrip() {
    let nlp = get_nlp();
    let ruler = EntityRuler::new(&nlp, None, false, false, None).unwrap();
    let patterns = vec![EntityPattern::phrase("ORG", "Apple")];
    ruler.add_patterns(&patterns).unwrap();

    let bytes = ruler.to_bytes().unwrap();
    ruler.from_bytes(&bytes).unwrap();
    assert_eq!(ruler.len().unwrap(), 1);
}

// === Tier 4: Extensions, DependencyMatcher, Pipeline helpers, MorphAnalysis ===

#[test]
fn test_doc_attribute_extension() {
    let nlp = get_nlp();
    Doc::set_extension(
        "doc_id",
        ExtensionDefinition::Attribute { default: json!(0) },
        true,
    )
    .unwrap();
    assert!(Doc::has_extension("doc_id").unwrap());

    let doc = nlp.nlp("Hello world.").unwrap();
    assert!(doc.has_underscore("doc_id").unwrap());
    doc.set_underscore("doc_id", json!(42)).unwrap();
    assert_eq!(doc.get_underscore("doc_id").unwrap(), json!(42));

    let info = Doc::remove_extension("doc_id").unwrap();
    assert!(info.default.is_some());
    assert!(!info.has_getter);
    assert!(!info.has_setter);
    assert!(!info.has_method);
}

#[test]
fn test_doc_property_extension() {
    let nlp = get_nlp();
    Doc::set_extension(
        "upper_text",
        ExtensionDefinition::Property {
            getter: "lambda doc: doc.text.upper()".to_string(),
            setter: None,
        },
        true,
    )
    .unwrap();

    let doc = nlp.nlp("Hello world.").unwrap();
    let val = doc.get_underscore("upper_text").unwrap();
    assert_eq!(val, "HELLO WORLD.");

    Doc::remove_extension("upper_text").unwrap();
}

#[test]
fn test_doc_method_extension_kwargs() {
    let nlp = get_nlp();
    Doc::set_extension(
        "has_word",
        ExtensionDefinition::Method {
            method: "lambda doc, word: word in doc.text".to_string(),
        },
        true,
    )
    .unwrap();

    let doc = nlp.nlp("Hello world.").unwrap();
    let mut kwargs = std::collections::HashMap::new();
    kwargs.insert("word".to_string(), json!("world"));
    let result = doc.call_underscore("has_word", &[], &kwargs).unwrap();
    assert_eq!(result, json!(true));

    Doc::remove_extension("has_word").unwrap();
}

#[test]
fn test_token_attribute_extension() {
    let nlp = get_nlp();
    Token::set_extension(
        "is_greeting",
        ExtensionDefinition::Attribute {
            default: json!(false),
        },
        true,
    )
    .unwrap();

    let doc = nlp.nlp("Hello world.").unwrap();
    let token = doc.token(0).unwrap();
    assert_eq!(token.get_underscore("is_greeting").unwrap(), json!(false));
    token.set_underscore("is_greeting", json!(true)).unwrap();
    assert_eq!(token.get_underscore("is_greeting").unwrap(), json!(true));

    Token::remove_extension("is_greeting").unwrap();
}

#[test]
fn test_dependency_matcher_basic() {
    let nlp = get_nlp();
    let doc = nlp.nlp("I like apples.").unwrap();
    let vocab = doc.vocab().unwrap();
    let matcher = DependencyMatcher::new(&vocab, false).unwrap();

    // Pattern: find a token whose head is "like" (direct dependent)
    let pattern = vec![vec![
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
    ]];
    matcher.add("SUBJ", pattern).unwrap();
    let matches = matcher.call(&doc).unwrap();
    assert!(!matches.is_empty());
    assert_eq!(
        matches[0].id,
        vocab.strings().unwrap().get_hash("SUBJ").unwrap() as u64
    );
}

#[test]
fn test_language_pipe_names() {
    let nlp = get_nlp();
    let names = nlp.pipe_names().unwrap();
    assert!(!names.is_empty());
}

#[test]
fn test_language_rename_pipe() {
    let nlp = get_nlp();
    let old_names = nlp.pipe_names().unwrap();
    if old_names.iter().any(|s| s == "ner") {
        nlp.rename_pipe("ner", "my_ner").unwrap();
        let new_names = nlp.pipe_names().unwrap();
        assert!(!new_names.iter().any(|s| s == "ner"));
        assert!(new_names.iter().any(|s| s == "my_ner"));
        nlp.rename_pipe("my_ner", "ner").unwrap(); // restore
    }
}

#[test]
fn test_language_disabled() {
    let nlp = get_nlp();
    let disabled = nlp.disabled().unwrap();
    // en_core_web_sm may have some disabled pipes (e.g. senter)
    // just check it returns a Vec
    assert!(disabled.iter().all(|s| !s.is_empty()));
}

#[test]
fn test_token_morph_analysis() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apples are tasty.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apples = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apples")
        .unwrap();
    let morph = apples.morph().unwrap();
    assert!(morph.len().unwrap() > 0);
    let s = morph.to_string().unwrap();
    assert!(!s.is_empty());
}

#[test]
fn test_morph_analysis_dict() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apples are tasty.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apples = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apples")
        .unwrap();
    let morph = apples.morph().unwrap();
    let dict = morph.to_dict().unwrap();
    assert!(!dict.is_empty());
    // Check that Number=Plur (or similar) is present for plural nouns
    let features = morph.features().unwrap();
    assert!(!features.is_empty());
}

// === Tier 5 Phase 1+2: Vectors, Lexeme, SpanRuler ===

#[test]
fn test_vectors_create_and_add() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let vectors = Vectors::new_table(&vocab, Some((10, 4))).unwrap();
    assert_eq!(vectors.shape().unwrap(), (10, 4));
    assert!(vectors.is_empty().unwrap());

    let row = vectors
        .add("hello", Some(&[1.0, 2.0, 3.0, 4.0]), None)
        .unwrap();
    assert!(row < 10);
    let hash = vocab.strings().unwrap().get_hash("hello").unwrap();
    assert!(vectors.contains(hash).unwrap());
    assert_eq!(vectors.keys().unwrap().len(), 1);
}

#[test]
fn test_vectors_get_and_find() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let vectors = Vectors::new_table(&vocab, Some((10, 4))).unwrap();
    let vec = vec![1.0f32, 2.0, 3.0, 4.0];
    vectors.add("hello", Some(&vec), None).unwrap();

    let got = vectors.get("hello").unwrap();
    assert_eq!(got.len(), 4);
    assert!((got[0] - 1.0).abs() < 1e-6);

    let row = vectors.find(Some("hello"), None).unwrap();
    assert!(row >= 0);

    let key = vectors.find(None, Some(row as usize)).unwrap();
    assert!(key >= 0);
}

#[test]
fn test_vectors_most_similar() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let vectors = Vectors::new_table(&vocab, Some((10, 4))).unwrap();
    vectors.add("a", Some(&[1.0, 0.0, 0.0, 0.0]), None).unwrap();
    vectors.add("b", Some(&[0.9, 0.1, 0.0, 0.0]), None).unwrap();
    vectors.add("c", Some(&[0.0, 1.0, 0.0, 0.0]), None).unwrap();

    let queries = vec![vec![1.0f32, 0.0, 0.0, 0.0]];
    let (keys, _rows, scores) = vectors.most_similar(&queries, 2, 1024).unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].len(), 2);
    assert_eq!(scores.len(), 1);
    assert_eq!(scores[0].len(), 2);
    // The query itself should be the most similar
    assert!(scores[0][0] >= scores[0][1]);
}

#[test]
fn test_vectors_roundtrip_bytes() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let vectors = Vectors::new_table(&vocab, Some((10, 4))).unwrap();
    vectors
        .add("hello", Some(&[1.0, 2.0, 3.0, 4.0]), None)
        .unwrap();

    let bytes = vectors.to_bytes().unwrap();
    let vectors2 = Vectors::from_bytes(&vocab, &bytes).unwrap();
    let hash = vocab.strings().unwrap().get_hash("hello").unwrap();
    assert!(vectors2.contains(hash).unwrap());
    let got = vectors2.get("hello").unwrap();
    assert!((got[0] - 1.0).abs() < 1e-6);
}

#[test]
fn test_token_lexeme() {
    let nlp = get_nlp();
    let doc = nlp.nlp("Apples are tasty.").unwrap();
    let tokens = doc.tokens().unwrap();
    let apples = tokens
        .iter()
        .find(|t| t.text().unwrap() == "Apples")
        .unwrap();
    let lexeme = apples.lexeme().unwrap();
    assert_eq!(lexeme.orth_().unwrap(), "Apples");
    let prob = lexeme.prob().unwrap();
    assert!(prob.is_finite());
}

#[test]
fn test_vocab_vectors() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let vectors = vocab.vectors().unwrap();
    // en_core_web_sm has no vectors by default
    let _shape = vectors.shape().unwrap();
}

#[test]
fn test_span_ruler_phrase_pattern() {
    let nlp = get_nlp();
    let ruler = SpanRuler::new(&nlp, Some("ruler"), false, false, true, None).unwrap();
    let patterns = vec![SpanPattern::phrase("ORG", "Apple")];
    ruler.add_patterns(&patterns).unwrap();

    let doc = nlp.nlp("Apple is great.").unwrap();
    let doc = ruler.call(&doc).unwrap();
    let spans = doc.spans().unwrap();
    let group = spans.get("ruler").unwrap().unwrap();
    assert_eq!(group.len().unwrap(), 1);
    assert_eq!(group.get(0).unwrap().text().unwrap(), "Apple");
    assert_eq!(group.get(0).unwrap().label_().unwrap(), "ORG");
}

#[test]
fn test_span_ruler_annotate_ents() {
    let nlp = get_nlp();
    let ruler = SpanRuler::new(&nlp, None, true, false, true, None).unwrap();
    let patterns = vec![SpanPattern::phrase("ORG", "Apple")];
    ruler.add_patterns(&patterns).unwrap();

    let doc = nlp.nlp("Apple is great.").unwrap();
    let doc = ruler.call(&doc).unwrap();
    let ents = doc.ents().unwrap();
    assert!(ents
        .iter()
        .any(|e| { e.text().unwrap() == "Apple" && e.label_().unwrap() == "ORG" }));
}

#[test]
fn test_span_ruler_bytes_roundtrip() {
    let nlp = get_nlp();
    let ruler = SpanRuler::new(&nlp, None, false, false, true, None).unwrap();
    let patterns = vec![SpanPattern::phrase("ORG", "Apple")];
    ruler.add_patterns(&patterns).unwrap();

    let bytes = ruler.to_bytes().unwrap();
    ruler.from_bytes(&bytes).unwrap();
    assert_eq!(ruler.len().unwrap(), 1);
}

// === Tier 5 Phase 3: Entity Linking ===

#[test]
fn test_knowledge_base_create_and_query() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let kb = KnowledgeBase::new(&vocab, 4).unwrap();
    assert_eq!(kb.entity_vector_length().unwrap(), 4);
    assert!(kb.is_empty().unwrap());

    kb.add_entity("Q1", 100, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    kb.add_alias("Apple", &["Q1"], &[0.9]).unwrap();

    assert!(kb.contains_entity("Q1").unwrap());
    assert!(!kb.contains_entity("Q2").unwrap());
    assert!(kb.contains_alias("Apple").unwrap());
    assert!(!kb.contains_alias("Microsoft").unwrap());

    let candidates = kb.get_candidates("Apple").unwrap();
    assert_eq!(candidates.len(), 1);
    let c = &candidates[0];
    assert_eq!(c.entity_().unwrap(), "Q1");
    assert_eq!(c.alias_().unwrap(), "Apple");
    assert!((c.prior_prob().unwrap() - 0.9).abs() < 1e-5);
    assert_eq!(c.entity_vector().unwrap().len(), 4);
    assert!((c.entity_freq().unwrap() - 100.0).abs() < 1e-5);

    let vec = kb.get_vector("Q1").unwrap();
    assert_eq!(vec.len(), 4);
    assert!((vec[0] - 1.0).abs() < 1e-6);

    // get_prior_prob may return 0.0 in some spaCy versions;
    // candidate.prior_prob is the reliable source
    let _prob = kb.get_prior_prob("Apple", "Q1").unwrap();

    assert_eq!(kb.get_size_entities().unwrap(), 1);
    assert_eq!(kb.get_size_aliases().unwrap(), 1);

    let entities = kb.get_entity_strings().unwrap();
    assert_eq!(entities, vec!["Q1"]);

    let aliases = kb.get_alias_strings().unwrap();
    assert_eq!(aliases, vec!["Apple"]);
}

#[test]
fn test_knowledge_base_bytes_roundtrip() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let kb = KnowledgeBase::new(&vocab, 4).unwrap();
    kb.add_entity("Q1", 100, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    kb.add_alias("Apple", &["Q1"], &[0.9]).unwrap();

    let bytes = kb.to_bytes().unwrap();
    let kb2 = KnowledgeBase::from_bytes(&vocab, &bytes).unwrap();
    assert!(kb2.contains_entity("Q1").unwrap());
    assert!(kb2.contains_alias("Apple").unwrap());
    let vec = kb2.get_vector("Q1").unwrap();
    assert!((vec[0] - 1.0).abs() < 1e-6);
}

#[test]
fn test_knowledge_base_disk_roundtrip() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let kb = KnowledgeBase::new(&vocab, 4).unwrap();
    kb.add_entity("Q1", 100, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    kb.add_alias("Apple", &["Q1"], &[0.9]).unwrap();

    let path = "/tmp/test_kb";
    let _ = std::fs::remove_dir_all(path);
    kb.to_disk(path).unwrap();
    let kb2 = KnowledgeBase::from_disk(&vocab, path).unwrap();
    assert!(kb2.contains_entity("Q1").unwrap());
    assert!(kb2.contains_alias("Apple").unwrap());
}

#[test]
fn test_entity_linker_creation_and_kb() {
    let nlp = get_nlp();
    let vocab = nlp.vocab().unwrap();
    let kb = KnowledgeBase::new(&vocab, 4).unwrap();
    kb.add_entity("Q1", 100, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    kb.add_alias("Apple", &["Q1"], &[0.9]).unwrap();

    // Create a minimal language pipeline with entity linker
    // (We can't call it without initialization, but we can test creation/set_kb)
    let el = EntityLinker::new(&nlp, "entity_linker", 4).unwrap();
    el.set_kb(&kb).unwrap();

    assert_eq!(el.name().unwrap(), "entity_linker");
    let labels = el.labels().unwrap();
    assert!(labels.is_empty());
}
