use crate::error_handle::app_errors::AppError::AnyHow;
use crate::error_handle::app_errors::AppResult;
use anyhow::anyhow;

#[test]
fn test_anyhow_marco() {
    let e = return_anyhow_error();
    match e {
        Ok(_) => {}
        Err(e) => {
            println!("{e}"); //anyhow error:`这是anyhow宏构建的错误`
        }
    }
    // println!("{:?}",e);//只能以debug格式直接打印，并且会打印出栈调用信息
}
// fn return_anyhow_error()->anyhow::Error{
fn return_anyhow_error() -> AppResult<()> {
    // anyhow!("这是anyhow宏构建的错误")
    // Err(AnyHow(anyhow!("这是anyhow宏构建的错误")))
    return_anyhow_result()?; //thiserror是通过？运算符转换错误的，不能直接返回一个不匹配的错误类型
    Ok(())
}
fn return_anyhow_result() -> Result<(), anyhow::Error> {
    // fn return_anyhow_error() -> AppResult<()> {
    Err(anyhow!("这是anyhow宏构建的错误"))
    // Err(AnyHow(anyhow!("这是anyhow宏构建的错误")))
}
