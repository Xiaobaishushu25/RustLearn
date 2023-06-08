#[cfg(test)]
mod test_web{
    use std::fmt::format;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    #[test]
    fn test_single_web(){
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection_single(stream);
        }
    }
    fn handle_connection_single(mut stream:TcpStream){
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        let (status_line,filename) = if buffer.starts_with(get){
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        print!("{contents}");
        let response = format!("{status_line}{contents}");
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }


}