use std::io::Read;

#[test]
fn test_example_1() {
    let data = parse_data("12345678");
    let transformed = fft(data, 4);

    assert_eq!(transformed, [0, 1, 0, 2, 9, 4, 9, 8]);
}

#[test]
fn test_example_2() {
    let data = parse_data("80871224585914546619083218645595");
    let mut transformed = fft(data, 100);
    transformed.truncate(8);

    assert_eq!(transformed, [2, 4, 1, 7, 6, 1, 7, 6]);
}

#[test]
fn test_example_3() {
    let data = parse_data("19617804207202209144916044189917");
    let n = data.len();
    let mut transformed = fft(data, 100);
    assert_eq!(n, transformed.len());
    transformed.truncate(8);

    assert_eq!(transformed, [7, 3, 7, 4, 5, 4, 1, 8]);
}

#[test]
fn test_example_4() {
    let data = parse_data("69317163492948606335995924319873");
    let mut transformed = fft(data, 100);
    transformed.truncate(8);

    assert_eq!(transformed, [5, 2, 4, 3, 2, 1, 3, 3]);
}

#[test]
fn test_example_offset_1() {
    let data = parse_data("03036732577212944063491565474664");
    let offset = 303673;
    let mut transformed = fft_repeat_offset(data, 10000, offset, 100);
    transformed.truncate(8);

    assert_eq!(transformed, [8, 4, 4, 6, 2, 0, 2, 6]);
}

fn parse_data(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Unhandled character"),
        })
        .collect()
}

fn fft(mut a: Vec<isize>, nphases: usize) -> Vec<isize> {
    let base = [0, 1, 0, -1];

    let mut b = Vec::with_capacity(a.len());
    for _ in 0..nphases {
        let mut transform = (0..a.len()).map(|i| {
            base.iter()
                .flat_map(|a| std::iter::repeat(a).take(i + 1))
                .cycle()
                .skip(1)
                .zip(a.iter())
                .map(|(c, x)| c * x)
                .sum::<isize>()
                .abs()
                % 10
        });

        b.extend(&mut transform);
        a.clear();
        std::mem::swap(&mut a, &mut b);
    }

    a
}

fn offset_to_number(data: &Vec<isize>) -> usize {
    data.iter().take(7).fold(0, |acc, s| acc * 10 + *s as usize)
}

fn fft_repeat_offset(data: Vec<isize>, repeat: usize, offset: usize, nrounds: usize) -> Vec<isize> {
    if 2 * offset < repeat * data.len() {
        panic!("This only works for offsets larger than half of the signal!");
    }

    let n_orig = data.len();

    let mut a: Vec<isize> = data
        .into_iter()
        .cycle()
        .take(n_orig * 10000)
        .skip(offset)
        .collect();

    let n = a.len();

    let mut b = vec![0; n];

    for _ in 0..nrounds {
        b[n - 1] = a[n - 1];
        for i in (0..n - 1).rev() {
            b[i] = a[i] + b[i + 1]
        }

        b.iter_mut().for_each(|x| *x = x.abs() % 10);
        std::mem::swap(&mut a, &mut b);
    }

    a
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("At least one command line argument is required.");

    let mut data = String::new();
    std::fs::File::open(path)
        .expect("Could not open input file!")
        .read_to_string(&mut data)
        .expect("Error while reading from file");

    let data = parse_data(data.trim());
    let n = data.len();
    let offset = offset_to_number(&data);

    let transformed = fft_repeat_offset(data, 10000, offset, 100);
    let prefix: Vec<u8> = transformed.iter().take(8).map(|x| *x as u8 + 48).collect();

    println!("The prefix is {}", String::from_utf8(prefix).unwrap());
}
