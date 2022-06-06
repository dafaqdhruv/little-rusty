use std::fs::{self};

// pub fn get_files_list(parent: &std::path::PathBuf)  {}



pub fn create_index_html(d: &std::path::PathBuf, child: &std::path::PathBuf) -> String {
    
    let html_content = String::from("<!DOCTYPE html><html lang=\"en\"><head><title>Hello, world!</title><meta charset=\"UTF-8\" /><meta name=\"description\" content=\"\" /></head>
        <body>
        <h1>Hello, world!</h1>
        This is a sample html file for my server. 
        <h3># List</h3>");

    if !child.starts_with(d) {
        return html_content;
    }

    let pwd = fs::read_dir(&child).unwrap();
    let mut files = String::new();

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

    format!("{}\n{}\n</body>\n</html>",html_content, files)
}