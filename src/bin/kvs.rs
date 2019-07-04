use kvs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs", about = "rust db exercise")]
enum Action {
    // Set a key to a value.
    // The (key,value) pair will be inserted
    // if they do not already exist.
    #[structopt(name = "set")]
    Set {
        #[structopt(parse(from_str))]
        key: String,
        #[structopt(parse(from_str))]
        value: String,
    },
    // Get the value at a key, if it exists.
    #[structopt(name = "get")]
    Get {
        #[structopt(parse(from_str))]
        key: String,
    },
    // Remove the value at a key, if it exists.
    #[structopt(name = "remove")]
    Remove {
        #[structopt(parse(from_str))]
        key: String,
    },
}

fn main() {
    // Initialise the in-memory store.
    let mut store = kvs::KvStore::new();

    // Get our 'Action'.
    let opt = Action::from_args();

    match opt {
        Action::Set { key, value } => store.set(key, value),
        Action::Get { key } => {
            let same = key.clone();
            match store.get(key) {
                None => println!("No value found at key: {}", same),
                Some(val) => println!("We got the pair: ({}, {})", same, val),
           }
        }
        Action::Remove { key } => store.remove(key),
    }
}
