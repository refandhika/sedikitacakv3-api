ALTER TABLE posts
    ALTER COLUMN category_id SET DEFAULT 1;

UPDATE posts
    SET category_id = 1
    WHERE category_id IS NULL;

ALTER TABLE posts
    ALTER COLUMN category_id SET NOT NULL;