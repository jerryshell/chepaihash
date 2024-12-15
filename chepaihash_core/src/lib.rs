#![no_std]

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

pub fn hash(value: &str) -> Result<[char; 8], PlateError> {
    let seed = value.chars().fold(0usize, |acc, c| {
        acc.wrapping_mul(31usize).wrapping_add(c as usize)
    });

    let mut rng = rng::LinearCongruentialRng::new(seed);

    let mut chepai = ['\0'; 8];
    chepai[0] = get_random_char(&mut rng, PROVINCE)?;
    chepai[1] = get_random_char(&mut rng, ALPHABET)?;
    chepai[2] = '·';
    for i in 0..5 {
        chepai[3 + i] = get_random_char(&mut rng, ALPHANUMERIC)?;
    }

    Ok(chepai)
}

fn get_random_char(rng: &mut rng::LinearCongruentialRng, chars: &str) -> Result<char, PlateError> {
    let index = rng.next().ok_or(PlateError::RandomGeneration)? % chars.chars().count();
    chars.chars().nth(index).ok_or(PlateError::IndexOutOfBounds)
}
