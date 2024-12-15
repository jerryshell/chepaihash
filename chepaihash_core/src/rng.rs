const DEFAULT_SEED: usize = 996;
const LCG_A: usize = 6364136223846793005;
const LCG_C: usize = 1;

pub struct LinearCongruentialRng {
    pub seed: usize,
}

impl Default for LinearCongruentialRng {
    fn default() -> Self {
        Self { seed: DEFAULT_SEED }
    }
}

impl Iterator for LinearCongruentialRng {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = LCG_A.wrapping_mul(self.seed).wrapping_add(LCG_C);
        Some(self.seed)
    }
}

impl LinearCongruentialRng {
    pub fn new(seed: usize) -> Self {
        Self { seed }
    }
}
