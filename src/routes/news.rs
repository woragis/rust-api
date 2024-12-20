use crate::handlers::news::{
    articles::{
        create_article, delete_article, read_article, read_articles, update_article,
        update_article_status,
    },
    comments::{create_comment, delete_comment, edit_comment, read_comments},
    likes::{get_articles_likes, get_comments_likes, like_article, like_comment},
};
use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

pub fn news_articles_routes() -> Scope {
    scope("/news/articles")
        .route("/", get().to(read_articles))
        .route("/", post().to(create_article))
        .route("/{article_id}", get().to(read_article))
        .route("/{article_id}", put().to(update_article))
        .route("/{article_id}/status", put().to(update_article_status))
        .route("/{article_id}", delete().to(delete_article))
        // likes
        .route("/{article_id}/likes", get().to(get_articles_likes))
        .route("/{article_id}/likes", post().to(like_article))
        // comments
        .route("/{article_id}/comments", get().to(read_comments))
        .route(
            "/{article_id}/comments/{comment_id}",
            post().to(create_comment),
        )
        .route(
            "/{article_id}/comments/{comment_id}",
            put().to(edit_comment),
        )
        .route(
            "/{article_id}/comments/{comment_id}",
            delete().to(delete_comment),
        )
        // comments likes
        .route(
            "/{article_id}/comments/{comment_id}/likes",
            get().to(get_comments_likes),
        )
        .route(
            "/{article_id}/comments/{comment_id}/likes",
            post().to(like_comment),
        )
        // article tags
        .route("/{article_id}/tags", get().to(read_articles))
        .route("/{article_id}/tags/{tag_id}", put().to(update_article))
        .route("/{article_id}/tags/{tag_id}", delete().to(delete_article))
        // views
        .route("/{article_id}/views", get().to(read_articles))
        .route("/{article_id}/views", post().to(create_article))
}

pub fn news_tags_routes() -> Scope {
    scope("/news/tags")
        .route("/", get().to(read_articles))
        .route("/", post().to(create_article))
        .route("/{tag_id}", get().to(read_article))
        .route("/{tag_id}", put().to(update_article))
        .route("/{tag_id}", delete().to(delete_article))
}
