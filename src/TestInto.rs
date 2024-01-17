#[cfg(test)]
mod test_into{
    #[derive(Debug,Clone)]
    struct User{
        name:String,
    }
    //注意：此trait只是user→String，并不能反过来，隐式实现into只是多了一种写法
    // 从user获得一个String(String from user)，同时自动实现user转换为&str(user into String)
    impl From<User> for String {
        fn from(value: User) -> Self {
            value.name
        }
    }
    // impl From<&str> for User{
    //     fn from(value: &str) -> Self {
    //         User{name:value.to_owned()}
    //     }
    // }
    #[test]
    fn test_into(){
        let user1 = User { name: "张三".to_owned() };
        let user2 = user1.clone();
        let user3 = user1.clone();

        let name1:String = user1.into();
        let name2 = String::from(user2);
        let name3 = Into::<String>::into(user3);
        println!("{name1}");
        println!("{name2}");
        println!("{name3}");
    }
}