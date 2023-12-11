mod lz77;
pub use lz77::LZ77;

pub trait LZ {
    fn compress(&self, raw_data: &[u8]) -> Vec<u8>;
    fn decompress(&self, compressed_data: &[u8]) -> Vec<u8>;
}
/// Recherche la plus longue correspondance entre une séquence de recherche et une séquence anticipée.
/// S'il ne la trouve pas, il enleve un le dernier carracterere et recommence, jusqu'a trouver une correspondance ou qu'il n'y est plus de charractere.
/// 
/// # Arguments
///
/// * `search` - Séquence de recherche.
/// * `ahead` - Séquence anticipée.
///
/// # Returns
///
/// Un tuple contenant la position et la longueur de la plus longue correspondance trouvée.
///
/// # Exemple
///
/// ```rust
/// use lzrs::find_longest_match;
///
/// // Séquence de recherche.
/// let search_sequence = b"abcde";
///
/// // Séquence anticipée.
/// let ahead_sequence = b"abcde12345";
///
/// // Recherche la plus longue correspondance entre les deux séquences.
/// let (position, length) = find_longest_match(search_sequence, ahead_sequence);
///
/// // Affiche les résultats.
/// println!("Position : {}", position);
/// println!("Longueur : {}", length);
///  
/// 
/// ```
pub fn find_longest_match(search: &[u8], ahead: &[u8]) -> (usize, usize) {
    /*
        Exemple
        Recherche du motif "je mange de la pomme de suite".
        La variable "ahead" est initialisée à "derrr" avec une taille de 5, et elle tente de localiser ce motif.
        Si le motif n'est pas trouvé, la taille est décrémentée, et la recherche est répétée avec "derr", "der", etc.
        Lors de la recherche du motif "de", deux occurrences sont trouvées. On choisira celle qui est la plus proche du tampon (la dernière).
    */
    let search_length: usize = search.len();
    let mut position: isize;
    for length in (1..=ahead.len()).rev() {
        // genere toutes les fenêtres de longueur ahead dans search puis cherche la derniere occurence de ahead
        position = match search
            .windows(length)
            .rposition(|window| window == &ahead[..length])
        {
            Some(pos) => pos as isize,
            None => -1,
        };
        if position != -1 {
            return (search_length - position as usize, length);
        }
    }
    return (0, 0);
}
