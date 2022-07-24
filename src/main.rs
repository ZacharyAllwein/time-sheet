mod time;

fn main() {

    let time = time::from_str("8:15").unwrap();
    let time2 = time::from_str("12:45").unwrap();

    println!("{:?}", time::elapsed(&time, &time2));
}
