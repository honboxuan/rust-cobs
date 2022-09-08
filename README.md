# rust-cobs

Simple Rust implementation of [Consistent Overhead Byte Stuffing](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing).

When decoding a stream of bytes, the `Cobs` struct stores unused bytes until a zero byte, the packet delimiter, is received. The stored bytes are then decoded. A `Vec` of successfully decoded packets are returned, or `None` if no packets are decoded. Malformed packets are discarded without warning.

## Usage

### Encoding

```rust
let encoded = cobs::Cobs::encode(&[0x11, 0x00, 0x00, 0x00]);
assert_eq!(encoded, vec![0x02, 0x11, 0x01, 0x01, 0x01, 0x00]);
```

### Decoding

```rust
let mut cobs = cobs::Cobs::default();
let decoded_1 = cobs.decode(&[0x02, 0x11, 0x01]);
assert_eq!(decoded_1, None); // Decoding is incomplete
let decoded_2 = cobs.decode(&[0x01, 0x01, 0x00]);
assert_eq!(decoded_2, Some(vec![vec![0x11, 0x00, 0x00, 0x00]])); // One packet decoded
```
