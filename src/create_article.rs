pub struct Article {
    title: String,
    content: String,
    has_images: bool,
    published: bool,
    has_meta: bool,
}

pub fn create_article(
    title: String,
    content: String,
    has_images: bool,
    published: bool,
    has_meta: bool,
) -> String {
    let article = Article {
        title,
        content,
        has_images,
        published,
        has_meta,
    };
    return article.title;
}
