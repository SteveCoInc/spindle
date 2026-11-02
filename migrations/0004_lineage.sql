BEGIN;
CREATE TABLE IF NOT EXISTS memory_versions (
 id TEXT PRIMARY KEY,
 project_id TEXT NOT NULL REFERENCES projects(id),
 author TEXT NOT NULL REFERENCES principals(id),
 text TEXT NOT NULL,
 active BOOLEAN NOT NULL DEFAULT TRUE,
 transformation TEXT NOT NULL CHECK(transformation IN ('manual_synthesis')),
 recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS memory_inputs (
 memory_id TEXT NOT NULL REFERENCES memory_versions(id),
 input_id TEXT NOT NULL,
 evidence_id TEXT REFERENCES evidence(id),
 parent_memory_id TEXT REFERENCES memory_versions(id),
 PRIMARY KEY(memory_id,input_id),
 CHECK((evidence_id IS NOT NULL AND parent_memory_id IS NULL AND input_id=evidence_id)
 OR (evidence_id IS NULL AND parent_memory_id IS NOT NULL AND input_id=parent_memory_id))
);
CREATE INDEX IF NOT EXISTS memory_inputs_reverse ON memory_inputs(input_id);
CREATE TABLE IF NOT EXISTS memory_outbox (
 memory_id TEXT PRIMARY KEY REFERENCES memory_versions(id)
);
INSERT INTO spindle_schema_migrations(version) VALUES(4) ON CONFLICT DO NOTHING;
COMMIT;
