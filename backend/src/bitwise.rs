pub fn test_bit(data: u8, mask: u8) -> bool {
    data & mask > 0
}

pub fn set_bit_on(data: &mut u8, mask: u8) {
    *data |= mask
}

pub fn set_bit_off(data: &mut u8, mask: u8) {
    *data &= !mask
}

pub fn set_bit_dynamic(data: &mut u8, mask: u8, is_on: bool) {
    (if is_on { set_bit_on } else { set_bit_off })(data, mask)
}

pub fn get_pair(hi: u8, lo: u8) -> u16 {
    (hi as u16) << 8 | (lo as u16)
}

pub fn set_pair(pair: u16) -> (u8, u8) {
    ((pair >> 8) as u8, pair as u8)
}
