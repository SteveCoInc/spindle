BEGIN;
ALTER TABLE memory_versions ADD COLUMN IF NOT EXISTS request_key TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS memory_request_key ON memory_versions(project_id,author,request_key);
INSERT INTO spindle_schema_migrations(version) VALUES(5) ON CONFLICT DO NOTHING;
COMMIT;
