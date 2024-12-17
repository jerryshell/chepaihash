#[derive(Debug)]
pub enum ChepaiError {
    RandomGeneration,
    IndexOutOfBounds,
}

impl core::fmt::Display for ChepaiError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ChepaiError::RandomGeneration => write!(f, "随机数生成失败"),
            ChepaiError::IndexOutOfBounds => write!(f, "字符索引越界"),
        }
    }
}

impl core::error::Error for ChepaiError {}
