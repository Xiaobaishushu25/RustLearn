#[cfg(test)]
mod test_mutual_arc {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test() {
        let arc = Arc::new(Mutex::new(0));
        let mut threads = Vec::with_capacity(10);
        for _ in 0..10 {
            let arc_clone = arc.clone();
            let handle = thread::spawn(move || {
                let mut mutex_guard = arc_clone.lock().unwrap();
                *mutex_guard += 1;
            });
            threads.push(handle);
        }
        for handle in threads {
            handle.join().unwrap();
        }
        println!("Result is {}", arc.lock().unwrap())
    }
}
#[cfg(test)]
mod test_block {
    use serde_yaml::Value;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::{env, fs};
    // fn toml2yaml(content:&str) -> Result<String>{
    //     let yaml = serde_yaml::from_str(content)?;
    //     Ok(serde_yaml::to_string())
    // }

    #[test]
    fn test() {
        // // if let Some(content1) = fs::read_to_string("./Cargo.toml"){
        // //     println!("content1")
        // // }else {
        // //     println!("error")
        // // };
        // // println!("{content1}")
        // let content1 = fs::read_to_string("./Cargo.toml").unwrap_or("错误".to_string());
        // let content2 = fs::read_to_string("./Cargo.lock").unwrap_or("错误".to_string());
        // println!("{content1}");
        // println!("{content2}");
        // // fs::write("Cargo1.toml",content1).unwrap();
        // // fs::write("/tmp/Cargo.lock",content2).unwrap();
        // let path = Path::new("./tmp");
        // if !path.exists() { fs::create_dir_all("./tmp").expect("Unable to create file"); }
        // fs::write("./tmp/Cargo.toml", content1).unwrap_or_else(|error| {
        //     if error.kind() == std::io::ErrorKind::NotFound {
        //         // 创建新文件
        //         panic!("Unable to find file: {}", error);
        //     } else {
        //         panic!("Unable to write file: {}", error);
        //     }
        // });
        let args: Vec<String> = env::args().collect();
        for s in args.iter() {
            println!("{s}")
        }
        let sss = "sda";
        let string = String::from("asdsad");
        let buf = env::current_dir().unwrap_or(PathBuf::from("/bin"));
        let cow = buf.to_string_lossy();
        println!("当前项目路径是{cow}")
    }
}
#[cfg(test)]
mod tests {
    use crate::sout;
    use std::thread;
    use std::thread::Thread;
    use std::time::Duration;
    use syn::__private::str;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_spawn() {
        let handle = tokio::spawn(async { println!("1111") });
        tokio::spawn(async { println!("2222") });
        println!("44444");
        let string = contain_async();
        println!("{string}")
    }
    fn contain_async() -> String {
        tokio::spawn(async {
            for i in 0..10 {
                println!("spanw{i}");
            }
        });
        "hello".to_string()
    }
}
