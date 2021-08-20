use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Needs key");
    let value = arguments.next().expect("Needs value");
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).expect("Failed to write to DB");

    let database = Database::new();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => return Err(error),
        // };
        // read kv.db file as string
        // ? below is same as above match
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // parse string
            let (key, value) = line.split_once('\t').expect("Corrupt DB");
            // populate map
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }
}
