#[cfg(test)]
mod test_arc {
    use std::cell::{Cell, OnceCell, RefCell};
    use std::ops::Deref;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread;
    use std::thread::Thread;
    use once_cell::sync::Lazy;
    use serde_yaml::Value::Bool;

    #[test]
    fn test_static(){
        static mut x:Lazy<String> = Lazy::new(|| {
            String::from("hello")
        });
        unsafe {
            let handle = thread::spawn(move || {
                println!("{}", x.len()); //5
            });
            x.push_str("world");
            println!("{}",x.len()); //10
            handle.join().unwrap();
        }
    }

    // #[test]
    // fn test_static_tuple1(){
    //     static t:(i32,String) = (1,String::from("hello"));
    //     println!("{}",t.1) //error[E0015]: cannot call non-const fn `<std::string::String as From<&str>>::from` in statics
    // }
    #[test]
    fn test_static_tuple3(){
        static t:Lazy<(i32,String)> = Lazy::new(|| {
            (1, String::from("hello"))
        });
        println!("{}",t.1) //hello
    }
    #[test]
    fn test_static_tuple2(){
        static mut t:(i32,bool) = (1,true);
        unsafe {
            println!("{}",t.1); //true
            t.1 = false;
            println!("{}",t.1); //true
        }
    }

    #[derive(Debug)]
    struct Color(i32,i32,i32);
    #[test]
    fn test_static_tuple_struct1(){
        static c:Color = Color(255,0,255);
        println!("{:?}",c); //Color(255, 0, 255)
    }
    #[test]
    fn test_static_tuple_struct2(){
        static c:Mutex<Color> = Mutex::new(Color(255,0,255));
        println!("{:?}",c.lock().unwrap()); //Color(255, 0, 255)
    }

    #[derive(Debug)]
    struct MyPosition {
        x:i32,
        y:i32
    }
    impl MyPosition {
        // const fn new(x:i32,y:i32)->Self{
        fn new(x:i32,y:i32)->Self{
            MyPosition {
                x,
                y
            }
        }
        fn setX(&mut self,x:i32){
            self.x = x;
        }
        fn setY(&mut self,y:i32){
            self.y = y;
        }
    }
    // #[test]
    // fn test_static_struct(){
    //     static mut p:MyPosition = MyPosition::new(10,20);
    //     unsafe {
    //         println!("{:?}",p); //MyPosition { x: 10, y: 20 }
    //         p.setX(20);
    //         println!("{:?}",p); //MyPosition { x: 10, y: 20 }
    //     }
    // }

    #[test]
    fn test_static_struct_leak(){
        static mut sp:Option<&mut MyPosition> = None;
        unsafe {
            let mut p = Box::new(MyPosition::new(10,20));
            sp = Some(Box::leak(p));
            println!("{:?}",sp); //Some(MyPosition { x: 10, y: 20 })

            // match sp {
            //     Some(p) => {
            //         println!("{:?}",p)
            //     }
            //     _ => {}
            // }
            // sp.unwrap().setX(20);
            let x:&mut &mut MyPosition = sp.as_mut().unwrap();
            x.setY(50);
            println!("{:?}",x); //MyPosition { x: 10, y: 50 }
            // pos.setY(30);
            println!("{:?}",sp.as_ref().unwrap()); //MyPosition { x: 10, y: 20 }
        }
    }

