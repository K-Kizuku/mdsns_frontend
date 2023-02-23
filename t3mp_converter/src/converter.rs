use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};

use html_minifier::HTMLMinifier;

use easy_scraper::Pattern;

extern crate html_escape;

fn read_file(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let md = fs::read_to_string(file_path.to_str().unwrap())?;
    Ok(md)
}

fn write_html_file(
    file_path: &Path,
    html: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    write!(file, "{}", html)?;
    Ok(())
}

fn get_title_from_html(html_file: &str) -> String {
    let pat = Pattern::new(r#"
    <h1>{{title}}</h1>
    "#).unwrap();
    let matches = pat.matches(html_file);

    matches[0]["title"].clone()
}

fn markdown_to_html(input_path: &Path) -> String {
    let markdown_input = read_file(input_path).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn decorate_html(
    html_file: &str,
    css_path: &Path/*,
    js_path: &std::path::Path,*/
) -> String {
    let mut html_minifier = HTMLMinifier::new();

    // header
    html_minifier.digest("<!DOCTYPE html>").unwrap();
    html_minifier.digest("<html>").unwrap();
    html_minifier.digest("<head>").unwrap();
    html_minifier.digest("<meta charset=UTF-8>").unwrap();
    html_minifier.digest("<title>").unwrap();
    html_minifier.digest(get_title_from_html(html_file)).unwrap();
    html_minifier.digest("</title>").unwrap();
    //CSS
    // TODO: Eventually, Getting the css from GitHub.
    let css_file = read_file(css_path).unwrap();
    html_minifier.digest("<style>").unwrap();
    html_minifier.digest(css_file).unwrap();
    html_minifier.digest("</style>").unwrap();
    // code highlight
    /* !! highlight_* path is temporary !! */
    if html_file.contains("</code></pre>") {
        html_minifier.digest("<script>").unwrap();
        let highlight_js = read_file(Path::new("./test/highlight.js")).unwrap();
        html_minifier.digest(html_escape::encode_script(&highlight_js).as_ref()).unwrap();
        html_minifier.digest("</script>").unwrap();
        html_minifier.digest("<style>").unwrap();
        let highlight_css = read_file(Path::new("./test/highlight.css")).unwrap();
        html_minifier.digest(highlight_css).unwrap();
        html_minifier.digest("</style>").unwrap();
    }
    html_minifier.digest("</head>").unwrap();
    html_minifier.digest("<body>").unwrap();
    html_minifier.digest("<article class=\"markdown-body\">").unwrap();
    // from markdown_to_html
    html_minifier.digest(&html_file).unwrap();
    html_minifier.digest("</article>").unwrap();
    html_minifier.digest("</body>").unwrap();
    html_minifier.digest("</html>").unwrap();

    let minified_html = match std::str::from_utf8(html_minifier.get_html()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    String::from(minified_html)

}

pub fn converter(
    input_md_path: &Path,
    output_html_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    let plane_html = markdown_to_html(input_md_path);
    /* !! CSS path is temporary !! */
    let css_path = Path::new("./test/test.css");
    let minified_html = decorate_html(&plane_html, css_path);
    write_html_file(output_html_path, &minified_html).unwrap();

    Ok(())
}

#[cfg(test)]
mod converter_tests {
    use super::*;

    #[test]
    fn markdown2html_test() {
        let input_md_path = std::path::Path::new("./test/test.md");
        let output_html_path = std::path::Path::new("./test/test.html");

        let html_file = markdown_to_html(input_md_path);

        write_html_file(output_html_path, &html_file).unwrap();

        let cmd_result = std::process::Command::new("file")
            .arg(output_html_path)
            .output()
            .expect("Failed to execute file command");

        assert_eq!(b"./test/test.html: HTML document, Unicode text, UTF-8 text\n",
                    cmd_result.stdout.as_slice());
    }

    #[test]
    #[ignore]
    fn decorate_html_test() {
        markdown2html_test();
        let input_html_path = std::path::Path::new("./test/test.html");
        let output_html_path = std::path::Path::new("./test/test_minified.html");
        let css_path = std::path::Path::new("./test/test.css");

        let plane_html = read_file(input_html_path).unwrap();

        let output_html = decorate_html(&plane_html, css_path);

        write_html_file(output_html_path, &output_html).unwrap();

        std::process::Command::new("firefox")
            .arg(output_html_path)
            .output()
            .expect("Failed to execute Firefox");
    }

    #[test]
    #[ignore]
    fn get_title_test() {
        markdown2html_test();
        let input_html_path = std::path::Path::new("./test/test.html");
        let plane_html = read_file(input_html_path).unwrap();

        assert_eq!(b"title", get_title_from_html(&plane_html).as_bytes());
    }

    #[test]
    #[ignore]
    fn converter_test() {
        let input_md_path = std::path::Path::new("./test/test.md");
        let output_html_path = std::path::Path::new("./test/test.html");

        converter(input_md_path, output_html_path).unwrap();

        std::process::Command::new("firefox")
            .arg(output_html_path)
            .output()
            .expect("Failed to execute Firefox");
    }

}

