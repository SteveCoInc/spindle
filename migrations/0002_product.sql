BEGIN;
CREATE TABLE IF NOT EXISTS principals (
 id TEXT PRIMARY KEY, token_hash TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS projects (
 id TEXT PRIMARY KEY, name TEXT NOT NULL, blueprint TEXT NOT NULL DEFAULT 'software-v1'
);
CREATE TABLE IF NOT EXISTS memberships (
 project_id TEXT REFERENCES projects(id), principal_id TEXT REFERENCES principals(id),
 role TEXT NOT NULL CHECK(role IN ('owner','member')), PRIMARY KEY(project_id,principal_id)
);
CREATE TABLE IF NOT EXISTS activations (
 principal_id TEXT REFERENCES principals(id), workspace TEXT NOT NULL,
 project_id TEXT REFERENCES projects(id), PRIMARY KEY(principal_id,workspace)
);
CREATE TABLE IF NOT EXISTS sessions (
 id TEXT PRIMARY KEY, principal_id TEXT REFERENCES principals(id),
 project_id TEXT REFERENCES projects(id), workspace TEXT NOT NULL,
 retracted BOOLEAN NOT NULL DEFAULT FALSE,
 created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS evidence (
 id TEXT PRIMARY KEY, session_id TEXT NOT NULL REFERENCES sessions(id),
 request_key TEXT NOT NULL, text TEXT NOT NULL,
 recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
 retracted BOOLEAN NOT NULL DEFAULT FALSE, UNIQUE(session_id,request_key)
);
CREATE TABLE IF NOT EXISTS search_outbox (
 evidence_id TEXT PRIMARY KEY REFERENCES evidence(id)
);
CREATE INDEX IF NOT EXISTS evidence_session ON evidence(session_id);
CREATE INDEX IF NOT EXISTS sessions_project ON sessions(project_id);
INSERT INTO spindle_schema_migrations(version) VALUES(2) ON CONFLICT DO NOTHING;
COMMIT;
