// Checks if the bit index is 1 in byte. Index starts at 0
#[inline]
#[allow(dead_code)]
pub fn is_bit_set(byte: &u8, bit: u8) -> bool {
  byte & (1 << bit) != 0
}

// Checks if the bit index is 0 in byte. Index starts at 0
#[inline]
pub fn is_bit_unset(byte: &u8, bit: u8) -> bool {
  byte & (1 << bit) == 0
}

// Set the bit index in byte to 1. Index starts at 0
#[inline]
#[allow(dead_code)]
pub fn set_bit(byte: &mut u8, bit: u8) {
  *byte |= 1 << bit;
}

// Set the bit index in byte to 0. Index starts at 0
#[inline]
pub fn unset_bit(byte: &mut u8, bit: u8) {
  *byte &= !(1 << bit);
} 

// Count how many 1 bits are there in byte
#[inline]
pub fn count_set_bits(byte: &u8) -> u8 {
  byte.count_ones() as u8
}

// Count how many 0 bits are there in byte
#[inline]
#[allow(dead_code)]
pub fn count_unset_bits(byte: &u8) -> u8 {
  byte.count_zeros() as u8
}

// Set the last (most significant) n bits in byte to 0.
#[inline]
pub fn unset_last_bits(byte: &mut u8, bit: u8) {
  *byte &= 0xff >> bit;
}