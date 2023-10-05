use console::Style;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::str::FromStr;

pub struct Todo {
    pub map: HashMap<String, bool>,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")
            .unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let map: HashMap<String, bool> = contents
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        fs::write("db.txt", content)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()>{
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    pub fn remove(&mut self, key:String){
        self.map.remove(&key);
    }

    pub fn list(self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (k, v) in self.map {
            if v == false {
                let new_k = Style::new().strikethrough().apply_to(&k).to_string();
                result.push(new_k);
            } else {
                let new_k = Style::new().bold().cyan().apply_to(k).to_string();
                result.push(new_k);
            }
        }
        result
    }
}
