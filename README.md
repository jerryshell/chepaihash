# ChepaiHash

_ChepaiHash_ 是一个将数据转换为中国车牌号的哈希工具

## 作为 Cli 使用

```bash
cargo install --locked --git https://github.com/jerryshell/chepaihash
```

```bash
chepaihash helloworld
# 赣Y·H45YP
```

## 作为 Lib 使用（no_std）

1. `Cargo.toml`

```toml
[dependencies]
chepaihash_core = { git = "https://github.com/jerryshell/chepaihash" }
```

2. `main.rs`

```rust
let data = "helloworld".as_bytes();
let chepai = chepaihash_core::hash(data)?;
let chepai_str = chepai.iter().collect::<String>();
println!("{}", chepai_str);
// 赣Y·H45YP
```

## 为什么

我需要一个可读性强、易于识别的哈希值来匿名化数据

## 相关项目

- [cunzaizhuyi/hashplate-cn](https://github.com/cunzaizhuyi/hashplate-cn)
- [hugoattal/hashplate](https://github.com/hugoattal/hashplate)

## 开源协议

[MIT](LICENSE)
