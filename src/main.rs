use std::env;
use std::str;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;

mod msg;
mod filehandler;

fn main () {

    let pwd = env::current_dir().unwrap();
    let port_num = env::args().nth(1).unwrap();
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port_num)).unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream, &pwd);
    }
}

pub fn handle_connection(mut stream: TcpStream, pwd : &std::path::PathBuf) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let prefix = b"GET / HTTP/1.1\r\n";
    let child : &PathBuf;
    let mut tmp;
    let mut buf_text = "";

    if buffer.starts_with(prefix){
        child = pwd;
    } else {
        tmp = buffer.strip_prefix(b"GET ").unwrap();
        
        let mut cnt=0;
        for i in tmp {
            if i == &b' ' {
                break;
            }
            cnt = cnt+1;
        }

        child = pwd.join(str::from_urf8(tmp.split_at(cnt+1)));
        dbg!(&tmp);
    }
    let contents = filehandler::create_index_html(pwd, child);
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);    

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
