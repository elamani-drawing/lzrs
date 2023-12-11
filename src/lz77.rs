use crate::{find_longest_match, LZ};

#[derive(Debug)]
pub struct LZ77 {
    /// Taille maximale du dictionnaire de recherche.
    max_dictionary_size: usize,
    /// Taille du tampon de recherche.
    lookahead_buffer_size: usize,
}

impl LZ77 {
    /// Crée une nouvelle instance de LZ77, par defaut le dictionnaire de recherche fait 12 bits (4095 usize)
    /// et le tampon de recherche est de 4 bits (15 usize)
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let lz77 = LZ77::new();
    /// ```
    pub fn new() -> Self {
        LZ77 {
            max_dictionary_size: 4095, // 12 bits
            lookahead_buffer_size: 15, // 4 bits
        }
    }

    /// Obtenir la taille maximale du dictionnaire de recherche.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let lz77 = LZ77::new();
    ///
    /// // Obtient la taille maximale du dictionnaire.
    /// let max_size = lz77.get_max_dictionary_size();
    /// ```
    pub fn get_max_dictionary_size(&self) -> usize {
        self.max_dictionary_size
    }

    /// Définir une nouvelle taille maximale pour le dictionnaire.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let mut lz77 = LZ77::new();
    ///
    /// // Définit une nouvelle taille maximale pour le dictionnaire.
    /// lz77.set_max_dictionary_size(8192);
    /// ```
    pub fn set_max_dictionary_size(&mut self, new_size: usize) {
        self.max_dictionary_size = new_size;
    }

    /// Obtenir la taille du tampon de recherche.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let lz77 = LZ77::new();
    ///
    /// // Obtient la taille du tampon de recherche.
    /// let buffer_size = lz77.get_lookahead_buffer_size();
    /// ```
    pub fn get_lookahead_buffer_size(&self) -> usize {
        self.lookahead_buffer_size
    }

    /// Définir une nouvelle taille pour le tampon de recherche.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let mut lz77 = LZ77::new();
    ///
    /// // Définit une nouvelle taille pour le tampon de recherche.
    /// lz77.set_lookahead_buffer_size(20);
    /// ```
    pub fn set_lookahead_buffer_size(&mut self, new_size: usize) {
        self.lookahead_buffer_size = new_size;
    }

    /// Encode les informations de position, longueur et caractère suivant dans le vecteur compressé.
    ///
    /// # Arguments
    ///
    /// * `compressed_data` - Vecteur où les données compressées sont stockées.
    /// * `position` - Position de début de la correspondance dans le dictionnaire.
    /// * `length` - Longueur de la correspondance.
    /// * `next_character` - Caractère suivant dans la séquence.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Vecteur pour stocker les données compressées.
    /// let mut compressed_data = Vec::new();
    ///
    /// // Ajoute une entrée compressée au vecteur.
    /// LZ77::encode(&mut compressed_data, 10, 3, b'a');
    /// // Vérifie que les données compressées correspondent aux attentes.
    /// assert_eq!(compressed_data, vec![10, 48, b'a']);
    /// ```
    /// ```
    pub fn encode(
        compressed_data: &mut Vec<u8>,
        position: usize,
        length: usize,
        next_charracter: u8,
    ) {
        compressed_data.push((position & 0x000000FF) as u8);
        compressed_data
            .push(((position & 0x00000F00) >> 8) as u8 | ((length & 0x0000000F) << 4) as u8);
        compressed_data.push(next_charracter);
    }
    /// Décode les informations de position, longueur et caractère suivant à partir du chunk donné
    /// et met à jour le buffer avec les données décompressées.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Vecteur contenant les données décompressées.
    /// * `chunk` - Chunk de données compressées à décoder.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Vecteur pour stocker les données décompressées.
    /// let mut buffer = Vec::new();
    ///
    /// let chunk = [10, 48, b'a'];
    ///
    /// // Décode le chunk et met à jour le buffer.
    /// LZ77::decode(&mut buffer, &chunk);
    ///
    /// assert_eq!(buffer, vec![b'a'; 1]);
    ///
    /// ```
    pub fn decode(buffer: &mut Vec<u8>, chunk: &[u8]) {
        let position: usize = chunk[0] as usize | ((chunk[1] as usize & 0x0F) << 8);
        let length: usize = (chunk[1] >> 4) as usize;
        // Vérifie si la position et la longueur indiquent une référence au dictionnaire.

        if !(position == 0 && length == 0) {
            // Calcule les indices de début et de fin dans le buffer pour extraire la fenêtre correspondante.

            let start: usize = buffer.len().saturating_sub(position);
            let end: usize = start + length;
            // Extrait la fenêtre du buffer et l'étend à la fin du buffer.
            if start < buffer.len() {
                let window: Vec<u8> = buffer[start..end].to_vec();
                buffer.extend_from_slice(&window);
            }
        }
        buffer.push(chunk[2]);
    }

