use crate::utils::{doc, spacy};

use cpython::*;
use spacy::*;
use std::collections::HashMap;

pub fn match_phrases() {
    let gil: GILGuard = Python::acquire_gil();
    let python: Python = gil.python();

    let spacy: Module = spacy::Module::init();
    spacy.load("en_core_web_sm");

    let text: doc::Doc = spacy::nlp("United Kingdom and United States");

    let builtins: PyModule = PyModule::import(python, "spacy.matcher").unwrap();
    let phrasematcher: PyObject = builtins.get(python, "PhraseMatcher").unwrap();

    let vocab: PyObject = spacy::vocab();
    let matcher: PyObject = phrasematcher.call(python, (vocab,), None).unwrap();

    let phrases_to_match: PyList = vec!["United Kingdom", "United States"].to_py_object(python);
    let patterns: PyObject = spacy::tokenizer(phrases_to_match);
    matcher
        .call_method(python, "add", ("COUNTRY", patterns), None)
        .unwrap();

    let mut kwargs: HashMap<String, bool> = HashMap::new();
    kwargs.insert("as_spans".to_string(), true);
    let py_kwargs: PyDict = kwargs.to_py_object(python);

    let matches: PyObject = matcher.call(python, (&text,), Some(&py_kwargs)).unwrap();
    println!("{}", matches);
}
