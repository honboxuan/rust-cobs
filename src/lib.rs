use std::collections::VecDeque;

/// Consistent Overhead Byte Stuffing
///
/// `bytes` stores incomplete packets, queued for decoding.
#[derive(Default)]
pub struct Cobs {
    bytes: VecDeque<u8>,
}

impl Cobs {
    /// Encodes packets using COBS.
    ///
    /// Returns encoded packet, including trailing zero byte.
    ///
    /// # Examples
    ///
    /// ```
    /// let encoded = cobs::Cobs::encode(&[0x11, 0x00, 0x00, 0x00]);
    /// assert_eq!(encoded, vec![0x02, 0x11, 0x01, 0x01, 0x01, 0x00]);
    /// ```
    pub fn encode(bytes: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        for split in bytes.split(|v| *v == 0) {
            if split.is_empty() {
                output.push(1);
            } else {
                for chunk in split.chunks(254) {
                    output.push(chunk.len() as u8 + 1);
                    output.extend_from_slice(chunk);
                }
            }
        }
        output.push(0);
        output
    }

    /// Decodes COBS encoded packet(s).
    ///
    /// Incomplete packet bytes will be stored and processed in subsequent calls.
    /// Returns Vec of decoded packets, or `None` if no packets are decoded.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cobs = cobs::Cobs::default();
    /// let decoded_1 = cobs.decode(&[0x02, 0x11, 0x01]);
    /// assert_eq!(decoded_1, None); // Decoding is incomplete
    /// let decoded_2 = cobs.decode(&[0x01, 0x01, 0x00]);
    /// assert_eq!(decoded_2, Some(vec![vec![0x11, 0x00, 0x00, 0x00]])); // One packet decoded
    /// ```
    pub fn decode(&mut self, bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
        self.bytes.extend(bytes);

        let mut outputs: Option<Vec<Vec<u8>>> = None;
        while let Some(position) = self.bytes.iter().position(|&v| v == 0) {
            let mut packet = self.bytes.drain(..=position).collect::<VecDeque<u8>>();
            let mut output: Vec<u8> = vec![];
            while let Some(length) = packet.pop_front() {
                if length == 0 || packet.len() < length as usize {
                    // Unexpected zero, or packet length too short: Terminate
                    break;
                }
                if length > 1 {
                    let range = ..length as usize - 1;
                    if packet.range(range).any(|&byte| byte == 0) {
                        // Unexpected zero: terminate
                        break;
                    } else {
                        // Move `length - 1` unaltered, non-zero bytes
                        output.extend(packet.drain(range));
                    }
                }
                if *packet.front().unwrap() == 0 {
                    // End of packet
                    if let Some(outputs) = outputs.as_mut() {
                        outputs.push(output);
                    } else {
                        outputs = Some(vec![output]);
                    }
                    break;
                } else if length != 0xFF {
                    // Altered byte
                    output.push(0);
                }
            }
        }
        outputs
    }
}
