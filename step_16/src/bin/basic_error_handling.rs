use std::{collections::HashMap, fmt::{self, Display, Formatter}, io::{self, Write}};

fn main() {
    let credit_cards = HashMap::from(
        [
            ("Amy", "1234567 04 25 123"),
            ("Bob", "1234567 06 27"),
            ("Eve", "1234567 Dec 27 123"),
        ]
    );

    print!("Enter a name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let name = read_input();
    let result = get_credit_card_info(&credit_cards, &name);

    match result {
        Ok(card) => println!("{}", card),
        Err(err) => println!("Error: {}", err),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

#[derive(Debug)]
struct Card {
    name: String,
    number: String,
    exp: Expiration,
    cvv: String,
}

#[derive(Debug)]
struct Expiration {
    month: u8,
    year: u16,
}

impl Card {
    fn parse_numbers(card_string: &str) -> Result<Vec<u32>, String> {
        let numbers: Vec<u32> = card_string
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|err| err.to_string())?;
        Ok(numbers)
    }

    fn parse(card_string: &str, name: &str) -> Result<Card, String> {
        let mut numbers = Card::parse_numbers(card_string)?;

        let cvv = numbers.pop().ok_or("CVV not found")?;
        let year = numbers.pop().map(|y| y as u16).ok_or("Year not found")?;
        let month = numbers.pop().map(|m| m as u8).ok_or("Month not found")?;
        let number = numbers.pop().ok_or("Number not found")?;

        let exp = Expiration { month, year };
        let card = Card {
            name: name.to_string(),
            number: number.to_string(),
            exp,
            cvv: cvv.to_string(),
        };
        Ok(card)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Name: {}\nNumber: {}\nExpiration: {}/{}\nCVV: {}", self.name, self.number, self.exp.month, self.exp.year, self.cvv)
    }
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, String> {
    let card_string = credit_cards.get(name).ok_or("Name not found")?;
    let card = Card::parse(card_string, name)?;
    Ok(card)
}