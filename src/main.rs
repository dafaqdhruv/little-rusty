use std::cmp::Ordering;
use std::env;
use std::fs;

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

    fs::File::create("favicon.ico").expect("cannot create favicon.");
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream, &pwd);
    }
}

pub fn handle_connection(mut stream: TcpStream, pwd : &std::path::PathBuf) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let prefix = b"GET / HTTP/1.1\r\n";
    let child : PathBuf;
    let tmp;
    let mut cnt = 0;
    

    if buffer.starts_with(prefix){
        child = pwd.to_path_buf();
    } else {

        println!("{}", String::from_utf8_lossy(&buffer[..]));
        tmp = match buffer.strip_prefix(b"GET "){
            Some(v) => v,
            None => {return;}
        };
        for i in tmp {
            if i.cmp(&32) == Ordering::Equal  {
                break;
            }
            cnt = cnt +1;
        }

        let child_path = String::from_utf8_lossy(&tmp[1..cnt]);
        let child_path = child_path.as_ref();
    
        dbg!(&child_path);
        child =  pwd.as_path().join(child_path);
    }

    let contents ;
    if child.is_dir() {
        contents = filehandler::create_index_html(pwd, &child);
    } else {
        contents = String::from("");
    }
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);    

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