    #[test]
    fn test_std_onceCell(){
        let mut cell = OnceCell::new();
        let mut position = cell.get_or_init(|| {
            MyPosition::new(15, 20)
        });
        println!("{:?}",position);
        let option = cell.get_mut();
        let x = option.unwrap();
        x.setX(150);
        // thread::spawn(move||{
        //     let option1 = cell.get_mut().unwrap();
        //     option1.setY(250);
        // });
        println!("{:?}",cell.get());
    }
    #[test]
    fn test_std_onceCell_static(){
        static mut cell:OnceCell<MyPosition> = OnceCell::new();
        unsafe {
            cell.get_or_init(||{
                MyPosition::new(15, 20)
            });
            let option = cell.get();
            println!("{:?}",option.unwrap()); //MyPosition { x: 15, y: 20 }
            let handle = thread::spawn(move || {
                let option1 = cell.get_mut().unwrap();
                option1.setY(250);
                println!("{:?}", cell.get().unwrap()); //MyPosition { x: 150, y: 250 }
            });
            let x = cell.get_mut().unwrap();
            x.setX(150);
            println!("{:?}",cell.get().unwrap()); //MyPosition { x: 150, y: 20 }
            handle.join().expect("TODO: panic message");
            let x = cell.get_mut().unwrap();
            x.setX(250);
            println!("{:?}",cell.get().unwrap()); //MyPosition { x: 250, y: 250 }
        }
    }
    #[derive(Debug)]
    struct Op{
        text:String,
    }
    impl Op {
        fn new()->Self{
            Op{
                text:"hello".to_string()
            }
        }
    }
    // 在你提供的代码中，你确实创建了两个并发线程，然后在这两个线程中调用了 cell.get_mut().unwrap() 方法，试图获取 OnceCell 的可变引用。
    // 然而，这是一个很危险的操作，因为 OnceCell 并没有为并发环境提供正确的同步机制。实际上，根据 Rust 的文档，
    // OnceCell::get_mut 方法在并发环境中的行为是未定义的。这就意味着你的代码可能会在某些情况下正常工作，但在其他情况下出错，或者产生不可预测的结果。
    // 你的代码能够运行成功可能是因为线程调度的结果导致了数据竞态条件没有发生。这是因为两个线程可能没有在同一时间尝试获取 OnceCell 的
    // 可变引用。然而，你不能依赖这种行为，因为线程调度的结果是非确定性的，它取决于很多因素，如操作系统的调度策略、系统的负载、CPU的数量等等。
    // 这就是为什么你应该总是使用适当的同步原语来保护并发环境中的共享数据。在 Rust 中，这通常意味着使用诸如 Mutex 或 RwLock 这样的
    // 类型来保护数据。如果你试图在没有适当同步的情况下访问共享数据，那么你的代码可能会在某些情况下工作，但在其他情况下出错，或者产生不可预测的结果。
    // 对于你的代码，如果你希望在多个线程中共享和修改 Op 实例，你应该使用 Mutex<Op> 或者 RwLock<Op> 来代替 OnceCell<Op>。这
    // 样可以保证在任何时候只有一个线程可以修改 Op，而其他线程可以安全地读取 Op 的值。
    #[test]
    fn test_std_onceCell_static_op(){
        static mut cell:OnceCell<Op> = OnceCell::new();
        unsafe {
            cell.get_or_init(||{
                Op::new()
            });
            let option = cell.get();
            println!("{:?}",option);
            let mut vec = Vec::with_capacity(300);
            for index in 0..10000{
                let handle = thread::spawn(move|| {
                    let option1 = cell.get_mut().unwrap();
                    println!("第{index} {:?}", cell.get().unwrap());
                });
                vec.push(handle);
            }
            for h in vec{
                h.join().unwrap();
            }
            let option2 = cell.get();
            println!("{:?}",option2)
            // handle.join().unwrap();
        }
    }
    #[test]
    fn test_std_onceLock_static_op(){
        static  cell:OnceLock<Op> = OnceLock::new();
        unsafe {
            cell.get_or_init(|| {
                Op::new()
            });
            let option2 = cell.get();
            println!("{:?}",option2.unwrap())
        }
    }
    #[test]
    fn test_error(){
        // static CACHE: OnceCell<Vec<i32>> = OnceCell::new();
        // fn get_data() -> &'static Vec<i32> {
        //     CACHE.get_or_init(|| {
        //         let data = vec![1, 2, 3, 4, 5];
        //         println!("Initializing cache");
        //         data
        //     })
        // }
        // let data = get_data();
        // println!("Data: {:?}", data);
        //
        // let data = get_data();
        // println!("Data: {:?}", data);
    }
}