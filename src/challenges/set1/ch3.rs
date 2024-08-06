#![allow(dead_code)]
use crate::core::FixedXor;

fn break_single_xor_cipher(hex_encrypted: &str) -> Vec<u8> {
    let decoded = hex::decode(hex_encrypted).unwrap();
    let total_len = decoded.len();
    let mut best_key = Vec::<u8>::new();
    let mut best_divergence = f32::INFINITY;

    (0..u8::MAX)
        .map(|byte| vec![byte; total_len])
        .for_each(|key| {
            let decrypted_bytes = decoded.fixed_xor_with(&key).unwrap();
            let divergence = score_decryption(&decrypted_bytes, &ENGLISH_CHAR_FREQUENCIES);
            if divergence < best_divergence {
                best_divergence = divergence;
                best_key = key;
            }
        });

    best_key
}

fn score_decryption(decrypted_bytes: &[u8], ref_frequencies: &[(u8, f32); 28]) -> f32 {
    let decrypted_freq = calculate_frequencies(decrypted_bytes);
    let divergence = decrypted_freq
        .iter()
        .zip(ref_frequencies)
        .fold(0f32, |acc, ((_, freq), (_, ref_freq))| {
            acc + (freq - ref_freq).powi(2)
        });

    divergence
}

fn calculate_frequencies(bytes: &[u8]) -> [(u8, f32); 28] {
    let mut frequencies = EMPTY_FREQUENCIES.clone();
    let total_len = bytes.len() as f32;

    bytes.iter().for_each(|&byte| {
        if byte >= b'a' && byte <= b'z' {
            let idx = (byte - b'a') as usize;
            frequencies[idx].1 += 1.0;
        } else if byte == b' ' {
            frequencies[26].1 += 1.0;
        } else {
            frequencies[27].1 += 1.0;
        }
    });

    frequencies
        .iter_mut()
        .for_each(|(_, freq)| *freq /= total_len);

    frequencies
}

#[cfg(test)]
mod tests {
    use crate::core::FixedXor;

    use super::break_single_xor_cipher;

    #[test]
    fn single_character_xor_cipher() {
        let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let key = break_single_xor_cipher(encrypted);
        let decrypted_bytes = hex::decode(encrypted)
            .unwrap()
            .fixed_xor_with(&key)
            .unwrap();

        let decrypted = String::from_utf8_lossy(&decrypted_bytes);

        assert_eq!(decrypted, "Cooking MC's like a pound of bacon");
    }
}

static ENGLISH_CHAR_FREQUENCIES: [(u8, f32); 28] = [
    (b'a', 0.082),
    (b'b', 0.015),
    (b'c', 0.028),
    (b'd', 0.043),
    (b'e', 0.13),
    (b'f', 0.022),
    (b'g', 0.02),
    (b'h', 0.061),
    (b'i', 0.07),
    (b'j', 0.0015),
    (b'k', 0.0077),
    (b'l', 0.04),
    (b'm', 0.024),
    (b'n', 0.067),
    (b'o', 0.075),
    (b'p', 0.019),
    (b'q', 0.00095),
    (b'r', 0.06),
    (b's', 0.063),
    (b't', 0.091),
    (b'u', 0.028),
    (b'v', 0.0098),
    (b'w', 0.024),
    (b'x', 0.0015),
    (b'y', 0.02),
    (b'z', 0.00074),
    (b' ', 0.13),
    (b'\0', 0.0),
];

static EMPTY_FREQUENCIES: [(u8, f32); 28] = [
    (b'a', 0f32),
    (b'b', 0f32),
    (b'c', 0f32),
    (b'd', 0f32),
    (b'e', 0f32),
    (b'f', 0f32),
    (b'g', 0f32),
    (b'h', 0f32),
    (b'i', 0f32),
    (b'j', 0f32),
    (b'k', 0f32),
    (b'l', 0f32),
    (b'm', 0f32),
    (b'n', 0f32),
    (b'o', 0f32),
    (b'p', 0f32),
    (b'q', 0f32),
    (b'r', 0f32),
    (b's', 0f32),
    (b't', 0f32),
    (b'u', 0f32),
    (b'v', 0f32),
    (b'w', 0f32),
    (b'x', 0f32),
    (b'y', 0f32),
    (b'z', 0f32),
    (b' ', 0f32),
    (b'\0', 0f32), // all the rest
];
