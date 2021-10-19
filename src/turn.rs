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

pub struct Card {
    question: String,
    answer: String,
    category: String
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::Turn;

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
