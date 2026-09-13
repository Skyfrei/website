mod blog;
mod blog_struct;
mod converter;
mod index;
mod metadata;

fn main() {
    let target = "res/md/blogs/test.md";

    match blog::generate_blog_index() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
    match index::generate_home_page() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
}
