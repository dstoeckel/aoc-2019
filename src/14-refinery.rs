use std::collections::HashMap;
use std::io::BufRead;

#[test]
fn example_small() {
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    let reactions = parse_reactions(input.as_bytes());

    let mut available = HashMap::new();
    let ore = required_ore(&reactions, &mut available, 1);
    assert_eq!(165, ore);
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Reactant {
    name: String,
    qty: usize,
}

impl Reactant {
    fn new(name: String, qty: usize) -> Reactant {
        Reactant { name, qty }
    }
}

struct Reaction {
    product: Reactant,
    educts: Vec<Reactant>,
}

fn parse_reactant(input: &str) -> Reactant {
    let mut s = input.split(" ");

    Reactant {
        qty: s.next().unwrap().parse().unwrap(),
        name: s.next().unwrap().to_string(),
    }
}

fn parse_reaction(input: String) -> (String, Reaction) {
    let mut split = input.split(" => ");

    let educts = split
        .next()
        .unwrap()
        .split(", ")
        .map(parse_reactant)
        .collect();
    let product = parse_reactant(split.next().unwrap());

    (product.name.clone(), Reaction { educts, product })
}

fn required_ore(
    reactions: &HashMap<String, Reaction>,
    available_reactants: &mut HashMap<String, usize>,
    n: usize,
) -> usize {
    let mut required_reactants = Vec::new();

    required_reactants.push(Reactant::new("FUEL".to_string(), n));

    let mut required_ore = 0;
    while let Some(reactant) = required_reactants.pop() {
        if reactant.name == "ORE" {
            required_ore += reactant.qty;
            continue;
        }

        let available = *available_reactants.get(&reactant.name).unwrap_or(&0);
        let required = if available > reactant.qty {
            0
        } else {
            reactant.qty - available
        };

        let available_after = if reactant.qty > available {
            0
        } else {
            available - reactant.qty
        };

        let reaction = reactions.get(&reactant.name).unwrap();

        let factor = (reaction.product.qty + required - 1) / reaction.product.qty;

        if required > 0 {
            for educt in reaction.educts.iter() {
                let mut tmp = educt.clone();
                tmp.qty *= factor;
                required_reactants.push(tmp);
            }
        }

        let left_over = reaction.product.qty * factor - required;
        available_reactants.insert(reactant.name, available_after + left_over);
    }

    required_ore
}

fn parse_reactions<B: BufRead>(reader: B) -> HashMap<String, Reaction> {
    reader.lines().map(|l| parse_reaction(l.unwrap())).collect()
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required");

    let file = std::fs::File::open(path).unwrap();

    let reactions = parse_reactions(std::io::BufReader::new(file));

    let mut available_reactants = HashMap::new();
    let minimum_ore = required_ore(&reactions, &mut available_reactants, 1);
    println!(
        "Need at least {} units of ORE to generate 1 unit of FUEL.",
        minimum_ore
    );

    // Solve part 2. We do this by implementing division to speed things up (by a lot)
    let mut available_reactants = HashMap::new();
    let mut available_ore = 1000000000000usize;

    let mut fuel = 0;
    let mut amount = available_ore / minimum_ore;
    loop {
        let mut tmp = available_reactants.clone();
        let r = required_ore(&reactions, &mut tmp, amount);

        if available_ore < r {
            if amount == 1 {
                break;
            }
            amount = (amount + 1) / 2;
            continue;
        }

        available_reactants = tmp;
        available_ore -= r;
        fuel += amount
    }

    println!("The amount of possible fuel is {}", fuel);
}
