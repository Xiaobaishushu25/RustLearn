// pub mod ssh_mod {
//     #[derive(Debug)]
//     pub struct Ssh {
//         pub username:String,
//         pub password:String,
//     }
//     impl Ssh {
//         pub fn connect(&self){
//             print!("使用用户名{}，密码{}尝试连接",self.username,self.password);
//         }
//     }
//     fn log(){
//         print!("这是一个log");
//     }
//     mod hosting{
//         fn test(){
//             print!("这是一个test");
//         }
//     }
// }
//
// // use crate::lib::Ssh;
// // use crate::lib::Ssh;
// // fn create_ssh(username:String,password:String){
// //     // let ssh1 = crate::lib::Ssh { username, password };
// //     let ssh1 = Ssh { username, password };
// //     println!("{:?}",ssh1)
// // }
// // fn create_ssh(username:String,password:String) {
// //     let ssh1 = crate::lib::Ssh { username, password };
// //     println!("{:?}", ssh1)
// // }
// fn main() {}
mod async_closure;
mod Storage;