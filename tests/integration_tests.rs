use rusty::{Head, Language, Matcher, PatternValue, PhraseMatcher, TokenPattern};

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
