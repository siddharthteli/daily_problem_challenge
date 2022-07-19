use std::io;
fn main() {
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Nahi");
    let mut count_int = count_input.trim().parse::<u32>().expect("sd");

    while (count_int > 0) {
        count_int -= 1;
        let mut x_y = String::new();
        io::stdin().read_line(&mut x_y).expect("Nahi");

        let inputs: Vec<u32> = x_y
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        if (inputs[0] >= inputs[1]) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
