use std::{collections::HashMap};

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error accurred: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        // 在我们的map中新增一个新的元素
        // 我们默认将其状态值设置为true
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
       let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occureed:{}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error accurred: {}", why),
            },
        }
    }
}
