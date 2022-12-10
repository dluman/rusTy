#[path = "spacy.rs"]
pub mod spacy;

use cpython::*;

use spacy::Module;

pub fn match_phrases() {
    let gil: GILGuard = Python::acquire_gil();
    let python: Python = gil.python();

    let spacy: Module = spacy::Module::init();
    spacy.load("en_core_web_sm");

    let text: spacy::doc::Doc = spacy::nlp("Cheese is tasty.");

    let builtins: PyModule = PyModule::import(python, "spacy.matcher").unwrap();
    let phrasematcher: PyObject = builtins.get(python, "PhraseMatcher").unwrap();
    let vocab: PyObject = spacy::vocab();
    let matcher: PyObject = phrasematcher.call(python, (vocab,), None).unwrap();

    let phrases_to_match: PyList = vec!["Cheese"].to_py_object(python);
    let patterns: PyObject = spacy::tokenizer(phrases_to_match);
    matcher
        .call_method(python, "add", ("FOOD", patterns), None)
        .unwrap();

    let matches: PyObject = matcher.call(python, (text,), None).unwrap();
    println!("{}", matches);
}
