use std::collections::HashMap;

#[derive(Debug)]
pub struct ProteinSequence {
    pub id: String,
    pub seq: String,
}

impl ProteinSequence {
    pub fn length(&self) -> usize {
        self.seq.len()
    }

    pub fn aa_composition(&self) -> HashMap<char, usize> {
        let mut comp = HashMap::new();
        for aa in self.seq.chars() {
            *comp.entry(aa).or_insert(0) += 1;
        }
        comp
    }
}
