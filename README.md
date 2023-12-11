# LZRS

lzrs est une bibliothèque Rust proposant des implémentations des algorithmes de compression et décompression LZ.

## Fonctionnalités
[x] Compression de données en utilisant l'algorithme LZ77.      
[x] Décompression des données compressées via LZ77.     
[ ] Compression d'un fichier en utilisant l'algorithme LZ77.        
[ ] Décompression d'un fichier en utilisant l'algorithme LZ77. 

## Installation
Pour utiliser lzrs dans votre projet, ajoutez la dépendance suivante à votre fichier Cargo.toml : (pas encore publié)
```toml
[dependencies]
lzrs = "0.1.0"
```
## Example

```rust
use lzrs::{LZ, LZ77};

fn main() {
    // Crée une nouvelle instance de LZ77.
    let lz77 = LZ77::new();

    // Données brutes à compresser.
    let input_data = b"hello world";

    // Compresse les données brutes à l'aide de la structure LZ77.
    let compressed_data = lz77.compress(input_data);

    // Vérifie que les données décompressées correspondent aux attentes.
    let decompressed_data = lz77.decompress(&compressed_data);
    assert_eq!(decompressed_data, input_data);
}

```
Pour plus d'exemples d'utilisation, consultez la documentation, les tests et [les fichiers d'exemple](/exemples/).


## Contributions
Les contributions sont les bienvenues ! Si vous souhaitez améliorer ou ajouter des fonctionnalités à Huffmanrs, veuillez ouvrir une pull request sur GitHub.

## License
Ce projet est sous [``licence MIT``](LICENSE). Veuillez consulter le fichier [``LICENSE``](LICENSE) pour plus d'informations.