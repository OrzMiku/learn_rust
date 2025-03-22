use std::{
    collections::HashMap, error::Error, fmt::{self, Display, Formatter}, io::{self, Write}
};

fn main() {
    env_logger::init();
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Bob", "1234567 06 27"),
        ("Eve", "1234567 Dec 27 123"),
    ]);

    print!("Enter a name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let name = read_input();
    let result = get_credit_card_info(&credit_cards, &name);

    match result {
        Ok(card) => println!("{}", card),
        Err(err) => {
            println!("{}", err);

            log::error!("Error: {:?}", err);
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
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

#[derive(Debug)]
struct ParseCreditCardInfoError {
    source: Option<Box<dyn Error>>,
    msg: Option<String>
}

impl Display for ParseCreditCardInfoError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg.as_ref().unwrap())
    }
}

impl Error for ParseCreditCardInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Card {
    fn parse_numbers(card_string: &str) -> Result<Vec<u32>, ParseCreditCardInfoError> {
        let numbers: Vec<u32> = card_string
            .split_whitespace()
            .map(|s| {
                s.parse::<u32>()
                .map_err(|_| ParseCreditCardInfoError {
                    source: None,
                    msg: Some(format!("{:?} cannot be parsed as u32", s))
                })
            })
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|err| ParseCreditCardInfoError {
                source: Some(Box::new(err)),
                msg: Some(format!("Failed to parse credit card info, Input: {}", card_string))
            })?;
        
        Ok(numbers)
    }

    fn parse(card_string: &str, name: &str) -> Result<Card, ParseCreditCardInfoError> {
        let mut numbers = Card::parse_numbers(card_string)?;

        let len = numbers.len();
        if len < 4 {
            return Err(ParseCreditCardInfoError {
                source: None,
                msg: Some("Not enough fields".to_string())
            });
        }

        let cvv = numbers.pop().unwrap();
        let year = numbers.pop().map(|y| y as u16).unwrap();
        let month = numbers.pop().map(|m| m as u8).unwrap();
        let number = numbers.pop().unwrap();

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
        write!(
            f,
            "Name: {}\nNumber: {}\nExpiration: {}/{}\nCVV: {}",
            self.name, self.number, self.exp.month, self.exp.year, self.cvv
        )
    }
}

#[derive(Debug)]
enum CreditCardError {
    CardInputError(String),
    Other(Box<dyn Error>, String),
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CardInputError(msg) => write!(f, "{}", msg),
            Self::Other(source, msg) => write!(f, "{}\n-- {}", msg, source),
        }
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CardInputError(_) => None,
            Self::Other(source, _) => Some(source.as_ref()),
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards
        .get(name)
        .ok_or(CreditCardError::CardInputError(format!(
            "Name {name} not found!"
        )))?;
    let card = Card::parse(card_string, name).map_err(|err| {
        CreditCardError::Other(
            Box::new(err),
            format!("Failed to parse credit card info for {}", name),
        )
    })?;
    Ok(card)
}
