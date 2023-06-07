#[cfg(test)]
mod test_rc {
    use std::rc::Rc;

    #[test]
    fn test_rc(){
        let rc = Rc::new(1);

    }
}
#[cfg(test)]
mod test_arc {
    use std::ops::Deref;
    use std::sync::Arc;
    use std::thread;
    use std::thread::Thread;
    // use futures::future::Lazy;
    use once_cell::sync::Lazy;
    #[test]
    fn test_arc(){
        // let arc = Arc::new(String::from("hello"));
        let x = 1;
        let arc = Arc::new(1);
        let handle = thread::spawn(move || {
            println!("{}", arc);
        });
        // println!("arc是{}",arc);
        handle.join().unwrap();
    }
    #[test]
    fn test_arc_static(){
        // let arc = Arc::new(String::from("hello"));
        // let x = 1;
        // static arc:Arc<i32> = Arc::new(1);
        // static x:String = String::from("hello");
        static  x:Lazy<String> = Lazy::new(|| {
            String::from("hello")
        });
        let handle = thread::spawn(move || {
            println!("{}", x.len());
        });
        println!("x是{}",x.len());
        handle.join().unwrap();
    }
}
#[cfg(test)]
mod test_refcell {
    use std::cell::RefCell;

    #[test]
    fn test_refcell(){
        let cell = RefCell::new(String::from("hello"));
        cell.borrow_mut().push_str("world");
        println!("{}",cell.borrow_mut())
    }
}