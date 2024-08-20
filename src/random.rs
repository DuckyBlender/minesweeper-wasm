// XORShift algorithm to generate pseudorandom numbers
// https://en.wikipedia.org/wiki/Xorshift
pub struct XORShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XORShift {
    pub fn new() -> XORShift {
        XORShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn next(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}
