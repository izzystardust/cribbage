mod card {
    use std::fmt;
    use std::iter::AdditiveIterator;
    use std::ops::Add;
    use std::slice;

    pub fn score(hand: Vec<Card>, start_card: Card) -> i32 {
        let mut with_start = hand.clone();
        with_start.push(start_card);
        2 * count_15s(&with_start)
            + 2 * count_pairs(&with_start)
            + score_runs(&with_start)
    }

    fn count_15s(cards: &Vec<Card>) -> i32 {
        power_set(&mut cards.iter()).iter()
            .filter(|ref x| x.iter().map(|ref x| x.rank.value()).sum() == 15)
            .count() as i32
    }

    fn count_pairs(cards: &Vec<Card>) -> i32 {
        power_set(&mut cards.iter()).iter()
            .filter(|&x| x.len() == 2)
            .filter(|ref x| x[0].rank == x[1].rank)
            .count() as i32
    }

    fn score_runs(cards: &Vec<Card>) -> i32 {
        let mut cpy = cards.clone();
        cpy.sort();
        let mut runs = Vec::new();
        let mut run: Vec<Card> = Vec::new();
        for card in cpy.iter() {
            //println!("Looking at {} and {}", card, run);
            if run.len() == 0 || run[run.len()-1].clone().rank + Rank(1) == card.rank {
                run.push(card.clone())
            } else {
                if run.len() > 1 {
                    runs.push(run);
                }
                run = Vec::new();
                run.push(card.clone());
            }
        }
        if run.len() > 1 {
            runs.push(run);
        }
        runs.iter()
            .map(|x| x.len() as i32)
            .sum()
    }

    fn power_set<'a, T: Clone + 'a>(items: &mut slice::Iter<'a,T>) -> Vec<Vec<T>> {
        let mut power = Vec::new();
        match items.next() {
            None       => power.push(Vec::new()),
            Some(item) => {
                for mut set in power_set(items).into_iter() {
                    power.push(set.clone());
                    set.push(item.clone());
                    power.push(set);
                }
            }
        }
        power
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    impl Rank {
        fn value(&self) -> i32 {
            let Rank(x) = *self;
            if x > 10 {
                10
            } else {
                x
            }
        }
    }

    impl Add for Rank {
        type Output = Rank;

        fn add(self, rhs: Rank) -> Rank {
            let (Rank(lh), Rank(rh)) = (self, rhs);
            Rank(lh + rh)
        }
    }

}

fn main() {
    let cards = vec![
        card::new(12, 'H'),
        card::new(11, 'S'),
        card::new(10, 'S'),
        card::new(5, 'C'),
        ];
    println!("{}", card::score(cards, card::new(4, 'D')));
}
