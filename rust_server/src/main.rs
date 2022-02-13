use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::vec::Vec;

mod friend; use friend::{Friend};
mod personality;
mod action;

pub struct Context {
    friends: Vec<Friend>
}

fn main() {
    let mut context = Context {
        friends: Vec::new()
    };

    friend::load_friends(&mut context).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(&mut context, stream),
            Err(_) => println!("Connection failed!")
        }
    }
}

fn route(context: &mut Context, request:String) -> Option<String> {
    let mut iter = request.split_ascii_whitespace();
    if iter.next().unwrap_or_default() == "GET" {
        let uri = iter.next().unwrap_or_default();
        match uri {
            "/" => Some(
                fs::read_to_string("templates/druzya_main.html").unwrap()
            ),
            "/friends" => {
                let mut friends_list = String::new();
                for friend in context.friends.iter() {
                    friends_list.push_str(friend.as_html_table_row().as_str());
                }
                let contents = fs::read_to_string("templates/friends.html").unwrap()
                    .replace("{{FRIENDS}}", friends_list.as_str());
                Some(contents)
            },
            _ => None
        }
    } else {
        None
    }
}

fn handle_connection(context: &mut Context, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let (status_line, contents) = if let Some(template) = route(context, String::from_utf8_lossy(&buffer[..]).into_owned()) {
        ("HTTP/1.1 200 OK", template)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "templates/404.html".to_owned())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}