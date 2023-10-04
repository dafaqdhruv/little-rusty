use std::cmp::Ordering;
use std::fs;

pub fn create_index_html(root: &std::path::PathBuf, pwd: &std::path::PathBuf) -> String {
    let html_content = String::from("<!DOCTYPE html><html lang=\"en\"><head><title>Hello, world!</title><meta charset=\"UTF-8\" /><link rel=\"icon\" href=\"data:,\"><meta name=\"description\" content=\"\" /></head>
        <body>
        <h1>Hello, world!</h1>
        This is a sample html page for little-rusty file server.
        <h3># Contents : </h3><hr>");

    if !pwd.starts_with(root) {
        return html_content;
    }

    let current_dir: fs::ReadDir = fs::read_dir(&pwd).unwrap();
    let mut files = String::new();

    for f in current_dir {
        let dir_entry = f.unwrap().path();
        let child_path = dir_entry.strip_prefix(root).unwrap();
        let tmp = dir_entry.strip_prefix(pwd).unwrap().display().to_string();

        if child_path.is_dir() {
            files = format!(
                "{}<br><a href=\"/{}\">{}/</a><br>",
                files,
                urlencoding::encode(child_path.to_str().unwrap()),
                tmp
            );
        } else {
            files = format!(
                "{}<br><a href=\"/{}\">{}</a><br>",
                files,
                urlencoding::encode(child_path.to_str().unwrap()),
                tmp
            );
        }
    }

    if pwd.cmp(root) != Ordering::Equal {
        let parent_dir = pwd
            .parent()
            .unwrap()
            .strip_prefix(root)
            .unwrap()
            .to_str()
            .unwrap();
        let parent_url = urlencoding::encode(parent_dir);

        files = format!(
            "<a href=\"/{}\">Go Back</a>\n<br>{}\n<br><hr><br><a href=\"/{}\">Go Back</a>\n<br>",
            parent_url, files, parent_url
        );
    }
    format!("{}\n{}\n</body>\n</html>", html_content, files)
}

pub fn get_mime(ext: &str) -> String {
    let audio = ["aac", "mpeg", "ogg", "mp3", "flac", "wav"];
    let text = ["css", "csv", "html", "md", "xml", "rtf", "txt"];
    let app = ["json", "zip"];
    let vid = ["avi", "mp4", "webm"];
    let img = ["gif", "jpeg", "png", "webp"];

    if audio.contains(&ext) {
        return format!("audio/{}", ext);
    } else if text.contains(&ext) {
        return match ext {
            "txt" => String::from("text/plain"),
            "md" => String::from("text/markdown"),
            _ => format!("text/{}", ext),
        };
    } else if app.contains(&ext) {
        return format!("application/{}", ext);
    } else if vid.contains(&ext) {
        return format!("video/{}", ext);
    } else if img.contains(&ext) {
        return format!("image/{}", ext);
    } else {
        return String::from("application/octect-stream");
    }
}

// download file locally
pub fn get_file() {}
