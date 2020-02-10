fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("Provide an argument");
    if arg.len() != 1 {
        panic!("Expecting only a one letter argument");
    }
    let arg = arg.chars().next().unwrap();
    if !arg.is_ascii() || !arg.is_alphanumeric() {
        panic!("Expecting only a one letter argument");
    }
    let max = arg as u8 - 'a' as u8;

    let range = (0..=max).clone().chain((0..max).rev());
    for index in range {
        let letter = (index + 'a' as u8) as char;
        print!("{}", " ".repeat((max - index) as usize));
        print!("{}", letter);
        if index != 0 {
            print!("{}", " ".repeat((index * 2 - 1) as usize));
            print!("{}", letter);
        }
        println!();
    }
}
