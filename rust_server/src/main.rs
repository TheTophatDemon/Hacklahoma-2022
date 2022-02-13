use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::io;
use std::vec::Vec;
use serde::{Deserialize};
use image::io::Reader as ImageReader;

#[derive(Deserialize)]
struct Personality {
    openness: f32,
    conscientiousness: f32,
    agreeableness: f32,
    neuroticism: f32,
    extroversion: f32
}

struct Friend {
    first_name: String,
    last_name: String,
    directory: String,
    icon: String, //Base 64 encoded icon image
    personality: Personality,
}

struct Context {
    friends: Vec<Friend>
}

fn main() {
    let mut context = Context {
        friends: Vec::new()
    };

    load_friends(&mut context).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(&mut context, stream),
            Err(_) => println!("Connection failed!")
        }
    }
}

fn load_friends(context: &mut Context) -> io::Result<()> {
    for dir_entry in fs::read_dir("friends")? {
        let dir_entry = dir_entry?;
        let friend_path = dir_entry.path();
        if friend_path.is_dir() {
            for entry in fs::read_dir(friend_path.clone())? {
                let entry = entry?;
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name == "info.json" {
                        let data: serde_json::Value = serde_json::from_str(
                            fs::read_to_string(entry.path())?
                            .as_str()
                        )?;
                        context.friends.push(parse_friend(friend_path.to_str().unwrap(), data));
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_friend(path: &str, data: serde_json::Value) -> Friend {
    let first_name = match data["first name"].clone() {
        serde_json::Value::String(string) => string,
        _ => String::new()
    };
    let last_name = match data["last name"].clone() {
        serde_json::Value::String(string) => string,
        _ => String::new()
    };
    let personality = serde_json::from_value(data["personality"].clone()).unwrap();
    
    let icon = base64::encode(fs::read(path.to_owned() + "/icon.png").unwrap());

    return Friend {
        first_name,
        last_name,
        directory: path.trim_start_matches("/").to_owned(),
        icon,
        personality
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
                    friends_list.push_str(format!("
                        <tr>
                            <td><img width='64' height='64' src=\"data:image/png;base64,{}\"></td>
                            <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td>
                        </tr>",
                        friend.icon.as_str(),
                        friend.first_name.as_str(), friend.last_name.as_str(), 
                        friend.personality.openness, friend.personality.conscientiousness, 
                        friend.personality.agreeableness, friend.personality.neuroticism, friend.personality.extroversion
                    ).as_str());
                }
                let contents = fs::read_to_string("templates/friends.html").unwrap()
                    .replace("{{FRIENDS}}", friends_list.as_str());
                Some(contents)
            },
            _ if uri.ends_with(".png") => {
                match ImageReader::open(uri.trim_start_matches("/")) {
                    Ok(reader) => match reader.decode() {
                        Ok(image) => Some(base64::encode(image.into_rgba8().into_raw())),
                        _ => None
                    },
                    Err(reason) => {
                        println!("Error opening image {} : {}", uri, reason);
                        None
                    }
                }
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