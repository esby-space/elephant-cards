CREATE TABLE users {
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL
}

CREATE TABLE posts {
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT REFERENCES users(id) NOT NULL,
    content TEXT NOT NULL
}

struct User {
    id: i64,
    username: String,
    password: String,
    posts: Vec<Post>
}

struct Posts {
    id: i64,
    content: String
}

