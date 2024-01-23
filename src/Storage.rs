///一个以u64为key，任意类型为值的map
mod storage {
    use crate::Storage::{WebEvent, RED};
    use std::any;
    use std::any::Any;
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::sync::{Arc, RwLock};

    pub struct Storage {
        data: RwLock<HashMap<u64, Arc<dyn Any + Send + Sync>>>,
    }
    impl Storage {
        pub fn new() -> Self {
            return Storage {
                data: RwLock::new(HashMap::new()),
            };
        }
        pub fn insert<T>(&self, name: u64, _value: T)
        where
            T: Any + Send + Sync,
        {
            let value = Arc::new(_value);
            let mut write_guard = self.data.write().unwrap();
            write_guard.insert(name, value);
        }
        pub fn get<V>(&self, name: &u64) -> Option<Arc<V>>
        where
            V: Any + Send + Sync,
        {
            let read_guard = self.data.read().unwrap();
            match read_guard.get(name) {
                Some(value) => {
                    //Any 类型可以判断类型
                    if value.is::<V>() {
                        let arc_value = value.clone();
                        let arc = arc_value.downcast::<V>().unwrap();
                        Some(arc)
                        //用下面这个unsafe也可以，这样就可以不用Send+Sync了
                        // unsafe {
                        //     // let c = Arc::into_raw(value.clone());
                        //     //因为这里Arc::into_raw(value.clone())返回的是*const dyn Any,所以需要强转。
                        //     Some(Arc::from_raw(Arc::into_raw(value.clone()) as *const V))
                        // }
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }
    fn print_type_of<T>(_: &T) {
        println!("{}", any::type_name::<T>());
    }
    #[test]
    fn test_main() {
        let storage = Storage::new();
        storage.insert(4, "sd".to_string());
        storage.insert(4, "sad".to_string());
        storage.insert(1, "sad");
        let option1 = storage.get::<&str>(&1);
        let option2 = storage.get::<String>(&4);
        println!("{:?}", option1); //Some("sad")
        println!("{:?}", option2); //Some("sad")
        print_type_of(&option1); //core::option::Option<alloc::sync::Arc<&str>> 可以看出确实是有类型信息的
        print_type_of(&option2); //core::option::Option<alloc::sync::Arc<alloc::string::String>>
        let event = WebEvent::Red(11, 2, 3);
    }
}
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
    Red(u8, u8, u8),
}
const RED: WebEvent = WebEvent::Red(11, 2, 3);
