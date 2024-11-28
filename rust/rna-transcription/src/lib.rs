#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !['A', 'C', 'G', 'T'].contains(&c)) {
            Some(i) => Err(i),
            None => Ok(Dna(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => panic!("Invalid DNA nucleotide"),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !['U', 'G', 'C', 'A'].contains(&c)) {
            Some(i) => Err(i),
            None => Ok(Rna(rna.to_string())),
        }
    }
}
