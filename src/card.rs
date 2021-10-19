pub struct Card {
    question: String,
    answer: String,
    category: String
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

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
}
