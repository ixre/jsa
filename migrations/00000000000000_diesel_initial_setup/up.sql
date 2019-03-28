-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE domains (id  SERIAL NOT NULL, user_id int4 NOT NULL, hash varchar(20) NOT NULL, domain varchar(40) NOT NULL, flag int2 NOT NULL, state int2 NOT NULL, notes varchar(255) NOT NULL, create_time int4 NOT NULL, PRIMARY KEY (id));
COMMENT ON TABLE domains IS '域名';
COMMENT ON COLUMN domains.id IS '编号';
COMMENT ON COLUMN domains.user_id IS '用户编号';
COMMENT ON COLUMN domains.hash IS 'hash值';
COMMENT ON COLUMN domains.domain IS '域名';
COMMENT ON COLUMN domains.flag IS '标志';
COMMENT ON COLUMN domains.state IS '状态';
COMMENT ON COLUMN domains.notes IS '备注';
COMMENT ON COLUMN domains.create_time IS '创建时间';
