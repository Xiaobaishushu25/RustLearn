#[cfg(test)]
mod test_closure {
    use std::thread;
    use std::thread::Thread;

    #[test]
    fn test_FnOnce1() {
        let mut op = 0;
        exec_Once(move || println!("{}",op));
        println!("{op}");//因为是栈类型，会自动复制一份，所以还能用
        let po = String::from("test once");
        // exec(move|| println!("{po}"));
        exec_Once(|| println!("{po}"));
        print!("{po}");//这里po是堆类型，且没有实现copy方法，会报错 borrow of moved value: `po`
    }
    #[test]
    fn test_FnOnce_fake() {
        let po = String::from("test once");
        exec_Once(|| println!("{po}"));
        print!("{po}");//正常
        exec_Once(||{
            let vec = po.into_bytes();
        });
        // print!("{po}"); //error[E0382]: borrow of moved value: `po`
    }
    #[test]
    fn test_FnMut(){
        let mut op = 1;
        exec_Mut(||{op = 2});
        println!("{op}") //2
    }
    #[test]
    fn test_Fn() {
        let mut op = 1;
        exec_Fn(move || println!("{}",op));
        println!("{op}");//因为是栈类型，会自动复制一份，所以还能用
        let po = String::from("test fn");
        // exec(move|| println!("{po}"));
        exec_Fn(|| println!("{po}"));
        exec_Fn(move|| println!("{po}"));
        // print!("{po}");//这里po是堆类型，且没有实现copy方法，会报错 borrow of moved value: `po`
    }
    #[test]
    fn test_dyn_FnOnce(){
        let closure = test_FnOnce();
        let string = closure();
        println!("{string}")
    }
    #[test]
    fn test_dyn_Fn(){
        // let closure = test_Fn();
        // let string = closure();
        // //let string = closure();//不能执行两次：this value implements `FnOnce`, which causes it to be moved when called
        // println!("{string}")
    }

    #[test]
    fn test_what_error(){
        let s = String::from("error");
        exec_Fn(move||{
            println!("{s}");
            // let vec = s.into_bytes();
        });
        // print!("{s}");
    }

    #[test]
    fn test_param(){
        let s = 5;
        let i = exec_Fn_with_param(s, |_| {
            s * 2
        });
        println!("{i}")
    }

    fn test_FnOnce() -> Box<dyn FnOnce() -> String> {
        let a: String = String::from("dyn FnOnce() -> String");
        Box::new(move || { a }) //  move occurs because `a` has type `String`, which does not implement the `Copy` trait
    }


    // fn test_Fn() -> Box<dyn Fn() -> String> {
    //     let a: String = String::from("dyn Fn() -> String");
    //     Box::new(move || { a }) //  move occurs because `a` has type `String`, which does not implement the `Copy` trait
    // }

    fn exec_Once<F>(f:F) where F:FnOnce() -> (){
        f();
    }
    fn exec_Mut<F:FnMut()>(mut f:F){
        f()
    }
    fn exec_Fn(f:impl Fn()){
        f()
    }
    fn exec_Fn_with_param(a:i32,f:impl Fn(i32)->i32)->i32{
        f(a)
    }
    #[test]
    fn test_twice_move(){
        let s = String::from("can move?");
        let join_handle = thread::spawn(move || {
            // let closure = exec_Once(||{
            //     println!("触发快捷键");
            //     let closure2 = exec_Once(move||{
            //         println!("你好");
            //         println!("{s}")
            //     });
            // });

            let closure = exec_Fn(||{
                let copy = s.clone();
                println!("触发快捷键");
                let closure2 = exec_Once(move||{
                    println!("你好");
                    println!("{copy}")
                });
            });
        });
        join_handle.join().expect("等待异常");
    }

}