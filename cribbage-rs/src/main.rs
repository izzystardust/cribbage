mod card {
    use std::fmt;
    use std::iter::AdditiveIterator;
    use std::ops::Add;
    use std::slice;

    pub fn score(hand: Vec<Card>, start_card: Card, crib: bool) -> i32 {
        let mut with_start = hand.clone();
        with_start.push(start_card.clone());
        let fifteens = count_15s(&with_start);
        let pairs = count_pairs(&with_start);
        let flushpoints = score_flushes(&hand, start_card, crib);
        let runpoints = score_runs(&with_start);
        let nobs = if his_nobs(&hand, start_card) {1} else {0};
        println!("fifteens: {}", fifteens);
        println!("pairs:    {}", pairs);
        println!("flush:    {} points", flushpoints);
        println!("runs:     {} points", runpoints);
        println!("his nobs: {}", nobs);
        2 * fifteens
            + 2 * pairs
            + flushpoints
            + runpoints
            + nobs
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

    fn score_flushes(cards: &Vec<Card>, start: Card, crib: bool) -> i32 {
        let mut cards = cards.clone();
        let expected_suit = cards.pop().unwrap().suit;
        let four_flush = cards.iter()
            .fold(true, |so_far, next| so_far && next.suit == expected_suit);
        match (four_flush, start.suit==expected_suit, !crib) {
            (true, true, _)      => 5,
            (true, false, true)  => 4,
            (true, false, false) => 0,
            (false, _, _)        => 0,
        }
    }

    fn his_nobs(cards: &Vec<Card>, start: Card) -> bool {
        cards.iter()
            .filter(|x| x.rank == Rank(11))
            .fold(false, |sofar, jack| sofar || jack.suit == start.suit)
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
        card::new(5, 'H'),
        card::new(5, 'D'),
        card::new(5, 'C'),
        card::new(11, 'S'),
        ];
    println!("{}", card::score(cards, card::new(5, 'S'), false));
}
