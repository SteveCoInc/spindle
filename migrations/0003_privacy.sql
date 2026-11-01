BEGIN;
CREATE TABLE IF NOT EXISTS personal_exclusions (
 principal_id TEXT PRIMARY KEY REFERENCES principals(id),
 terms TEXT[] NOT NULL,
 version BIGINT NOT NULL DEFAULT 1
);
CREATE TABLE IF NOT EXISTS project_exclusions (
 project_id TEXT PRIMARY KEY REFERENCES projects(id),
 terms TEXT[] NOT NULL,
 version BIGINT NOT NULL DEFAULT 1
);
ALTER TABLE evidence ADD COLUMN IF NOT EXISTS personal_policy_version BIGINT NOT NULL DEFAULT 0;
ALTER TABLE evidence ADD COLUMN IF NOT EXISTS project_policy_version BIGINT NOT NULL DEFAULT 0;
INSERT INTO spindle_schema_migrations(version) VALUES(3) ON CONFLICT DO NOTHING;
COMMIT;
