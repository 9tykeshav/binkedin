CREATE TABLE
    users (
        email VARCHAR(100) NOT NULL PRIMARY KEY,
        password VARCHAR(12) NOT NULL
    );

CREATE TABLE
    posts (
        post_id SERIAL PRIMARY KEY,
        user_email VARCHAR(100) NOT NULL,
        caption VARCHAR(150),
        image_url VARCHAR(50),
        post_like_count INT NOT NULL,
        post_comment_count INT NOT NULL,
        post_time VARCHAR(100) NOT NULL,
        FOREIGN KEY (user_email) REFERENCES users (email)
    );


INSERT INTO posts
    (user_email, caption, image_url, 
    post_like_count, post_comment_count, 
    post_time) 
    VALUES ($1, $2, $3,$4,$5,$6)