use clap::Parser;
use passwords::PasswordGenerator;

const YES: &str = "y";
const NO: &str = "n";
const YES_NO: &str = "Yes/No";
const NUMBER: &str = "Number";

/// Password generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Passwords to generate
    #[clap(short, long, default_value_t = 5, value_name = NUMBER,display_order=1)]
    quantity: usize,

    /// Size/Length of password
    #[clap(short, long, default_value_t = 8,value_name = NUMBER,display_order=2)]
    size: usize,

    /// Include numbers
    #[clap(short, long, value_parser, default_value = YES,value_name=YES_NO,value_parser=[YES,NO],display_order=3)]
    numbers: String,

    /// Include lowercase letters
    #[clap(short, long, value_parser, default_value = YES,value_name=YES_NO,value_parser=[YES,NO],display_order=4)]
    lowercase_letters: String,

    /// Include uppercase letters
    #[clap(short, long, value_parser, default_value = YES,value_name=YES_NO,value_parser=[YES,NO],display_order=5)]
    uppercase_letters: String,

    /// Include symbols
    #[clap(short='y', long, value_parser, default_value = YES,value_name=YES_NO,value_parser=[YES,NO],display_order=6)]
    symbols: String,
}

fn main() {
    let args = Args::parse();

    let quantity = args.quantity;
    let length = args.size;

    let numbers = match args.numbers.as_str() {
        YES => true,
        _ => false,
    };

    let lowercase_letters = match args.lowercase_letters.as_str() {
        YES => true,
        _ => false,
    };

    let uppercase_letters = match args.uppercase_letters.as_str() {
        YES => true,
        _ => false,
    };

    let symbols = match args.symbols.as_str() {
        YES => true,
        _ => false,
    };

    if !numbers && !lowercase_letters && !uppercase_letters && !symbols {
        println!("You must select at least 1 character type to generate a password from");
        return;
    }

    let pg = PasswordGenerator {
        length,
        numbers,
        lowercase_letters,
        uppercase_letters,
        symbols,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };

    let passwords = pg.generate(quantity).unwrap();
    for i in passwords {
        println!("{:?}", i);
    }
}
