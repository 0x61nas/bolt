static PORT: u16 = 3344;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    lib_bolt::start(args, PORT);
}
