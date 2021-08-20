fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Needs key");
    let value = arguments.next().expect("Needs value");
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).expect("Failed to write to DB");
}
