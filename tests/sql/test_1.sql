-- 创建用户表，含主键和非空约束
CREATE TABLE users (
    id INT(10) PRIMARY KEY,      -- 主键，长度标注
    name VARCHAR(50) NOT NULL,   -- 非空用户名
    age INT(3),
    email VARCHAR(100)
);

-- 创建订单表
CREATE TABLE orders (
    order_id INT(10) PRIMARY KEY,
    user_id INT(10) NOT NULL,
    amount INT(10) NOT NULL,
    order_date VARCHAR(20)
);

INSERT INTO users (id, name, age, email) VALUES
(1, 'Alice', 30, 'alice@example.com'),
(2, 'Bob', NULL, 'bob@example.com');

-- 按列插入多行数据
INSERT INTO orders (order_id, user_id, amount, order_date) VALUES
(102, 2, 1500, '2025-05-03'),
(103, 2, 700, '2025-05-05');

-- 更新数据示例
UPDATE users SET id =12 WHERE id = 1;
SELECT * From users;
DROP table users, orders;
-- 创建用户表，含主键和非空约束
CREATE TABLE users (
    id INT(10) PRIMARY KEY,      -- 主键，长度标注
    name VARCHAR(50) NOT NULL,   -- 非空用户名
    age INT(3),
    email VARCHAR(100)
);

-- 创建订单表
CREATE TABLE orders (
    order_id INT(10) PRIMARY KEY,
    user_id INT(10) NOT NULL,
    amount INT(10) NOT NULL,
    order_date VARCHAR(20)
);

INSERT INTO users (id, name, age, email) VALUES
(1, 'Alice', 30, 'caonima@example.com'),
(2, 'Bob', NULL, 'bob@example.com');

-- 按列插入多行数据
INSERT INTO orders (order_id, user_id, amount, order_date) VALUES
(102, 2, 1500, '2025-05-03'),
(103, 2, 700, '2025-05-05');

-- 更新数据示例
UPDATE users SET id =12 WHERE id = 1;
SELECT * From users order by age ASC;