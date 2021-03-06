use rand;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Dice(u8);

impl Dice {
    const N: usize = 6;
    const P: u32 = 2;
    pub fn new() -> Self {
        let r: usize = rand::random();
        Dice((r % Self::N.pow(Self::P)) as u8)
    }
    pub fn shuffle(&mut self) {
        *self = Self::new()
    }
    pub fn raw(&self) -> usize {
        self.0 as usize
    }
    pub fn get(&self, i: u32) -> usize {
        self.raw() / Self::N.pow(i) % Self::N + 1
    }
    pub fn sum(&self) -> usize {
        (0..Self::P).map(|i| self.get(i)).sum()
    }
}

#[test]
fn dice_range() {
    for _ in 0..100 {
        let dice = Dice::new();
        let (a,b) = (dice.get(0), dice.get(1));
        assert!(1 <= a && a <= 6);
        assert!(1 <= b && b <= 6);
    }
}