    /// Compresse les données brutes en utilisant l'algorithme LZ77.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Données brutes à compresser.
    /// * `max_dictionary_size` - Taille maximale du dictionnaire de recherche.
    /// * `lookahead_buffer_size` - Taille du tampon de recherche.
    ///
    /// # Returns
    ///
    /// Un vecteur contenant les données compressées.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Données brutes à compresser.
    /// let input_data = b"hello world";
    ///
    /// // Taille maximale du dictionnaire de recherche.
    /// let max_dict_size = 4095;
    ///
    /// // Taille du tampon de recherche.
    /// let lookahead_buffer_size = 15;
    ///
    /// // Compresse les données brutes.
    /// let compressed_data = LZ77::compress(input_data, max_dict_size, lookahead_buffer_size);
    ///
    /// // Vérifie que les données décompressées correspondent aux attentes.
    /// let decompressed_data = LZ77::decompress(&compressed_data);
    /// assert_eq!(decompressed_data, input_data);
    /// ```
    pub fn compress(
        raw_data: &[u8],
        max_dictionary_size: usize,
        lookahead_buffer_size: usize,
    ) -> Vec<u8> {
        // la longueur de la donnée
        let raw_data_length: usize = raw_data.len();
        // contient la donnée compresser
        let mut compressed_data: Vec<u8> = Vec::new();
        // stocke le carractere dans l'encodage
        let mut next_character: u8;
        // la taille du dictionnaire de recherche
        let search_buffer_length: usize = max_dictionary_size; 
        // le curseur dans le dictionnaire de recherche
        let mut search_buffer_index: usize;
        // la taille du tampon de recherche
        let ahead_buffer_length: usize = lookahead_buffer_size;
        // le curseur dans le tampon de recherche
        let mut ahead_buffer_index: usize;
        let mut cursor: usize;
        let mut length: usize;
        let mut i: usize = 0;
        while i < raw_data_length {
            // le max entre et 0
            search_buffer_index = std::cmp::max(i.saturating_sub(search_buffer_length), 0);
            ahead_buffer_index = std::cmp::min(i + ahead_buffer_length, raw_data_length);

            (cursor, length) = find_longest_match(
                &raw_data[search_buffer_index..i],
                &raw_data[i..ahead_buffer_index],
            );

            if i + length >= raw_data_length {
                next_character = 0;
            } else {
                next_character = raw_data[i + length];
            }
            LZ77::encode(&mut compressed_data, cursor, length, next_character);

            i += length + 1;
        }

        compressed_data
    }

