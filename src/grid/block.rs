use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Block {
    OneByOne,
    TwoByOne,
    TwoByTwo,
    FourByOne,
    FourByTwo,
    EightByTwo
    //OneByOne,
    //OneByTwo,
    //TwoByTwo,
    //TwoByFour,
    //TwoByEight,
}


lazy_static! {
    pub static ref BLOCK_RATIOS: HashMap<Block, f64> = {
        let mut map = HashMap::new();
        // uhh I'm basing this on memories from 10 years ago
        // is there a better authority on this? besides acquiring/counting legos?
        map.insert(Block::OneByOne,     0.05);
        map.insert(Block::TwoByOne,     0.10);
        map.insert(Block::TwoByTwo,     0.20);
        map.insert(Block::FourByOne,    0.10);
        map.insert(Block::FourByTwo,    0.40);
        map.insert(Block::EightByTwo,   0.15);
        map
    };
}
