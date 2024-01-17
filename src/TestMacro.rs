//声明式宏( declarative macros )
#[cfg(test)]
mod test_macro{
    use std::collections::HashMap;

    ///取text的前num个字符并转为大写
    //索引操作符要求索引值必须是usize类型，而不是i32类型
    fn show(text:String,num:usize){
        let text_split:Vec<_> = text.split("").collect();
        // println!("{:?}",text_split);
        let x = text_split[0..num+1].concat().to_uppercase();
        println!("{x}");
    }
    /**
    用宏可以起到类似函数重载的效果，可以给函数指定默认值，但是当我们使用 show!（）这样的宏时，我们无法知道需要的参数类型了
    */
    macro_rules! show {
        ($a:expr) => {
            show($a,5)
        };
        ($a:expr,$b:expr) => {
            show($a,$b)
        };
    }
    #[test]
    fn test_show_macro(){
        show("hello world".to_string(),3);
        show!("hello world".to_string());
    }
    ///在宏调用中，冒号 : 是一个特殊的标记，不能直接在 macro_rules! 宏的规则中使用
    macro_rules! hashmap {
        () => {HashMap::new()};
        //这里表示匹配多个 $key => $value参数，分隔符是逗号(,)   *表示匹配任意次
        ($($key:expr=>$value:expr),*) => {
            // 这里一定要有大括号包裹，因为这里有多条语句。使用大括号，产生一个块表达式。即这个块具有返回值。
            // 而且宏展开之后就看的比较清晰了
            {
                let mut map = HashMap::new();
                //$() 和 * 则用于实现重复模式，允许在宏展开期间重复执行相同的操作，以处理宏调用中的多个参数。
                $(
                    map.insert($key,$value);
                )*
                map
            }
    }
    // macro_rules! hashmap {
    // // 和vec!一样，没有任何参数则创建一个空的hashmap
    // () => {
    //     {::std::collections::HashMap::new()}
    // };
    // // 这里表示匹配多个 $key => $value参数，分隔符是逗号(,)
    // // 最后$(,)? 表示最后一个逗号(,)可有可无
    // ($($key:expr => $value: expr),+$(,)?) => {
    //     { // 这里一定要有大括号包裹，因为这里有多条语句。使用大括号，产生一个块表达式。宏展开之后就看的比较清晰了
    //         let mut _m = ::std::collections::HashMap::new();
    //         $(
    //             _m.insert($key, $value);
    //         )*
    //         _m
    //     }
    // }
}

    #[test]
    fn test_hash_macro(){
        let map = hashmap!{
            "a"=>1,
            "b"=>2
        };
        println!("map是{:?}",map)
    }
}
//proc_macro：过程宏
#[cfg(test)]
mod test_proc_macro{
    use std::mem;
    use quote::quote;
    use syn::parse_macro_input;

    pub trait HeapSize{
        fn heap_size_of_children(&self) -> usize;
    }

    impl HeapSize for u8 {
        /// A `u8` does not own any heap memory.
        fn heap_size_of_children(&self) -> usize {
            0
        }
    }
    impl HeapSize for String{
        /// A `String` owns heap memory.
        fn heap_size_of_children(&self) -> usize {
            self.capacity()
        }
    }
    impl<T> HeapSize for Box<T>
    where T:?Sized + HeapSize,
    {
        fn heap_size_of_children(&self) -> usize {
            //因为self是引用，先解引用得到Box<T>，再解引用得到T，在引用
            mem::size_of_val(&**self) + (**self).heap_size_of_children()
        }
    }

    impl<T> HeapSize for [T]
        where
            T:HeapSize
    {
        fn heap_size_of_children(&self) -> usize {
            // self.iter().map(|it|{it.heap_size_of_children()}).sum()
            //函数指针也实现了FnMut，跟c++类似，函数名不带括号可以视为函数指针
            self.iter().map(HeapSize::heap_size_of_children).sum()
        }
    }

    impl<'a, T> HeapSize for &'a T
        where
            T: ?Sized,
    {
        // A shared reference does not own heap memory.
        fn heap_size_of_children(&self) -> usize {
            0
        }
    }
    // #[proc_macro_derive(HeapSize)]
    // pub fn derive_heap_size(input:proc_marco::TokenStream) -> proc_marco::TokenStream{
    //     // 基于 input 构建 AST 语法树
    //     let ast = syn::parse(input).unwrap();
    //     // let input = parse_macro_input!(input as DeriveInput);
    //
    //     impl_hello_macro(&ast)
    // }
    #[test]
    fn test_macro(){
        let x = Box::new(String::from("hello"));
        println!("大小是{}", x.heap_size_of_children());
    }
    #[derive(Debug)]
    struct A{
        id:i32,
    }

    impl A {
        fn new() -> Self{
            A{id:1}
        }
    }

    #[test]
    fn test_box(){
        let x = Box::new(A::new());
        // let y = &x;
        teet(&x);
        // println!("{:?}",**y)
    }
    fn teet(a:&Box<A>) {
        println!("{:?}",**a)
    }
}