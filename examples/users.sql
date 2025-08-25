-- postgresql

CREATE TABLE users (
    id BIGSERIAL PRIMARY key not null,
    name varchar(64),
    age int,
    password varchar(32),
    update_at timestamp null default now(),
    amount numeric(10,2),
    status int default 1,
);


-- mysql

-- create database
CREATE DATABASE `hello` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

-- create table
use `hello`;
CREATE TABLE `users` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT ,
  `name` varchar(64) NOT NULL,
  `age` int(10) unsigned DEFAULT '0',
  `password` varchar(32) DEFAULT NULL,
  `update_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `amount` decimal(10,2) DEFAULT '0.00',
  `status` int(10) DEFAULT '1',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4;

-- insert
INSERT INTO `users` (`id`, `name`, `age`, `password`, `amount`, `status`) VALUES
(1, 'jack', 18, '123456', 100.00, 1),
(2, 'rose', 20, '123456', 200.00, 1),
(3, 'tom', 22, '123456', 300.00, 1),
(4, 'jerry', NULL, '123456', 300.00, 1);