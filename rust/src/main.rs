
use onejoker::cards::*;

fn main() ->Result<(), OjError> {
    let v = cards_from_text("  [5dJsAd7cJk]");
    println!("{:?}", v);
    Ok(())
}

