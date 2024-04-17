use crate::U8bits;


pub struct State {
    mem: [U8bits; 0xff]
}

impl State {
    pub fn new() -> State {
        State {
            mem: [0; 0xff] // todo: make sure reset vector indeed results in this
        }
    }
}