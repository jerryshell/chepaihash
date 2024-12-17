const DEFAULT_SEED: usize = 996;
const LCG_A: usize = 6364136223846793005;
const LCG_C: usize = 1;

pub struct LinearCongruentialGenerator {
    pub seed: usize,
}

impl Default for LinearCongruentialGenerator {
    fn default() -> Self {
        Self { seed: DEFAULT_SEED }
    }
}

impl Iterator for LinearCongruentialGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = LCG_A.wrapping_mul(self.seed).wrapping_add(LCG_C);
        Some(self.seed)
    }
}

impl LinearCongruentialGenerator {
    pub fn new(seed: usize) -> Self {
        Self { seed }
    }
}
