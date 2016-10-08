pub fn shift_bytes_left(data: &mut Vec<u8>, shift: usize) {
    if shift == 0 {
        return;
    }
    let mask = !(0xFF >> shift);
    let mut frag: u8;
    if data.len() < 1 {
        return;
    }
    data[0] <<= shift;
    for i in 1..data.len() {
        frag = data[i] & mask;
        data[i] <<= shift;
        data[i - 1] |= frag >> (8 - shift);
    }
}
