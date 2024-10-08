
use onejoker::*;

fn main() ->Result<(), OjError> {
    let s = OrphanHand::from_text("As2d3d4d5d6d7d8d9dTd");
    for i in 0..10 { println!("{}", s[i]); }
    for c in s.iter() { println!("{}", c); }

    Ok(())
}
