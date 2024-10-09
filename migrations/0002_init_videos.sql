CREATE TABLE videos (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    raw_file_path TEXT NOT NULL,
    processed_file_path TEXT,
    processing_status TEXT NOT NULL DEFAULT 'pending'
);
