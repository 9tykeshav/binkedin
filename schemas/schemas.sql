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
        post_time INT NOT NULL,
        FOREIGN KEY (user_email) REFERENCES users (email)
    );