#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn cards_in_category(&self, category: &str) -> Vec<Card> {
        let cards: Vec<Card> = self.cards.clone();
        let found_cards: Vec<Card> = cards
            .into_iter()
            .filter(|card| card.category.eq(category))
            .collect::<Vec<Card>>();
        found_cards
    }
}

pub struct Turn {
    card: Card,
    guess: String
}

impl Turn {
    pub fn feedback(&self) -> String {
        if self.is_correct() {
            String::from("Correct!")
        } else {
            String::from("Incorrect.")
        }
    }

    fn is_correct(&self) -> bool {
        self.guess == self.card.answer
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    question: String,
    answer: String,
    category: String
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::Turn;
    use super::Deck;

    #[test]
    fn deck_returns_count_of_cards() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let card2 = Card {
            question: String::from("What is the capital of Alaska?"),
            answer: String::from("Juneau"),
            category: String::from("Geography")
        };

        let card3 = Card {
            question: String::from("What is the closest planet to our sun?"),
            answer: String::from("Mercury"),
            category: String::from("Astronomy")
        };

        let deck = Deck {
            cards: vec![card1, card2, card3]
        };

        assert_eq!(deck.count(), 3)
    }

    #[test]
    fn deck_returns_card_for_category() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let card2 = Card {
            question: String::from("What is the capital of Alaska?"),
            answer: String::from("Juneau"),
            category: String::from("Geography")
        };

        let card3 = Card {
            question: String::from("What is the closest planet to our sun?"),
            answer: String::from("Mercury"),
            category: String::from("Astronomy")
        };

        let deck = Deck {
            cards: vec![card1, card2, card3.clone()]
        };

        let astronomy_cards = deck.cards_in_category("Astronomy");

        assert_eq!(astronomy_cards.len(), 1);
        assert_eq!(astronomy_cards[0].question, card3.question);
        assert_eq!(astronomy_cards[0].answer, card3.answer);
        assert_eq!(astronomy_cards[0].category, card3.category);
    }

    #[test]
    fn returns_attr_data() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        assert_eq!(card1.question, "What is 2 + 2?");
        assert_eq!(card1.answer, "4");
        assert_eq!(card1.category, "Math");
    }

    #[test]
    fn turn_returns_true_for_correct_guess() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let turn1 = Turn {
            card: card1,
            guess: String::from("4")
        };

        assert_eq!(turn1.is_correct(), true)
    }

    #[test]
    fn turn_returns_false_for_incorrect_guess() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let turn1 = Turn {
            card: card1,
            guess: String::from("5")
        };

        assert_eq!(turn1.is_correct(), false)
    }

    #[test]
    fn gives_feedback_when_correct() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let turn1 = Turn {
            card: card1,
            guess: String::from("4")
        };

        assert_eq!(turn1.feedback(), "Correct!")
    }

    #[test]
    fn gives_feedback_when_incorrect() {
        let card1 = Card {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
            category: String::from("Math")
        };

        let turn1 = Turn {
            card: card1,
            guess: String::from("5")
        };

        assert_eq!(turn1.feedback(), "Incorrect.")
    }
}
