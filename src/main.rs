fn main() {
    let ahead_sequence = "abcde12345";
    let window_size = 3; // Taille de la fenêtre

    for i in 0..=ahead_sequence.len() - window_size {
        let window = &ahead_sequence[i..i + window_size];
        let rposition = ahead_sequence.len() - i - window_size;
        println!("Window: {} \t Position: {} \t RPosition: {}", window, i, rposition);
    }
}
