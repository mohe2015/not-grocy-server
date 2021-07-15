-- change type of stock.opened_date from timestamp to date

ALTER TABLE stock ADD COLUMN tmp_opened_date DATE;

UPDATE stock SET tmp_opened_date = opened_date;

ALTER TABLE stock DROP COLUMN opened_date;

ALTER TABLE stock RENAME COLUMN tmp_opened_date TO opened_date;
