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
-- 2. 正常插入单条记录 (带列名)
INSERT INTO users (id, name, age, bio)
VALUES (1, 'Alice', 25, 'Loves hiking');
INSERT INTO users (id, name, age, bio)
VALUES (2, 'Bob', NULL, 'Enjoys painting');
INSERT INTO products (sku, price, description)
VALUES ('SKU12345', 299, 'Smartphone');
INSERT INTO products (sku, price, description)
VALUES ('SKU67890', 99, 'Wireless Mouse');
-- 3. 正常插入多条记录 (带列名)
INSERT INTO users (id, name, age, bio)
VALUES (3, 'Charlie', 30, 'Likes swimming'),
  (4, 'Diana', 28, 'Loves photography');
INSERT INTO products (sku, price, description)
VALUES ('SKU13579', 499, 'Laptop'),
  ('SKU24680', 59, 'USB-C Hub');
-- 4. 正常插入单条记录 (不带列名，字段顺序必须和建表顺序一致)
INSERT INTO users
VALUES (5, 'Eve', 22, 'Runner and gamer');
INSERT INTO products
VALUES ('SKU99999', 150, 'Gaming Keyboard');
-- 5. 正常插入多条记录 (不带列名)
INSERT INTO users
VALUES (6, 'Frank', NULL, 'Musician'),
  (7, 'Grace', 35, 'Traveler');
INSERT INTO products
VALUES ('SKU11111', 25, 'Phone case'),
  ('SKU22222', 2999, 'Premium Laptop');
-- 6. 故意插入失败（测试异常处理）
-- 6.1 失败：users 表 id 主键重复 (id=1 已存在)
INSERT INTO users (id, name, age, bio)
VALUES (1, 'Duplicate', 40, 'Should fail - duplicate id');
-- 6.2 失败：users 表 name 非空约束，赋值 NULL
INSERT INTO users (id, name, age, bio)
VALUES (8, NULL, 24, 'Name cannot be NULL');
-- 6.3 失败：users 表 age 超过 3 位数 (1000超限)
INSERT INTO users (id, name, age, bio)
VALUES (9, 'Henry', 1000, 'Age too large');
-- 6.4 失败：products 表 sku 主键冲突 (sku='SKU12345' 已存在)
INSERT INTO products (sku, price, description)
VALUES ('SKU12345', 199, 'Duplicate SKU');
-- 6.5 失败：products 表 description 超过100字符限制
INSERT INTO products (sku, price, description)
VALUES (
    'SKU33333',
    20,
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.'
  );
-- 6.6 失败：users 表插入时不带列名，但列数不匹配（故意少一列）
INSERT INTO users
VALUES (10, 'Incomplete', 21),
  (11, 'Incomplete2', 22, 23),
  (12, 'Incomplete3', 24, 'Missing bio', 11);
-- bio列缺失
-- 6.7 失败：products 表插入时数据类型不匹配（price放了字符串）
INSERT INTO products (sku, price, description)
VALUES ('SKU44444', 'notanumber', 'Invalid price type');
DROP TABLE users;
DROP TABLE products;