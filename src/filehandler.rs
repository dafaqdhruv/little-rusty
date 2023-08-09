use std::{fs::{self}, cmp::Ordering};

pub fn create_index_html(d: &std::path::PathBuf, child: &std::path::PathBuf) -> String {

    let html_content = String::from("<!DOCTYPE html><html lang=\"en\"><head><title>Hello, world!</title><meta charset=\"UTF-8\" /><meta name=\"description\" content=\"\" /></head>
        <body>
        <h1>Hello, world!</h1>
        This is a sample html page for little-rusty file server.
        <h3># Contents : </h3><hr>");

    if !child.starts_with(d) {
        return html_content;
    }

    let pwd = fs::read_dir(&child).unwrap();
    let mut files = String::new();


    // insert links to directory's contents
    for f in pwd {

        let dir_entry = f.unwrap().path();
        let child_path = dir_entry.strip_prefix(d).unwrap();
        let tmp = dir_entry.strip_prefix(child).unwrap().display().to_string();

        if child_path.is_dir() {
            files = format!("{}<br><a href=\"/{}\">{}/</a><br>",files, child_path.display().to_string(), tmp);
        } else {
            files = format!("{}<br><a href=\"/{}\">{}</a><br>",files, child_path.display().to_string(), tmp);
        }
    }

    if child.cmp(d) != Ordering::Equal {
        let parent_dir = child.parent().unwrap().strip_prefix(d).unwrap().display().to_string();
        files = format!("{}\n<br><hr><br><a href=\"/{}\">Go Back</a>\n<br>",files, parent_dir);
    }
    format!("{}\n{}\n</body>\n</html>",html_content, files)
}


// donwload file locally
pub fn get_file() {

}