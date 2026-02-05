CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    subject_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL
);
