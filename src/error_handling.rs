#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::Error;
    use std::fs::{read, File};
    use syn::Path;

    fn double_arg(mut argv: env::Args) -> Result<i32, String> {
        argv.nth(1)
            .ok_or("Please give at least one argument".to_owned())
            .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
            .map(|n| 2 * n)
    }
    #[test]
    fn main() {
        // match double_arg(env::args()) {
        //     Ok(n) => println!("{}", n),
        //     Err(err) => println!("Error: {}", err),
        // };
        let result: Result<&str, Error> = Result::Ok("hello");
        let option = Some(1);
        let option1 = option.map(|x| Some(x + 1));
        let option2 = option.and_then(|x| Some(x + 1));
        let x = result.map(|x| Ok::<std::string::String, Error>(x.to_owned()));
        let result1 = result.and_then(|x| Ok(x.to_owned()));
        // let string1 = String::from("he");
        let string1 = String::from("你好");
        let mut x1 = string1.as_bytes().to_owned();
        println!("{:?}", x1);

        for u in x1.iter_mut() {
            *u = *u + 1_u8;
        }
        let result2 = String::from_utf8(x1);
        println!("{:?}", result2);
    }
    fn double_float<P: AsRef<Path>>(path: P) -> Result<i32, String> {
        Ok(1)
    }
    trait X {
        type ItemType;
        fn x(&self, other: Self::ItemType);
    }
    struct A {
        a: Box<dyn X<ItemType = i32>>,
    }
}
