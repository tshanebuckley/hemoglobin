use std::collections::HashMap;

/// https://rosalind.info/problems/ba1a/
/// Compute the Number of Times a Pattern Appears in a Text.
/// Bioinformatics Algorithms: An Active-Learning Approach; page 8.
/// --------------------------------------------------------------
/// Given text and pattern, returns the number of times that the
/// pattern appears in the given text.
/// --------------------------------------------------------------
pub fn pattern_count(text: &str, pattern: &str) -> u64
{
    let mut count = 0;
    let pattern_len = pattern.len();
    let upper_bound = text.len() - pattern_len + 1;

    for i in 0..upper_bound
    {
        let end = i + pattern_len;
        if &text[i..end] == pattern 
        {
            count += 1;
        }
    }
    count
}

/// https://rosalind.info/problems/ba1b/
/// Compute the Number of Times a Pattern Appears in a Text.
/// Bioinformatics Algorithms: An Active-Learning Approach; page 8.
/// --------------------------------------------------------------
/// Given text and kmer length, returns the most frequently occurring
/// kmer of the given length.
/// --------------------------------------------------------------
pub fn frequent_words(text: &str, word_size: u64) -> Vec<&str>
{
    let size = word_size as usize;
    let upper_bound = text.len() - size + 1;
    let ref mut word_frequencies = HashMap::new();
    let mut highest_frequency = 0;

    for i in 0..upper_bound
    {
        let end = i + size;
        let word = &text[i..end];
        if !word_frequencies.contains_key(word)
        {
            word_frequencies.insert(word, 0);
        }
        else
        {
            match word_frequencies.get_mut(word) {
                None => { },
                Some(count) => {
                    *count += 1;
                    if *count > highest_frequency
                    {
                        highest_frequency = *count;
                    }
                },
            };
        }
    };

    let mut ret_val = vec![];
    for word in word_frequencies.keys()
    {
        if word_frequencies[word] == highest_frequency
        {
            ret_val.push(*word);
        }
    };

    ret_val

}
