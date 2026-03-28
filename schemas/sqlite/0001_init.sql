-- StrataForge SQLite Schema v1
-- Common datasets table
CREATE TABLE IF NOT EXISTS datasets (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    crs_def TEXT NOT NULL,
    created_at TEXT NOT NULL,
    tags_json TEXT DEFAULT '[]',
    provenance_json TEXT
);

-- Dataset blob references
CREATE TABLE IF NOT EXISTS dataset_blobs (
    dataset_id TEXT NOT NULL,
    blob_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    PRIMARY KEY (dataset_id, blob_hash, role),
    FOREIGN KEY (dataset_id) REFERENCES datasets(id) ON DELETE CASCADE
);

-- Wells
CREATE TABLE IF NOT EXISTS wells (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    crs_def TEXT NOT NULL,
    head_x REAL NOT NULL,
    head_y REAL NOT NULL,
    kb_elevation REAL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (id) REFERENCES datasets(id) ON DELETE CASCADE
);

-- Trajectories (stations stored as binary blob)
CREATE TABLE IF NOT EXISTS trajectories (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL,
    stations_blob_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (well_id) REFERENCES wells(id) ON DELETE CASCADE
);

-- Logs
CREATE TABLE IF NOT EXISTS logs (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL,
    depth_mnemonic TEXT NOT NULL,
    depth_unit TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (well_id) REFERENCES wells(id) ON DELETE CASCADE
);

-- Curves (values stored as binary blob)
CREATE TABLE IF NOT EXISTS curves (
    id TEXT PRIMARY KEY,
    log_id TEXT NOT NULL,
    mnemonic TEXT NOT NULL,
    unit TEXT NOT NULL,
    null_value REAL NOT NULL,
    values_blob_hash TEXT NOT NULL,
    FOREIGN KEY (log_id) REFERENCES logs(id) ON DELETE CASCADE
);

-- Surfaces
CREATE TABLE IF NOT EXISTS surfaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    crs_def TEXT NOT NULL,
    mesh_blob_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (id) REFERENCES datasets(id) ON DELETE CASCADE
);

-- Workflow runs
CREATE TABLE IF NOT EXISTS workflow_runs (
    id TEXT PRIMARY KEY,
    graph_json TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    started_at TEXT,
    finished_at TEXT,
    logs_blob_hash TEXT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_datasets_type ON datasets(type);
CREATE INDEX IF NOT EXISTS idx_wells_name ON wells(name);
CREATE INDEX IF NOT EXISTS idx_trajectories_well ON trajectories(well_id);
CREATE INDEX IF NOT EXISTS idx_logs_well ON logs(well_id);

-- Faults
CREATE TABLE IF NOT EXISTS faults (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color_r REAL NOT NULL,
    color_g REAL NOT NULL,
    color_b REAL NOT NULL,
    is_visible INTEGER NOT NULL DEFAULT 1
);

-- Fault Sticks
CREATE TABLE IF NOT EXISTS fault_sticks (
    id TEXT PRIMARY KEY,
    fault_id TEXT NOT NULL,
    name TEXT NOT NULL,
    picks_blob BLOB NOT NULL, -- Serialized Vec<[f32; 3]>
    FOREIGN KEY(fault_id) REFERENCES faults(id) ON DELETE CASCADE
);

-- Seismic Volumes (for multi-volume management)
CREATE TABLE IF NOT EXISTS seismic_volumes (
    id TEXT PRIMARY KEY,
    dataset_id TEXT NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    is_visible INTEGER NOT NULL DEFAULT 1,
    channel_assignment INTEGER NOT NULL DEFAULT 0, -- 0: None, 1: Red, 2: Green, 3: Blue
    FOREIGN KEY(dataset_id) REFERENCES datasets(id) ON DELETE CASCADE
);
