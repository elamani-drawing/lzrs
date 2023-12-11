use lzrs::{LZ, LZ77};

fn main() {
    // Crée une nouvelle instance de LZ77.
    let lz77 = LZ77::new();

    // Phrase à compresser.
    let phrase = b"Une phrase d'exemple Une phrase d'exemple";

    // Compresse la phrase.
    let compressed_data = lz77.compress(phrase);

    // Décompresse les données compressées.
    let decompressed_data = lz77.decompress(&compressed_data);

    // Affiche la phrase décompressée.
    let decoded_str = String::from_utf8_lossy(&decompressed_data);
    println!("Phrase décompressée : {}", decoded_str);

    // Vérifie que les données décompressées correspondent à la phrase d'origine.
    assert_eq!(decompressed_data, phrase);
}
