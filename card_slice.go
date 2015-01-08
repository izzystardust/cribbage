// Generated by: gen
// TypeWriter: slice
// Directive: +gen on Card

package main

// CardSlice is a slice of type Card. Use it where you would use []Card.
type CardSlice []Card

// AggregateInt iterates over CardSlice, operating on each element while maintaining ‘state’. See: http://clipperhouse.github.io/gen/#Aggregate
func (rcv CardSlice) AggregateInt(fn func(int, Card) int) (result int) {
	for _, v := range rcv {
		result = fn(result, v)
	}
	return
}
