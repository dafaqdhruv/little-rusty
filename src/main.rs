use std::cmp::Ordering;
use std::env;
use std::fs;

use regex::bytes;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path;

mod filehandler;

const INADDR_ANY: &str = "0.0.0.0"; // For a server, we typically want to listen to all interfaces (which is why we bind to INADDR_ANY)
const DEFAULT_PORT: &str = "8990";

fn main() {
    let pwd: path::PathBuf = env::current_dir().unwrap();

    let port: String = env::args().nth(1).unwrap_or(String::from(DEFAULT_PORT));
    let bind_addr: String = format!("{}:{}", INADDR_ANY, port);

    let listener: TcpListener = TcpListener::bind(bind_addr).expect("Unable to bind listener");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        serve(stream, &pwd);
    }
}

pub fn serve(mut stream: TcpStream, root: &std::path::PathBuf) {
    let mut request_buf: [u8; 0x400] = [0; 0x400];
    stream
        .read(&mut request_buf)
        .expect("Unable to read request into buffer");

    let mut path: String = "".to_owned();
    match request_buf.strip_prefix(b"GET /") {
        Some(buf) => {
            for c in buf {
                match *c as char {
                    ' ' => break,
                    _ => path.push(char::from(*c)),
                };
            }
        }
        None => return,
    };

    let url_decoded_path = urlencoding::decode(&path).unwrap().to_string();
    let path_to_serve = root.join(path::PathBuf::from(url_decoded_path));

    if path_to_serve.is_dir() {
        let contents = filehandler::create_index_html(&root, &path_to_serve);
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        match stream.flush() {
            Ok(_) => (),
            Err(e) => println!("{}: Failed stream.flush", e),
        };
    } else if path_to_serve.is_file() {
        let file_content = fs::read(&path_to_serve).expect("Failed to read file");
        let file_mime_type = filehandler::get_mime(
            path_to_serve
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap(),
        );
        let file_size = file_content.len();

        println!(
            "Requesting {:#?}",
            path_to_serve.strip_prefix(root).unwrap()
        );

        let agent = bytes::Regex::new(r"(?i)user-agent:\s+(.*?)\r\n").unwrap();
        let agent = agent.captures(&request_buf);
        match agent {
            Some(v) => println!("Agent: {:#?}", String::from_utf8_lossy(&v[1])),
            _ => (),
        };

        let referrer = bytes::Regex::new(r"(?i)referer:\s+(.*?)\r\n").unwrap();
        let referrer = referrer.captures(&request_buf);
        match referrer {
            Some(v) => println!("Referer: {:#?}", String::from_utf8_lossy(&v[1])),
            _ => (),
        };

        let mut start: usize = 0;
        let mut end: usize = file_size - 1;

        let byte_ranges = bytes::Regex::new(r"(?i)range:\s*bytes=(\d*)-(\d*)").unwrap();
        let byte_ranges = byte_ranges.captures(&request_buf);
        match byte_ranges {
            Some(v) => {
                let first_byte = String::from_utf8(v[1].to_vec()).unwrap();
                let last_byte = String::from_utf8(v[2].to_vec()).unwrap();

                if !first_byte.is_empty() && !last_byte.is_empty() {
                    if first_byte.cmp(&last_byte) == Ordering::Less {
                        start = first_byte.parse().unwrap();
                        end = last_byte.parse().unwrap();
                    }
                } else if !first_byte.is_empty() {
                    if first_byte.parse::<usize>().unwrap().cmp(&(file_size - 1)) == Ordering::Less
                    {
                        start = first_byte.parse().unwrap();
                    }
                } else if !last_byte.is_empty() {
                    if last_byte.parse::<usize>().unwrap().cmp(&(file_size - 1)) == Ordering::Less {
                        start = file_size - last_byte.parse::<usize>().unwrap();
                    }
                }
            }
            _ => (),
        }

        let mut response_status = "200 OK";
        let requested_content: Vec<u8> = (&file_content[start..end + 1]).to_vec();
        let content_range = match file_size != requested_content.len() {
            true => {
                response_status = "206 Partial Content";
                format!("Content-Range: bytes {}-{}/{}\r\n", start, end, file_size)
            }
            false => "".to_owned(),
        };

        let response = format!(
            "HTTP/1.1 {}\r\n\
            {}\
            Content-Length: {}\r\n\
            Accept-Ranges: bytes\r\n\
            Connection: Keep-Alive\r\n\
            Server: Little-Rusty\r\n\
            Content-Type: {}\r\n\r\n",
            response_status,
            content_range,
            requested_content.len(),
            file_mime_type
        );

        stream.write(response.as_bytes()).unwrap();
        stream.write(&requested_content.as_slice()).unwrap();
        match stream.flush() {
            Ok(_) => (),
            Err(e) => println!("{}: Failed stream.flush", e),
        };
    }
}
