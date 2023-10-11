use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_suit(num_suit: usize, repeat: bool) -> Vec<String> {
    let suit = vec![
        "Spades".to_string(),
        "Diamonds".to_string(),
        "Clubs".to_string(),
        "Hearts".to_string(),
    ];

    let mut rng = thread_rng();
    let mut suits = suit;

    if repeat {
        suits = suits.into_iter().cycle().take(num_suit).collect();
    } else {
        suits.shuffle(&mut rng);
        suits.truncate(num_suit);
    }

    suits
}

pub fn create_value(num_value: usize, repeat: bool) -> Vec<String> {
    let value = vec![
        "Ace of".to_string(),
        "King of".to_string(),
        "Queen of".to_string(),
        "Jack of".to_string(),
        "Ten of".to_string(),
        "Nine of".to_string(),
        "Eight of".to_string(),
        "Seven of".to_string(),
        "Six of".to_string(),
        "Five of".to_string(),
        "Four of".to_string(),
        "Three of".to_string(),
        "Two of".to_string(),
    ];

    let mut rng = thread_rng();
    let mut values = value;

    if repeat {
        values = values.into_iter().cycle().take(num_value).collect();
    } else {
        values.shuffle(&mut rng);
        values.truncate(num_value);
    }

    values
}

#[test]
fn test_create_suit() {
    let num_suit = 4;
    let repeat = true;
    let suits = create_suit(num_suit, repeat);
    assert_eq!(suits.len(), num_suit);
}
