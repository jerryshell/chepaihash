# ChepaiHash

_ChepaiHash_ 是一个将字符串转换为中国车牌号的哈希工具

## 作为 Cli 使用

```bash
go install github.com/jerryshell/chepaihash
```

```bash
chepaihash helloworld
# 赣Y·H45YP
```

## 作为 Lib 使用

1. `go.mod`

```go
module github.com/jerryshell/chepaihash

go 1.16
```

2. `main.go`

```go
package main

import (
	"fmt"
	"github.com/jerryshell/chepaihash"
)

func main() {
	chepai := chepaihash.Hash("helloworld")
	fmt.Println(chepai)
	// 赣Y·H45YP
}
```

## 为什么

我需要一个可读性强、易于识别的哈希值来匿名化数据

## 相关项目

- [cunzaizhuyi/hashplate-cn](https://github.com/cunzaizhuyi/hashplate-cn)
- [hugoattal/hashplate](https://github.com/hugoattal/hashplate)

## 开源协议

[MIT](LICENSE)
