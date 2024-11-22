use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .copied()
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut m = HashMap::new();
    m.insert('A', 0);
    m.insert('C', 0);
    m.insert('G', 0);
    m.insert('T', 0);
    for c in dna.chars() {
        m.get_mut(&c).map(|v| *v += 1).ok_or(c)?;
    }
    Ok(m)
}
