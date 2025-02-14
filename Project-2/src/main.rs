mod io;
mod structs;

fn main() {
    let inf = io::read_from_json("train/train_0.json").unwrap();
    println!("{:?}", inf);
}
