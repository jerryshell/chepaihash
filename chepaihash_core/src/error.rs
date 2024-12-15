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
