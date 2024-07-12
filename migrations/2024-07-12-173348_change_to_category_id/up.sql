ALTER TABLE posts ADD COLUMN category_id INT;

ALTER TABLE posts DROP COLUMN category;

ALTER TABLE posts
ADD CONSTRAINT fk_category
FOREIGN KEY (category_id) REFERENCES post_categories(id);
