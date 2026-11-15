mod about;
mod blog;
mod blog_struct;
mod converter;
mod cv;
mod guestbook;
mod index;
mod journal;
mod metadata;
mod webring;
use std::fs;

fn generate_seo_files() -> std::io::Result<()> {
    // 1. Write robots.txt
    let robots_content = "User-agent: *\nAllow: /\nSitemap: https://skyfrei.dev/sitemap.xml\n";
    fs::write("website/robots.txt", robots_content)?;

    // 2. Write sitemap.xml (listing your main static routes)
    let sitemap_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url><loc>https://skyfrei.dev/</loc></url>
    <url><loc>https://skyfrei.dev/cv.html</loc></url>
    <url><loc>https://skyfrei.dev/blog.html</loc></url>
    <url><loc>https://skyfrei.dev/about.html</loc></url>
    <url><loc>https://skyfrei.dev/guestbook.html</loc></url>
    <url><loc>https://skyfrei.dev/journal.html</loc></url>
    <!-- Add your specific markdown/blog post slugs here dynamically if your generator tracks them -->
    <url><loc>https://skyfrei.dev/future_sugoroku.html</loc></url>
    <url><loc>https://skyfrei.dev/haskell-scraper.html</loc></url>
    <url><loc>https://skyfrei.dev/intrusive-lists.html</loc></url>
</urlset>"#;

    fs::write("website/sitemap.xml", sitemap_content)?;
    Ok(())
}

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
    match journal::generate_journal_index() {
        Ok(html) => {
            println!("-------------------------------------\n\n\n\n\n");
            println!("{}", html);
        }
        Err(e) => {
            eprintln!("Error parsing {target}: {e}");
        }
    }
    generate_seo_files().expect("Failed to generate SEO files");
    println!("SEO files generated successfully!");
}
