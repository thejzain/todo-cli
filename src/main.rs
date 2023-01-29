use std::collections::HashMap;
use std::env;


struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert (&mut self, key: String){
        self.map.insert(key, true);
    }

    fn save (self) -> Result<(),std::io::Error>{
        let mut content = String::new();
        for (k,v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main(){
    let action: String = env::args().nth(1).expect("action err");
    let item : String = env::args().nth(2).expect("item err");
    let mut todo = Todo{
        map: HashMap::new(),
    };
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    }

}