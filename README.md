# CodeWars-Toggle-Set-and-Clear-Bits-Bit-Manipulation-Basics--7-kyu---Passed
Toggle, Set, and Clear Bits
Your task is to implement a collection of utility functions that perform common bitwise operations on integers. All bit positions are zero-based, meaning position 0 refers to the least significant bit.

Toggles (flips) the bit at the given position. If the bit is 0, it becomes 1; if it is 1, it becomes 0.

toggleBit(5, 1) => 7
Sets the bit at the given position to 1, without modifying other bits.

setBit(5, 1) => 7
Clears (sets to 0) the bit at the given position, leaving all other bits unchanged.

clearBit(7, 1) => 5
Checks whether the bit at the given position is set to 1. Returns true if it is set, otherwise false.

isBitSet(5, 0) => true
Sets all bits to 1 wherever the mask has 1s.

setMultipleBits(5, 3) => 7
Clears all bits wherever the mask has 1s.

clearMultipleBits(7, 2) => 5
Toggles all bits wherever the mask has 1s.

toggleMultipleBits(5, 3) => 6
Notes
All functions should return the resulting number (or a boolean for isBitSet).

TEST CASES:
#[cfg(test)]
mod tests {
    use super::{
        clear_bit, clear_multiple_bits, is_bit_set, set_bit, set_multiple_bits, toggle_bit,
        toggle_multiple_bits,
    };

    #[test]
    fn _1_test_toggle_bit() {
        assert_eq!(toggle_bit(5, 1), 7);
    }
    #[test]
    fn _2_test_set_bit() {
        assert_eq!(set_bit(5, 1), 7);
    }
    #[test]
    fn _3_test_clear_bit() {
        assert_eq!(clear_bit(7, 1), 5);
    }
    #[test]
    fn _4_test_is_bit_set() {
        assert_eq!(is_bit_set(5, 0), true);
    }
    #[test]
    fn _5_test_set_multiple_bits() {
        assert_eq!(set_multiple_bits(5, 3), 7);
    }
    #[test]
    fn _6_test_clear_multiple_bits() {
        assert_eq!(clear_multiple_bits(7, 2), 5);
    }
    #[test]
    fn _7_test_toggle_multiple_bits() {
        assert_eq!(toggle_multiple_bits(5, 3), 6);
    }

    static TOGGLE: fn(u32, u32) -> u32 = |n, mask| n ^ mask;
    static SET: fn(u32, u32) -> u32 = |n, mask| n | mask;
    static CLEAR: fn(u32, u32) -> u32 = |n, mask| n & !mask;
    static TOGGLE_BIT: fn(u32, u32) -> u32 = |n, pos| TOGGLE(n, 1 << pos);
    static SET_BIT: fn(u32, u32) -> u32 = |n, pos| SET(n, 1 << pos);
    static CLEAR_BIT: fn(u32, u32) -> u32 = |n, pos| CLEAR(n, 1 << pos);
    static IS_BIT_SET: fn(u32, u32) -> bool = |n, pos| (n >> pos) & 1 == 1;

    #[test]
    fn _r1_random_test_toggle_bit() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let pos = rng.gen_range(0..32);
            let expected = TOGGLE_BIT(n, pos);
            let actual = toggle_bit(n, pos);
            assert_eq!(
                actual, expected,
                "toggle_bit({n}, {pos}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r2_random_test_set_bit() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let pos = rng.gen_range(0..32);
            let expected = SET_BIT(n, pos);
            let actual = set_bit(n, pos);
            assert_eq!(
                actual, expected,
                "set_bit({n}, {pos}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r3_random_test_clear_bit() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let pos = rng.gen_range(0..32);
            let expected = CLEAR_BIT(n, pos);
            let actual = clear_bit(n, pos);
            assert_eq!(
                actual, expected,
                "clear_bit({n}, {pos}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r4_random_test_is_bit_set() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let pos = rng.gen_range(0..32);
            let expected = IS_BIT_SET(n, pos);
            let actual = is_bit_set(n, pos);
            assert_eq!(
                actual, expected,
                "is_bit_set({n}, {pos}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r5_random_test_set_multiple_bits() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let mask = rng.gen();
            let expected = SET(n, mask);
            let actual = set_multiple_bits(n, mask);
            assert_eq!(
                actual, expected,
                "set_multiple_bits({n}, {mask}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r6_random_test_clear_multiple_bits() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let mask = rng.gen();
            let expected = CLEAR(n, mask);
            let actual = clear_multiple_bits(n, mask);
            assert_eq!(
                actual, expected,
                "clear_multiple_bits({n}, {mask}) should be {expected}, but got {actual}"
            );
        }
    }
    #[test]
    fn _r7_random_test_toggle_multiple_bits() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen();
            let mask = rng.gen();
            let expected = TOGGLE(n, mask);
            let actual = toggle_multiple_bits(n, mask);
            assert_eq!(
                actual, expected,
                "toggle_multiple_bits({n}, {mask}) should be {expected}, but got {actual}"
            );
        }
    }
}
