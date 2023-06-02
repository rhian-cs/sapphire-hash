use hash_calculator::file_hash_calculator;

fn main() {
    let hex = file_hash_calculator::calculate("tmp/hello-world.txt").unwrap();

    println!("{hex}");
}
