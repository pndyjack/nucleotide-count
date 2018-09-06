use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
  let nucleotides = vec!['A', 'T', 'C', 'G'];
  if !nucleotides.contains(&nucleotide) {
    return Err(nucleotide);
  }
  let dna_vec = Vec::from(dna);
  dna_vec
    .iter()
    .map(|&nucleotide_u8| {
      if nucleotide_u8 as char == nucleotide {
        return Ok(1);
      } else if !nucleotides.contains(&(nucleotide_u8 as char)) {
        return Err(nucleotide_u8 as char);
      }
      return Ok(0);
    })
    .sum::<Result<usize, char>>()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
  let nucleotides = vec!['A', 'T', 'C', 'G'];
  let mut result: HashMap<char, usize> = HashMap::new();
  for nucleotide in nucleotides {
    match count(nucleotide, dna) {
      Ok(count) => {
        result.insert(nucleotide, count);
      }
      Err(nucleotide) => {
        return Err(nucleotide);
      }
    }
  }
  Ok(result)
}
