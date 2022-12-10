#[path = "spacy.rs"]
pub mod spacy;

use cpython::*;

use spacy::Module;
use std::collections::HashMap;

pub fn match_phrases() {
    let gil: GILGuard = Python::acquire_gil();
    let python: Python = gil.python();

    let spacy: Module = spacy::Module::init();
    spacy.load("en_core_web_sm");

    let text: spacy::doc::Doc = spacy::nlp("United Kingdom and United States");

    let builtins: PyModule = PyModule::import(python, "spacy.matcher").unwrap();
    let phrasematcher: PyObject = builtins.get(python, "PhraseMatcher").unwrap();
    let vocab: PyObject = spacy::vocab();
    let matcher: PyObject = phrasematcher.call(python, (vocab,), None).unwrap();

    let phrases_to_match: PyList = vec!["United Kingdom", "United States"].to_py_object(python);
    let patterns: PyObject = spacy::tokenizer(phrases_to_match);
    matcher
        .call_method(python, "add", ("FOOD", patterns), None)
        .unwrap();

    let mut kwargs = HashMap::new();
    kwargs.insert("as_spans".to_string(), true);
    let py_kwargs = kwargs.to_py_object(python);

    let matches: PyObject = matcher.call(python, (text,), Some(&py_kwargs)).unwrap();
    println!("{}", matches);
}
