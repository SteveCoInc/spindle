BEGIN;
ALTER TABLE memory_versions ADD COLUMN IF NOT EXISTS supersedes TEXT REFERENCES memory_versions(id);
INSERT INTO spindle_schema_migrations(version) VALUES(6) ON CONFLICT DO NOTHING;
COMMIT;
