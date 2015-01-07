mod card {
    use std::fmt;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Card {
        rank: Rank,
        suit: Suit,
    }

    pub fn new(rank: i32, suit: char) -> Card {
        Card{rank: Rank(rank), suit: from_char(suit)}
    }

    impl fmt::Show for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Suit {
        Spades,
        Hearts,
        Clubs,
        Diamonds,
    }

    impl Suit {
        pub fn to_char(&self) -> char {
            match *self {
                Suit::Spades   => 'S',
                Suit::Hearts   => 'H',
                Suit::Diamonds => 'D',
                Suit::Clubs    => 'C',
            }
        }
    }

    impl fmt::Show for Suit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", self.to_char())
        }
    }

    pub fn from_char(a: char) -> Suit {
        match a {
            'S'=> Suit::Spades,
            'H'=> Suit::Hearts,
            'C'=> Suit::Clubs,
            'D'=> Suit::Diamonds,
            _  => panic!("Not a suit")
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Rank(i32);

    impl fmt::Show for Rank {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let value = match *self {
                Rank(13) => "K".to_string(),
                Rank(12) => "Q".to_string(),
                Rank(11) => "J".to_string(),
                Rank(aa) => aa.to_string(),
            };
            write!(f, "{}", value)
        }
    }
}

fn main() {
    let mut cards = vec!(
        card::new(12, 'H'),
        card::new(11, 'S'),
        card::new(1, 'S'),
        );
    cards.sort();
    println!("{}", cards);
}
