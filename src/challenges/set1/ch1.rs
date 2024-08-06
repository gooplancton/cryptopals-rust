#[cfg(test)]
mod tests {
    use base64::Engine;

    #[test]
    fn convert_hex_to_base64() {
        let encoded = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let decoded = hex::decode(encoded).unwrap();
        let b64encoded = base64::engine::general_purpose::STANDARD.encode(decoded);
        assert_eq!(b64encoded, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}

