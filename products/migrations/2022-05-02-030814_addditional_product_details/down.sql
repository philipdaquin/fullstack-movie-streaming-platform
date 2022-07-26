-- This file should undo anything in `up.sql`

ALTER TABLE products DROP COLUMN category;
ALTER TABLE products DROP COLUMN created_by;
ALTER TABLE products DROP COLUMN tags;
ALTER TABLE products DROP COLUMN created_at;
ALTER TABLE products DROP COLUMN updated_at;
ALTER TABLE products DROP COLUMN description;
ALTER TABLE products DROP COLUMN image_url;