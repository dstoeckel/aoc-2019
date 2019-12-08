use std::io::Read;

fn merge_pixels(pixels: (u8, &u8)) -> u8 {
    match pixels {
        (2, b) => *b,
        (a, _) => a,
    }
}

fn checksum(pixels: &Vec<u8>, width: usize, height: usize) {
    let layers = pixels.as_slice().chunks(width * height);
    let most_zeros: &[u8] = layers
        .min_by_key(|x| x.iter().filter(|p| **p == 0).count())
        .unwrap();

    let ones = most_zeros.iter().filter(|x| **x == 1).count();
    let twos = most_zeros.iter().filter(|x| **x == 2).count();

    println!(
        "The layer with the fewest 0 digits contains {} ones and {} twos yielding {}",
        ones,
        twos,
        ones * twos
    );
}

fn decode(pixels: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let transparent = vec![2; width * height];
    pixels
        .as_slice()
        .chunks(width * height)
        .fold(transparent, |acc, x| {
            acc.into_iter()
                .zip(x.into_iter())
                .map(merge_pixels)
                .collect()
        })
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let width = std::env::args()
        .nth(2)
        .map(|x| str::parse::<usize>(&x).unwrap())
        .expect("No width argument was given");

    let height = std::env::args()
        .nth(3)
        .map(|x| str::parse::<usize>(&x).unwrap())
        .expect("No width argument was given");

    let mut code = String::new();
    std::fs::File::open(path)
        .expect("Could not open input file")
        .read_to_string(&mut code)
        .expect("Error while reading from file.");

    let pixels: Vec<u8> = code.trim().as_bytes().into_iter().map(|x| x - 48).collect();

    checksum(&pixels, width, height);
    let decoded = decode(&pixels, width, height);

    for h in 0..height {
        for w in 0..width {
            let c = decoded[h * width + w];

            if c == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        print!("\n");
    }
}