    /// Décompresse les données compressées en utilisant l'algorithme LZ77.
    ///
    /// # Arguments
    ///
    /// * `compressed_data` - Données compressées à décompresser.
    ///
    /// # Returns
    ///
    /// Un vecteur contenant les données décompressées.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::LZ77;
    ///
    /// // Données brutes à compresser.
    /// let input_data = b"hello world";
    ///
    /// // Taille maximale du dictionnaire de recherche.
    /// let max_dict_size = 4095;
    ///
    /// // Taille du tampon de recherche.
    /// let lookahead_buffer_size = 15;
    ///
    /// // Compresse les données brutes.
    /// let compressed_data = LZ77::compress(input_data, max_dict_size, lookahead_buffer_size);
    ///
    /// // Vérifie que les données décompressées correspondent aux attentes.
    /// let decompressed_data = LZ77::decompress(&compressed_data);
    /// assert_eq!(decompressed_data, input_data);
    /// ```
    pub fn decompress(compressed_data: &[u8]) -> Vec<u8> {
        let compressed_data_length: usize = compressed_data.len();
        let mut raw_data: Vec<u8> = Vec::new();

        for i in (0..compressed_data_length).step_by(3) {
            LZ77::decode(&mut raw_data, &compressed_data[i..i + 3]);
        }
        if raw_data.len() > 0 && raw_data[raw_data.len() - 1] == 0 {
            return raw_data[..raw_data.len() - 1].to_vec();
        }
        raw_data
    }
}

impl Clone for LZ77 {
    fn clone(&self) -> Self {
        LZ77 {
            max_dictionary_size: self.get_max_dictionary_size(),
            lookahead_buffer_size: self.get_lookahead_buffer_size(),
        }
    }
}

impl LZ for LZ77 {
    /// Compresse les données brutes en utilisant l'algorithme LZ77.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Données brutes à compresser.
    ///
    /// # Returns
    ///
    /// Un vecteur contenant les données compressées
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::{LZ77, LZ};
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let lz77 = LZ77::new();
    ///
    /// // Données brutes à compresser.
    /// let input_data = b"hello world";
    ///
    /// // Compresse les données brutes à l'aide de la structure LZ77.
    /// let compressed_data = lz77.compress(input_data);
    ///
    /// // Vérifie que les données décompressées correspondent aux attentes.
    /// let decompressed_data = lz77.decompress(&compressed_data);
    /// assert_eq!(decompressed_data, input_data);
    /// ```
    fn compress(&self, raw_data: &[u8]) -> Vec<u8> {
        LZ77::compress(
            raw_data,
            self.get_max_dictionary_size(),
            self.get_lookahead_buffer_size(),
        )
    }

    /// Décompresse les données compressées en utilisant l'algorithme LZ77.
    ///
    /// # Arguments
    ///
    /// * `compressed_data` - Données compressées à décompresser.
    ///
    /// # Returns
    ///
    /// Un vecteur contenant les données décompressées.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lzrs::{LZ77, LZ};
    ///
    /// // Crée une nouvelle instance de LZ77.
    /// let lz77 = LZ77::new();
    ///
    /// // Données brutes à compresser.
    /// let input_data = b"hello world";
    ///
    /// // Compresse les données brutes à l'aide de la structure LZ77.
    /// let compressed_data = lz77.compress(input_data);
    ///
    /// // Vérifie que les données décompressées correspondent aux attentes.
    /// let decompressed_data = lz77.decompress(&compressed_data);
    /// assert_eq!(decompressed_data, input_data);
    /// ```
    fn decompress(&self, compressed_data: &[u8]) -> Vec<u8> {
        LZ77::decompress(compressed_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_and_decompression() {
        // Crée une nouvelle instance de LZ77.
        let lz77 : LZ77 = LZ77::new();

        // Phrase à compresser.
        let phrase : &[u8; 41] = b"Une phrase d'exemple Une phrase d'exemple";

        // Compresse la phrase.
        let compressed_data : Vec<u8> = lz77.compress(phrase);

        // Décompresse les données compressées.
        let decompressed_data : Vec<u8> = lz77.decompress(&compressed_data);

        // Vérifie que les données décompressées correspondent à la phrase d'origine.
        assert_eq!(decompressed_data, phrase);
    }
}
