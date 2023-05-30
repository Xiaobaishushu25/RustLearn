#[cfg(test)]
mod test_mutual_arc{
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::thread::Thread;

    #[test]
    fn test(){
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
        for handle in threads{
            handle.join().unwrap();
        }
        println!("Result is {}",arc.lock().unwrap())
    }
}
#[cfg(test)]
mod test_dref{
    #[test]
    fn test(){
        let mut x = 5;
        let mut y = &mut x;
        *y = *y+1;
        println!("Result is {}",y)
    }
}
#[cfg(test)]
mod test_String{
    #[test]
    fn test(){
        let mut s = String::from("test15 84 hel12llo");
        let news:String = s.chars().map(|c|if c.is_ascii_digit() { "".to_string() }else { c.to_string() } ).collect();
        println!("{news}");//test  helllo
        let newss = s.replace(|c:char| c.is_ascii_digit(), "");
        println!("{newss}");//test  helllo
    }
}