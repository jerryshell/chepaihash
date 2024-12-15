pub mod rng;

const PROVINCE: &str = "黑吉辽京津晋冀鲁豫蒙沪渝苏浙皖闽湘赣鄂桂琼川贵云藏陕甘宁青新粤";
const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const ALPHANUMERIC: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ0123456789";

#[derive(Debug)]
pub enum PlateError {
    RandomGeneration,
    IndexOutOfBounds,
}

impl core::fmt::Display for PlateError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PlateError::RandomGeneration => write!(f, "随机数生成失败"),
            PlateError::IndexOutOfBounds => write!(f, "字符索引越界"),
        }
    }
}

impl core::error::Error for PlateError {}

pub fn hash(value: &str) -> Result<String, PlateError> {
    let seed = get_seed_from_string(value);
    let mut rng = rng::LinearCongruentialRng::new(seed);

    let chepai = format!(
        "{}{}·{}",
        get_random_char(&mut rng, PROVINCE)?,
        get_random_char(&mut rng, ALPHABET)?,
        generate_plate_number(&mut rng)?
    );

    Ok(chepai)
}

pub fn get_seed_from_string(value: &str) -> usize {
    let mut seed = 0usize;
    for c in value.chars() {
        seed = seed.wrapping_mul(31_usize).wrapping_add(c as usize);
    }
    seed
}

fn get_random_char(rng: &mut rng::LinearCongruentialRng, chars: &str) -> Result<char, PlateError> {
    let index = rng.next().ok_or(PlateError::RandomGeneration)? % chars.chars().count();
    chars.chars().nth(index).ok_or(PlateError::IndexOutOfBounds)
}

fn generate_plate_number(rng: &mut rng::LinearCongruentialRng) -> Result<String, PlateError> {
    (0..5).map(|_| get_random_char(rng, ALPHANUMERIC)).collect()
}
