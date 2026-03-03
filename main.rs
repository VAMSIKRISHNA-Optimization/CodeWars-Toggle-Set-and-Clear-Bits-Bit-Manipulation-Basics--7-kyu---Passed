fn toggle_bit(n: u32, pos: u32) -> u32 {
    n ^ (1 << pos)
}
fn set_bit(n: u32, pos: u32) -> u32 {
    n | (1 << pos)
}
fn clear_bit(n: u32, pos: u32) -> u32 {
    n & !(1 << pos)
}
fn is_bit_set(n: u32, pos: u32) -> bool {
    (n >> pos) & 1 == 1
}
fn set_multiple_bits(n: u32, mask: u32) -> u32 {
    n | mask
}
fn clear_multiple_bits(n: u32, mask: u32) -> u32 {
    n & !mask
}
fn toggle_multiple_bits(n: u32, mask: u32) -> u32 {
    n ^ mask
}
