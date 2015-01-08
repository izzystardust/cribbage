package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/deckarep/golang-set"
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
	fmt.Println(hand.Score(startCard))

}

// +gen slice:"Where"
type Hand []Card

func (h Hand) Score(start Card) int {
	fifteens := append(h, start).Fifteens()
	pairs := append(h, start).Pairs()
	fmt.Println("fifteens:", fifteens)
	fmt.Println("pairs:   ", pairs)
	return 2*fifteens + 2*pairs
}

func (h Hand) Fifteens() int {
	return len(h.PowerSet().Where(func(a Hand) bool { return a.Sum() == 15 }))
}

func (h Hand) Pairs() int {
	return len(h.PowerSet().Where(func(a Hand) bool { return len(a) == 2 }).Where(func(a Hand) bool { return a[0].rank == a[1].rank }))
}

func (h Hand) Sum() int {
	return CardSlice(h).AggregateInt(func(v int, c Card) int { return v + c.Value() })
}

func (h Hand) PowerSet() HandSlice {
	// ew.
	asInterface := make([]interface{}, len(h))
	for i, v := range h {
		asInterface[i] = interface{}(v)
	}
	setRep := mapset.NewSetFromSlice(asInterface)
	power := setRep.PowerSet().ToSlice()
	ret := make([]Hand, len(power))
	for i, v := range power {
		subsetAsI := v.(mapset.Set).ToSlice()
		subset := make(Hand, len(subsetAsI))
		for j, vv := range subsetAsI {
			subset[j] = vv.(Card)
		}
		ret[i] = subset
	}
	return ret
}

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

// +gen slice:"Aggregate[int]"
type Card struct {
	rank Rank
	suit Suit
}

func (c Card) Value() int {
	if c.rank > 10 {
		return 10
	}
	return int(c.rank)
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
