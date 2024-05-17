-- postgresql

CREATE TABLE users (
    id BIGSERIAL PRIMARY key not null,
    name varchar(64),
    age int,
    password varchar(32),
    update_at timestamp null default now(),
    amount numeric(10,2)
);


-- mysql
CREATE TABLE `users` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT ,
  `name` varchar(64) NOT NULL,
  `age` int(10) unsigned DEFAULT '0',
  `password` varchar(32) DEFAULT NULL,
  `update_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `amount` decimal(10,2) DEFAULT '0.00',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4;