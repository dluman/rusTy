use spacy_rs::Language;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nlp = Language::load("en_core_web_lg")?;

    let pangram1 = nlp.nlp("With tenure, Suzie'd have all the more leisure for yachting, but her publications are no good.")?;
    let pangram2 = nlp.nlp("Amazingly few discotheques provide jukeboxes.")?;

    let similarity = pangram1.similarity(&pangram2)?;
    println!("Similarity: {}", similarity);

    println!("\n--- Tokens ---");
    for token in pangram1.tokens()? {
        println!(
            "{:15} | lemma: {:15} | pos: {:5} | dep: {:10}",
            token.text()?,
            token.lemma_()?,
            token.pos_()?,
            token.dep_()?
        );
    }

    println!("\n--- Entities ---");
    for ent in pangram1.ents()? {
        println!("{:20} | label: {:10}", ent.text()?, ent.label_()?);
    }

    println!("\n--- Sentences ---");
    for (i, sent) in pangram1.sents()?.iter().enumerate() {
        println!("Sentence {}: {}", i, sent.text()?);
    }

    println!("\n--- Noun Chunks ---");
    for chunk in pangram1.noun_chunks()? {
        println!("{}", chunk.text()?);
    }

    Ok(())
}
