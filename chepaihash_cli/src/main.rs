fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().nth(1).expect("请提供一个字符串参数");
    let chepai = chepaihash_core::hash(&arg)?;
    println!("{}", chepai);
    Ok(())
}
