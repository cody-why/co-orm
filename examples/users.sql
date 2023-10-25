-- postgresql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name varchar(50),
    password varchar(50)
);


-- mysql
CREATE TABLE users (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name varchar(255),
    password varchar(50)
);
