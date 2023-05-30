pub fn test(){
    println!("这是test")
}
#[cfg(test)]
mod test_closure {
    #[test]
    fn it_works() {
        let mut op = 0;
        exec(move || println!("{}",op));
        println!("{op}");//因为是栈类型，会自动复制一份，所以还能用
        let po = String::from("test");
        exec(|| println!("{po}"));
        print!("{po}");//这里po是堆类型，且没有实现copy方法，会报错 borrow of moved value: `po`
    }
    fn exec<F>(f:F) where F:FnOnce() -> (){
        f();
    }
}
#[cfg(test)]
mod test_reference_mut{
    #[test]
    fn test(){
        let s = String::from("test");
        do1(s);
        let s = String::from("test");
        do2(&s);
        let mut s = String::from("test");
        do3(&mut s);
        do1(s);
        let s = String::from("test");
        // do5(s);
        let change = do51(s, String::from("hello"));
        println!("改变后是{change}");
        let mut s = String::from("test");
        let mut d = String::from("hello");
        do6(&mut s);
        do7(&mut s,&mut d);
        println!("s改变之后是{s}")
        // let mut s = String::from("test");
        // do6(&mut s);
        // let mut s = String::from("test");
        // let mut_s = do6(&mut s);
        // println!("{mut_s}")
    }
    fn do1(c: String) {}
    fn do2(c: &String) {}
    fn do3(c: &mut String) {
        c.push_str("new"); //c的内容是可变的
    }
    fn do41(mut c:String){
        c.push_str("hello")
    }
    fn do4(c: &mut String,d:&mut String) {
        c.push_str("asd");//c的内容是可变的
        //c = d; error:Cannot assign twice to immutable variable(不能将两次赋值给不可变变量),c指向不可变！
    }
    fn do411(mut c: String,mut d:String) {
        c.push_str("asd");//c的内容是可变的
        c = d; //这里会调用c的drop，将其释放
    }
    fn do5(mut c: String) {}
    fn do51(mut c: String,mut d:String)->String {
        c = d;
        c
    }
    fn do6(mut c: &mut String) {}
    // fn do6(mut c: &String){ //error: cannot borrow `*c` as mutable, as it is behind a `&` reference
    //     c.push_str("asd");
    // }
    fn do7<'a>(mut c: &'a mut String,mut d: &'a mut String){
        c.push_str("asd");
        c = d;
        println!("在函数内部c是{c}")
    }
}
#[cfg(test)]
mod test_reference_mut_2{
    #[test]
    fn test(){
        let mut s = String::from("hello");
        let mut t = String::from("hello");
        // do5(&mut s);
        // assert_eq!(s, "hello"); // s 的值没有被修改
        // do5(&mut t);
        // assert_eq!(t, "world"); // t 的值被修改为 "world"
    }
    // fn do5(mut c: &mut String) {
    //     let mut s = String::from("world");
    //     c = &mut s;
    // }
}
#[cfg(test)]
mod test_drop{
    struct people{
        name:String
    }
    impl Drop for people{
        fn drop(&mut self) {
            println!("删除{}",self.name)
        }
    }
    #[test]
    fn test(){
        let a = people { name: "张三".to_string() };
        let b = people { name: "李四".to_string() };
        chane_point(a,b)
    }
    fn chane_point(mut a:people,mut b:people){
        a = b; //在这里就把a删除了，不会造成内存泄露
        println!("赋值完毕")
    }
}
#[cfg(test)]
mod test_new_type{
    use std::fmt::{Display, Formatter};

    struct Wrapper<T>(Vec<T>);
    #[test]
    fn test(){
        let mut list = vec![1,5,78,54];
        let string = String::from("asdas");
    }
}
#[cfg(test)]
mod test_open{
    use std::fs::File;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test(){
        let path = Path::new("F:\\删除\\1标签loss和22扰动和五千卷积");
        if path.exists(){
            Command::new("cmd.exe").arg("/c").arg(&("start ".to_string()+path.to_str().unwrap())).output().unwrap();
        }else {
            println!("文件夹不存在")
        }
    }
}