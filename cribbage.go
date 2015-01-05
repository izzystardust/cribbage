package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	if len(os.Args) == 1 {
		fmt.Println("bad args")
		return
	}
	handRaw := make([]string, len(os.Args)-1)
	copy(handRaw, os.Args[1:])

	startCard := Card{
		suit: ToSuit(pop(&handRaw)),
		rank: ToRank(pop(&handRaw)),
	}

	var hand Hand
	for len(handRaw) > 0 {
		hand = append(hand, Card{
			suit: ToSuit(pop(&handRaw)),
			rank: ToRank(pop(&handRaw)),
		})
	}
	fmt.Println(startCard)
	fmt.Println(hand)

}

type Hand []Card

func (h Hand) Len() int           { return len(h) }
func (h Hand) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h Hand) Less(i, j int) bool { return h[i].rank < h[j].rank }

func (h Hand) String() string {
	var ret string
	for _, c := range h {
		ret += c.String() + " "
	}
	return ret
}

type Card struct {
	rank Rank
	suit Suit
}

func (c Card) String() string {
	return fmt.Sprintf("%v%v", c.rank, c.suit)
}

type Rank int

func (r Rank) String() string {
	switch r {
	case 13:
		return "K"
	case 12:
		return "Q"
	case 11:
		return "J"
	default:
		return strconv.Itoa(int(r))
	}
}

func ToRank(a string) Rank {
	r, err := strconv.Atoi(a)
	if err != nil {
		panic(err)
	}
	return Rank(r)
}

type Suit byte

func (s Suit) String() string {
	return string([]byte{byte(s)})
}

func ToSuit(a string) Suit {
	return Suit(a[0])
}

func pop(a *[]string) string {
	val := (*a)[len(*a)-1]
	*a = (*a)[:len(*a)-1]
	return val
}
