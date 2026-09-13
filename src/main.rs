mod converter;
mod metadata;

fn main() {
    let target = "res/md/test.md";

    match converter::mdFileToHtml(target) {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
}
