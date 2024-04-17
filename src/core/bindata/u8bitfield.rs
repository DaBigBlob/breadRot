
pub enum U8idx {
    N0 = 0,
    N1 = 1,
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7
}

pub fn get(int: u8, idx: U8idx) -> bool {
    let mask = 1 << idx as u8;
    int & mask == mask
}

pub fn set(int: &mut u8, idx: U8idx) {
    let mask = 1 << idx as u8;
    *int |= mask;
}