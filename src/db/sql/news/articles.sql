-- Articles table to store news articles
CREATE TABLE articles (
    article_id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    summary VARCHAR(500),
    writer_id INT REFERENCES users(id),
    -- category_id INT REFERENCES categories(category_id),
    published_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

UPDATE articles SET
title = $1, content = $2, summary = $3, category_id = $4,
published = $5, updated_at = CURRENT_TIMESTAMP
WHERE id = $6;

DELETE FROM articles WHERE id = $1;

-- Categories table to store article categories
CREATE TABLE categories (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Comments table for reader comments on articles
CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    article_id INT REFERENCES articles(article_id),
    reader_id INT REFERENCES readers(reader_id),
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tags table for storing tags that can be associated with articles
CREATE TABLE tags (
    tag_id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Article tags table for many-to-many relationship between articles and tags
CREATE TABLE article_tags (
    article_id INT REFERENCES articles(article_id),
    tag_id INT REFERENCES tags(tag_id),
    PRIMARY KEY (article_id, tag_id)
);

-- Likes table to record reader likes on articles
CREATE TABLE likes (
    like_id SERIAL PRIMARY KEY,
    article_id INT REFERENCES articles(article_id),
    reader_id INT REFERENCES readers(reader_id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (article_id, reader_id)
);

-- Article views tracking table
CREATE TABLE article_views (
    view_id SERIAL PRIMARY KEY,
    article_id INT REFERENCES articles(article_id),
    reader_id INT REFERENCES readers(reader_id) NULL,
    ip_address INET,
    view_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
