fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().nth(1).expect("请提供一个字符串参数");
    let data = arg.as_bytes();
    let chepai = chepaihash_core::hash(data)?;
    let chepai_str = chepai.iter().collect::<String>();
    println!("{}", chepai_str);
    Ok(())
}
