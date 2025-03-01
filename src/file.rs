use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn save<T: serde::Serialize>(list: &Vec<T>, fname: &str) {
    let list_as_json = serde_json::to_string(list).expect("Failed to serialize list");
    let mut file = File::create(fname).expect("Could not create file!");
    file.write_all(list_as_json.as_bytes())
        .expect("Cannot write to the file!");
}

pub fn load<T: serde::de::DeserializeOwned>(fname: &str) -> Vec<T> {
    if !Path::new(fname).exists() {
        return vec![];
    }

    let file = File::open(fname).expect("Could not create file!");
    let list: Vec<T> = serde_json::from_reader(file).expect("Could not deserialize list");
    list
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_save() {
        let list = vec![1, 2, 3];
        save(&list, "test.json");
        assert!(Path::new("test.json").exists());
    }

    #[test]
    fn test_load() {
        let list = vec![1, 2, 3];
        save(&list, "test.json");
        let loaded_list = load::<i32>("test.json");
        assert_eq!(loaded_list, list);
    }
}