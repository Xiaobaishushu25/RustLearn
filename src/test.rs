pub fn test(){
    println!("这是test")
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
        // do6(&s);
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
    // fn do6(mut c: &String){
    //     c.push_str("asd");//error: cannot borrow `*c` as mutable, as it is behind a `&` reference
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
mod test_option{
    //这俩方法好像没啥区别，都是转换后返回一个Option
    #[test]
    fn test(){
        let option1 = Option::Some(1);
        let new_option = option1.and_then(|value| {
            Option::Some(value + 1)
        });
        println!("{:?}",new_option);
        let option2: Option<i32> = Option::Some(2);
        let x = option2.map(|value| {
            value + 1
        }).map(|value|{
            value.to_string()
        });
        println!("{:?}",x);
        let option3 = Some("hello|world".to_string());
        let new_option3 = option3.map(|it| {
            let vecx:Vec<String> = it.split("|").map(|it|{it.to_string()}).collect();
            vecx
        });
        println!("{:?}",new_option3);
        let option4 = Some("hello|world".to_string());
        // let new_option3 = option4.and_then(|it| {
        //     let vecx:Vec<String> = it.split("|").collect();
        //     vecx
        // });
        // println!("{:?}",new_option3)
    }
}
#[cfg(test)]
mod test_scope{
    use std::thread;
    use std::time::Duration;

    struct SomeThing {
        msg: String
    }

    impl SomeThing {
        fn print(&self) {
            println!("{}", self.msg);
        }
        fn do_something(&self) {
            let mut s = "sd";
            //scope可以实现多个线程借用同一个变量，好像是个阻塞的，会将当前线程阻塞到结束
            thread::scope(|t|{
                let mut children = vec![];
                for _ in 0..10 {
                    children.push(t.spawn( || {
                        thread::sleep(Duration::new(10,0));
                        self.print();
                    }));
                }
                // for child in children {
                //     let _ = child.join();
                // }
            });
            print!("函数结束");
            // for child in children {
            //     let _ = child.join();
            // }
        }
    }
    #[test]
    fn test(){
        let sth = SomeThing{msg: String::from("hi")};
        sth.do_something();
    }
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        // let mut vec1:Vec<i32> = Vec::with_capacity(nums.len());
        let mut vec1:Vec<i32> = vec![0;nums.len()];
        println!("{}",nums.len());
        for (i,num) in nums.into_iter().enumerate(){
            vec1[num as usize] += 1;
            if vec1[num as usize]>1 { return num}
        }
        -1
    }
    #[test]
    fn test_find(){
        let i = find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]);
    }
}