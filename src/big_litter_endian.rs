use image::ImageBuffer;

#[test]
fn test_i32() {
    let x: i32 = 123456789;
    //0 表示用0来填充结果字符串。
    //32 表示结果字符串应该有32个字符宽。
    //b 表示结果应该是二进制格式。
    println!("{:032b}", x); //00000111 01011011 11001101 00010101
    verify();
    let be: [u8; 4] = x.to_be_bytes(); //将i32转为大端序列
    let le: [u8; 4] = x.to_le_bytes(); //将i32转为小端序列
    println!("{:08b}", x >> 24); //00000111
    println!("{:016b}", x >> 16); //00000111 01011011
    println!("{:024b}", x >> 8); //00000111 01011011 11001101
    println!("{:08b}", (x >> 8) as u8); //11001101
    println!("{:08b}", ((x >> 8) & 0xFF)); //11001101
    println!("{:?}", x.to_be_bytes()); //[7, 91, 205, 21]
    println!("{:?}", i32_to_bytes_be(x)); //[7, 91, 205, 21]
    println!("{:?}", i32_to_bytes_be2(x)); //[7, 91, 205, 21]
}
fn verify() {
    //00000111 01011011 11001101 00010101
    let index = vec![0, 2, 4, 8, 10, 11, 14, 15, 16, 17, 19, 20, 22, 24, 25, 26];
    let mut sum = 0;
    for i in index {
        sum += 2_i32.pow(i);
    }
    println!("{sum}"); //123456789
}
fn i32_to_bytes_be(num: i32) -> [u8; 4] {
    let mut bytes = [0; 4];
    bytes[0] = ((num >> 24) & 0xFF) as u8;
    bytes[1] = ((num >> 16) & 0xFF) as u8;
    bytes[2] = ((num >> 8) & 0xFF) as u8;
    bytes[3] = (num & 0xFF) as u8;
    bytes
}
fn i32_to_bytes_be2(num: i32) -> [u8; 4] {
    let mut bytes = [0; 4];
    bytes[0] = (num >> 24) as u8;
    bytes[1] = (num >> 16) as u8;
    bytes[2] = (num >> 8) as u8;
    bytes[3] = num as u8;
    bytes
}
fn bytes_to_i32_be(bytes: &[u8; 4]) -> i32 {
    let mut result: i32 = 0;
    result |= (bytes[0] as i32) << 24;
    result |= (bytes[1] as i32) << 16;
    result |= (bytes[2] as i32) << 8;
    result |= bytes[3] as i32;
    result
}
