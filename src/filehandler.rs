use std::fs;

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
    
        let tmp = f.unwrap().path().strip_prefix(child).unwrap().display().to_string();
        files = format!("{}<br><a href=\"{1}\">{1}</a><br>",files, tmp);
    }

    format!("{}\n{}\n</body>\n</html>",html_content, files)
}