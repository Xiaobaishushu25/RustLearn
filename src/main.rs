use std::alloc::System;
use std::any::type_name;
use std::collections::hash_map::Entry::Vacant;
use std::sync::Arc;
pub mod async_test;
// pub mod lib;
pub mod ssh;
pub mod test;
pub mod TimerFuture;
pub mod cow;
pub mod closure;
pub mod SmartPointer;
pub mod Static;
pub mod web;
pub mod thread_pool;
pub mod TestMacro;
pub mod CheckTrait;
pub mod TestInto;
// mod ui;
// mod collection;
// use lib::ssh_mod::Ssh;
fn main() {
    println!("Hello, world!");
    let mut i = 0;
    while i < 10 {
        i+=1;
        println!("{}",i);
    }
    let c:char = 's';
    let mut string = String::from("hello");
    let x1 = &string[0..3];
    let x:&str ="asdas你妈";
    println!("x明白了{}", x.len());
    // let rectangle = Rectangle(15, 16); 
    let mut rectangle1 = Rectangle { width: 30, height: 20 };
    println!("{}", rectangle1.area());
    rectangle1.set_width(15);
    print!("{:?}",rectangle1);
    println!("{:?}", Rectangle::create_square(32));
    println!("克隆后是否地址是否相等？ {}",std::ptr::eq(&rectangle1,&(rectangle1.clone())));
    let qie = String::from("asda");
    let y = &qie;
    print!("字符串的地址是{}",y); // asda
    let i: [i32; 5] = [1,5,7,8,9];
    let x1: &[i32; 5] = &i;
    let x2: &[i32] = &i[1..4];
    display(&i);
    display(&x2);
    print!("数组切片是{:p}",x2); //print会自动解引用，要用{:p}
    // let ssh = lib::lib::Ssh { username: String::from("XBss"), password: "123456".parse().unwrap() };
    // let ssh = lib::ssh_mod::Ssh { username: String::from("XBss"), password: "123456".parse().unwrap() };
    // let ssh = Ssh { username: String::from("XBss"), password: "123456".parse().unwrap() };
    // println!("{:?}",ssh);
    // let add1 = lib::collection::vec::vec::add(1, 2);
    // print!("{}",add1);
    test::test();
    ssh::connect::connect();
    ssh::collection::vec::test();
    // ui::app::test();
    ssh::ui::app::test();
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.insert(2,1);
    match vec1.get(0){
        Some(value) => print!("{}",value),
        None => print!("获取失败")
    }
    println!("{}",vec1[vec1.len()-1]);
    for value in &vec1{
        print!("{}",value);
    }
    println!("闭包练习");
    let mut op = 0;
    exec(move || println!("{}",op));
    print!("{op}");//因为是栈类型，会自动复制一份，所以还能用
    let po = String::from("test");
    let po1 = po.clone();
    exec(move|| println!("{po1}"));
    print!("{po}");//这里po是堆类型，且没有实现copy方法，会报错 borrow of moved value: `po`
    let x = {
        let i = 1;
        move || {
            op = op+i;
            println!("{}",op)
        }
    };
    sout(x);
}
fn display(arr:&[i32]){
    println!("{:?}",arr)
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }
    fn set_width(&mut self, width:u32){
        self.width = width
    }
    //不用接收self作为参数的函数称为关联函数，用结构体名::函数名引用，我觉得像是static的作用
    fn create_square(size:u32) -> Rectangle{
        Rectangle{width:size, height:size}
    }
}
impl Clone for Rectangle{
    fn clone(&self) -> Self {
        Rectangle{
            ..*self
        }
    }
}
fn sout<F:FnMut()>(mut f:F){
    f()
}
fn exec<F>(f:F)->i32 where F:FnOnce(){
    f();
    1
}