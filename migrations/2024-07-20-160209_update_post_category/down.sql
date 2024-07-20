ALTER TABLE posts
    ALTER COLUMN category_id DROP NOT NULL;

ALTER TABLE posts
    ALTER COLUMN category_id DROP DEFAULT;
    