
// use std::mem::{ size_of };
use onejoker::*;

fn main() -> Result<(), OjError> {
    println!("{} {}", 
        size_of::<[u16; 2598961]>(),
        size_of::<[(HandLevel, [Rank; 5]); 7463]>());
    Ok(())
}
