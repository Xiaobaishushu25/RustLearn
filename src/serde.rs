#[cfg(test)]
mod test_json{
    use std::ops::Index;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Source {
        #[serde(rename = "bookSourceComment")]
        pub source_url: String,
        #[serde(rename = "bookSourceName")]
        pub name: String,
        #[serde(rename = "bookSourceGroup")]
        pub group: Option<String>,
        #[serde(rename = "loginUrl")]
        pub login_url: Option<String>,
        #[serde(rename = "bookUrlPattern")]
        pub book_url_pattern: Option<String>,
        pub header: Option<String>,
    }

    #[cfg(test)]
    mod test {
        use super::Source;
        use std::{fs, path::Path};

        #[test]
        fn test_list_deserialize() {
            let i = &fs::read_to_string(Path::new("./src/test.json")).unwrap();
            print!("{i}");
            let vec1 = vec![1, 2, 3];
            let i1 = vec1[0];
            println!("---------------------------");
            let i: Vec<Source> = serde_json::from_str(i).unwrap();
            print!("{i:#?}");
        }
    }
}