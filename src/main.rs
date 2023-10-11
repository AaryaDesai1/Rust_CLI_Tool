use clap::Parser;
use make_card::create_suit;
use make_card::create_value;
use std::io;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Generate cards with a combination of suit and value"
)]
struct Opts {
    #[clap(short, long)]
    num_cards: Option<usize>, // Changed the field name to num_cards and made it optional
}

fn main() {
    let opts: Opts = Opts::parse();

    let num_cards: usize = match opts.num_cards {
        Some(num) => num,
        None => {
            println!("Please enter the number of cards to generate:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().parse::<usize>().unwrap()
        }
    };

    // Generate suits and values
    let suits = create_suit(num_cards, true);
    let values = create_value(num_cards, true);

    // Combine suits and values to create cards
    let cards: Vec<String> = suits
        .into_iter()
        .zip(values)
        .map(|(s, v)| format!("{} {}", v, s))
        .collect();

    // Print the generated cards
    println!("Generated cards:");
    for card in cards {
        println!("{}", card);
    }
}
