use::std::io;
use::std::io::Write; // Used for stdout flushing
use std::collections::HashMap;

// Type alias
type DB = HashMap<String, HashMap<String, String>>;
type Entry = HashMap<String, String>;


// Utility Functions
fn input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error: unable to read from stdin");

    buffer
}


// CRUD functions
fn put(mut db: DB, mut cmd: std::str::SplitWhitespace) -> DB {
    let key = String::from(cmd.next().expect("nil"));

    let entry = db.get_mut(key.as_str());

    match entry {
        Some(item) => {
            item.insert(
                String::from(cmd.next().expect("nil")),
                String::from(cmd.next().expect("nil"))
            );
        },
        None => {  // primary_key does not exist in DB yet
            let mut item = Entry::new();

            item.insert(
                String::from(cmd.next().expect("nil")),
                String::from(cmd.next().expect("nil"))
            );

            db.insert(key, item);
        }
    }

    db
}


fn get(db: HashMap<String, Entry>, mut cmd: std::str::SplitWhitespace) -> (HashMap<String, Entry>, String) {
    let key = String::from(cmd.next().expect("nil"));

    let result = db.get(key.as_str());
    
    match result {
        Some(entry) => {
            let key = cmd.next().expect("nil");
            let to_return = entry.get(key).expect("nil").clone(); // Cloned because db is borrowed otherwise

            (db, String::from(to_return)) // ???
        },
        None => (db, String::from("nil"))
    }
}


fn update(db: HashMap<String, Entry>, cmd: std::str::SplitWhitespace) -> HashMap<String, Entry> {
    put(db, cmd)
}


fn delete(mut db: HashMap<String, Entry>, mut cmd: std::str::SplitWhitespace) -> HashMap<String, Entry> {
    let key = cmd.next().expect("nil");
    db.remove(key);

    db
}


// DB Management functions
pub fn start() {
    let mut db: HashMap<String, Entry> = HashMap::new();

    loop {
        let cmd = input();
        let mut cmds = cmd.split_whitespace();

        match cmds.next() {
            Some("put")    => db = put(db, cmds),
            Some("get")    => {
                let res = get(db, cmds);
                db = res.0;
                println!("{}", res.1);
            },

            Some("update") => db = update(db, cmds),
            Some("delete") => db = delete(db, cmds),
            Some("exit")   => exit(),
            Some(_)        => println!("error: could not read command"),
            None           => println!("\n")
        }
    }
}


fn exit() {
    println!("bye");
    std::process::exit(0);
}
