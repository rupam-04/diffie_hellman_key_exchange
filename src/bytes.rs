pub fn bytes_to_binary(bytes: &[u8], binaries: &mut Vec<u8>) {
    for byte in bytes.iter() {
        let mut b = *byte;
        for _ in 0..8 {
            let bit = b & 0x80;
            if bit > 0 {
                binaries.push(1);
            }
            else {
                binaries.push(0);
            }
            b <<= 1;
        }
    }
}