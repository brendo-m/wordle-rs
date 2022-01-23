use rust_embed::RustEmbed;
use std::io::Write;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
struct Resources;

enum Clue {
    Fixed(char, usize),
    Somewhere(char, usize),
    Nowhere(char),
    And(Box<Clue>, Box<Clue>),
}

impl Clue {
    fn matches(&self, str: &str) -> bool {
        match self {
            Clue::Fixed(c, i) => str.chars().nth(*i).unwrap() == *c,
            Clue::Somewhere(c, i) => str.contains(*c) && str.chars().nth(*i).unwrap() != *c,
            Clue::Nowhere(c) => !str.contains(*c),
            Clue::And(l, r) => l.matches(str) && r.matches(str),
        }
    }

    fn parse_clue(str: &str) -> Clue {
        str.split(" ")
            .enumerate()
            .map(|(i, s)| {
                let chars: Vec<char> = s.chars().collect();
                match chars[..] {
                    ['!', c] => Clue::Nowhere(c),
                    ['?', c] => Clue::Somewhere(c, i),
                    [c] => Clue::Fixed(c, i),
                    _ => panic!("invalid clue"),
                }
            })
            .reduce(|l, r| Clue::And(Box::new(l), Box::new(r)))
            .unwrap()
    }
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    let resource = Resources::get("five_letter_words.txt").unwrap();
    let words: Vec<&str> = std::str::from_utf8(resource.data.as_ref())
        .unwrap()
        .split("\n")
        .filter(|s| s.len() > 0) // remove trailing ""
        .collect();

    println!(
        "Enter clues like so: \"h ?e !l !l o\".\n \
       ? means letter appears somewhere, ie yellow.\n \
       ! means letter does not appear, ie grey.\n \
       no modifier means the letter appears exactly there, ie green."
    );

    let mut filtered = words;
    while filtered.len() > 1 {
        let clue_input = prompt("Clue: ");
        let clue = Clue::parse_clue(&clue_input);
        filtered = filtered.into_iter().filter(|&w| clue.matches(w)).collect();

        println!("Possibilities are {:?}", filtered);
    }

    println!("Final word is: {}", filtered.get(0).unwrap())
}
