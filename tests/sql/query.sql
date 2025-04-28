-- 1. 创建表格
CREATE TABLE users (
  id INT PRIMARY KEY,
  -- 主键，必须唯一且非空
  name VARCHAR(20) NOT NULL,
  -- 非空，最大20字符
  age INT(3),
  -- 最多3位的整数
  bio VARCHAR -- 无长度限制的简介
);
CREATE TABLE products (
  sku VARCHAR PRIMARY KEY,
  -- 商品SKU，主键，无长度限制
  price INT,
  -- 商品价格
  description VARCHAR(100) -- 商品描述，最多100字符
);
-- 2. 插入记录 
INSERT INTO users (id, name, age, bio)
VALUES (1, 'Alice', 25, 'Loves hiking'),
  (2, 'Bob', NULL, 'Enjoys painting'),
  (3, 'Charlie', 30, 'Likes swimming'),
  (4, 'Diana', 28, 'Loves photography'),
  (5, 'Eve', 22, 'Runner and gamer'),
  (6, 'Frank', NULL, 'Musician'),
  (7, 'Grace', 35, 'Traveler');
INSERT INTO products (sku, price, description)
VALUES ('SKU12345', 299, 'Smartphone'),
  ('SKU67890', 99, 'Wireless Mouse'),
  ('SKU13579', 499, 'Laptop'),
  ('SKU24680', 59, 'USB-C Hub'),
  ('SKU99999', 150, 'Gaming Keyboard'),
  ('SKU11111', 25, 'Phone case'),
  ('SKU22222', 2999, 'Premium Laptop');
-- 3. 查询所有记录
SELECT *
FROM users;
SELECT *
FROM products;
-- 4. 查询单列记录
SELECT name
FROM users;
SELECT price
FROM products;
-- 5. 查询多列记录
SELECT name,
  age
FROM users;
SELECT sku,
  description
FROM products;
-- 6. 查询空记录
SELECT
FROM users;
SELECT
FROM products;
-- 7. 查询列名错误的记录
SELECT nam
FROM users;
SELECT pricee
FROM products;
-- 0. 删除表格
DROP TABLE users;
DROP TABLE products;