#![no_std]

pub mod error;
pub mod rng;

const PROVINCE: &str = "黑吉辽京津晋冀鲁豫蒙沪渝苏浙皖闽湘赣鄂桂琼川贵云藏陕甘宁青新粤";
const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const ALPHANUMERIC: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ0123456789";

pub fn hash(data: &[u8]) -> Result<[char; 8], error::ChepaiError> {
    let seed = data.iter().fold(0usize, |acc, x| {
        acc.wrapping_mul(31usize).wrapping_add(*x as usize)
    });

    let mut rng = rng::LinearCongruentialGenerator::new(seed);

    let mut chepai = ['\0'; 8];
    chepai[0] = get_random_char(&mut rng, PROVINCE)?;
    chepai[1] = get_random_char(&mut rng, ALPHABET)?;
    chepai[2] = '·';
    for i in 0..5 {
        chepai[3 + i] = get_random_char(&mut rng, ALPHANUMERIC)?;
    }

    Ok(chepai)
}

fn get_random_char(
    rng: &mut impl Iterator<Item = usize>,
    chars: &str,
) -> Result<char, error::ChepaiError> {
    let index = rng.next().ok_or(error::ChepaiError::RandomGeneration)? % chars.chars().count();
    chars
        .chars()
        .nth(index)
        .ok_or(error::ChepaiError::IndexOutOfBounds)
}
