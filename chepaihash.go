package main

import (
	"fmt"
	"os"
)

const (
	PROVINCE    = "黑吉辽京津晋冀鲁豫蒙沪渝苏浙皖闽湘赣鄂桂琼川贵云藏陕甘宁青新粤"
	ALPHABET    = "ABCDEFGHJKLMNPQRSTUVWXYZ"
	ALPHANUMERIC = "ABCDEFGHJKLMNPQRSTUVWXYZ0123456789"
)

type LinearCongruentialRng struct {
	seed uint64
}

func NewLinearCongruentialRng(seed uint64) *LinearCongruentialRng {
	return &LinearCongruentialRng{seed: seed}
}

func (rng *LinearCongruentialRng) Next() uint64 {
	const LCG_A = 6364136223846793005
	const LCG_C = 1
	rng.seed = LCG_A*rng.seed + LCG_C
	return rng.seed
}

func hash(value string) string {
	seed := uint64(0)
	for _, c := range value {
		seed = seed*31 + uint64(c)
	}

	rng := NewLinearCongruentialRng(seed)

	chepai := make([]rune, 8)
	chepai[0] = getRandomChar(rng, PROVINCE)
	chepai[1] = getRandomChar(rng, ALPHABET)
	chepai[2] = '·'
	for i := 0; i < 5; i++ {
		chepai[3+i] = getRandomChar(rng, ALPHANUMERIC)
	}

	return string(chepai)
}

func getRandomChar(rng *LinearCongruentialRng, chars string) rune {
	index := rng.Next() % uint64(len(chars))
	return rune(chars[index])
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("请提供一个字符串参数")
		return
	}
	arg := os.Args[1]
	chepai := hash(arg)
	fmt.Println(chepai)
}
