#[cfg(test)]
mod tests {
    use crate::core::FixedXor;

    #[test]
    fn fixed_xor() {
        let str1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let str2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

        let res = str1.fixed_xor_with(&str2).unwrap();
        assert_eq!(res, hex::decode("746865206b696420646f6e277420706c6179").unwrap())
    }
}
