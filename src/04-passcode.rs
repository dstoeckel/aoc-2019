const FROM: u32 = 245318;
const TO: u32 = 765747;

#[test]
fn test_examples() {
    assert_eq!(false, check_conditions(&[1, 1, 1, 1, 1, 1]));
    assert_eq!(true, check_conditions(&[1, 1, 1, 1, 2, 2]));
    assert_eq!(false, check_conditions(&[1, 2, 3, 4, 4, 4]));
    assert_eq!(false, check_conditions(&[2, 2, 3, 4, 5, 0]));
    assert_eq!(false, check_conditions(&[1, 2, 3, 7, 8, 9]));
    assert_eq!(true, check_conditions(&[1, 2, 3, 7, 7, 9]));
    assert_eq!(true, check_conditions(&[1, 1, 2, 2, 3, 3]));
}

#[test]
fn test_to_byte_array() {
    let mut buf = [0; 6];
    to_byte_array(111111, &mut buf);
    assert_eq!([1; 6], buf);

    to_byte_array(981723, &mut buf);
    assert_eq!([9, 8, 1, 7, 2, 3], buf);
}

fn to_byte_array(mut x: u32, buf: &mut [u8; 6]) {
    let mut div = 100000;
    for i in 0..6 {
        let rem = x / div;
        x -= div * rem;
        div /= 10;

        buf[i] = rem as u8;
    }
}

fn check_conditions(buf: &[u8; 6]) -> bool {
    let mut counts = [0u8; 10];
    for i in 0..5 {
        if buf[i] > buf[i + 1] {
            return false;
        }

        if buf[i] == buf[i + 1] {
            counts[buf[i] as usize] += 1;
        }
    }

    counts.iter().any(|x| *x == 1)
}

fn main() {
    let mut array = [0; 6];
    let mut count = 0;
    for i in FROM..TO {
        to_byte_array(i, &mut array);
        if check_conditions(&array) {
            count += 1;
        }
    }

    println!("Found {} matching passcodes", count);
}
