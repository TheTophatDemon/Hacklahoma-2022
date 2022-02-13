use serde::{Deserialize};
use crate::{Context};
use std::fs;
use std::io;

#[derive(Deserialize)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
    pub extroversion: f32
}

pub struct Friend {
    pub first_name: String,
    pub last_name: String,
    pub directory: String,
    pub icon: String, //Base 64 encoded icon image
    pub personality: Personality,
}

impl Friend {
    pub fn as_html_table_row(&self) -> String {
        format!("
            <tr>
                <td><img width='64' height='64' src=\"data:image/png;base64,{}\"></td>
                <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td> <td>{}</td>
            </tr>",
            self.icon.as_str(),
            self.first_name.as_str(), self.last_name.as_str(), 
            self.personality.openness, self.personality.conscientiousness, 
            self.personality.agreeableness, self.personality.neuroticism, self.personality.extroversion
        )
    }
}

pub fn load_friends(context: &mut Context) -> io::Result<()> {
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