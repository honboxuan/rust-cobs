use cobs::Cobs;

#[test]
fn test_encode_decode_01() {
    let input: Vec<u8> = (0u8..255u8).collect();
    let encoded = Cobs::encode(input.as_slice());
    let mut cobs = Cobs::default();
    let output = cobs.decode(encoded.as_slice()).unwrap();
    assert_eq!(vec![input], output);
}

#[test]
fn test_encode_decode_02() {
    let input: Vec<u8> = (0u8..=255u8).collect();
    let encoded = Cobs::encode(input.as_slice());
    let mut cobs = Cobs::default();
    let output = cobs.decode(encoded.as_slice()).unwrap();
    assert_eq!(vec![input], output);
}

#[test]
fn test_encode_decode_03() {
    let input: Vec<u8> = (0u8..=255u8).chain(0u8..=255u8).collect();
    let encoded = Cobs::encode(input.as_slice());
    let mut cobs = Cobs::default();
    let output = cobs.decode(encoded.as_slice()).unwrap();
    assert_eq!(vec![input], output);
}
