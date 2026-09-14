mod about;
mod blog;
mod blog_struct;
mod converter;
mod cv;
mod guestbook;
mod index;
mod metadata;
mod webring;

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
    //    match webring::generate_webring_page() {
    //        Ok(html) => {
    //            println!("-------------------------------------\n\n\n\n\n");
    //            println!("{}", html);
    //        }
    //        Err(e) => {
    //            eprintln!("Error parsing {target}: {e}");
    //        }
    //    }
    match about::generate_about_page() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
    match cv::generate_cv_page() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
    match guestbook::generate_guestbook_page() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
}
