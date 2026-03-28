# StrataForge Project Format

A StrataForge project is a folder that contains all data, metadata, and derived products for a subsurface interpretation project.

## Folder Structure

```
MyField.sf/
├── project.yaml          # Project manifest
├── metadata.sqlite       # SQLite database
├── blobs/                # Content-addressed blob store
│   ├── ab/
│   │   └── cd/
│   │       └── <blake3_hash>
│   └── ...
├── cache/                # Derived data cache
│   ├── tiles/            # Seismic tiles
│   ├── decimated/        # LOD meshes
│   └── thumbnails/       # Preview images
├── workflows/            # Workflow execution records
│   └── runs/
│       └── <uuid>.json
└── logs/                 # Application logs
    ├── app.log
    └── server.log
```

## project.yaml

The project manifest contains basic metadata:

```yaml
name: MyField
default_crs: EPSG:32648
created_at: '2026-03-28T10:00:00Z'
version: 0.1.0
datasets: []
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Project display name |
| `default_crs` | string | Default CRS (EPSG code recommended) |
| `created_at` | string | ISO 8601 timestamp |
| `version` | string | StrataForge format version |
| `datasets` | array | Optional dataset list (authoritative index is in SQLite) |

## metadata.sqlite

SQLite database containing structured metadata:

### Tables

**Common:**
- `datasets` - Master registry of all datasets
- `dataset_blobs` - Blob references for datasets

**Wells:**
- `wells` - Well head information
- `trajectories` - Trajectory references
- `logs` - Well log headers
- `curves` - Log curve references

**Surfaces:**
- `surfaces` - Surface metadata

**Workflows:**
- `workflow_runs` - Workflow execution records

See `schemas/sqlite/0001_init.sql` for full schema.

## Blob Store

Large binary objects are stored by BLAKE3 hash:

```
blobs/
  ab/cd/abcdef1234567890...  # 64-char hex hash
```

### Benefits

1. **Deduplication:** Identical content stored once
2. **Integrity:** Hash verified on every read
3. **Portability:** Hash is content fingerprint

### Stored Objects

- Surface meshes
- Seismic tiles
- Decimated geometry
- Workflow outputs

## Cache

Derived data stored for performance:

### tiles/

Seismic tile pyramid for fast visualization:
```
tiles/
  volume_<hash>/
    z0/  # Full resolution
    z1/  # Level 1
    ...
```

### decimated/

Level-of-detail meshes:
```
decimated/
  surface_<hash>_lod0.obj
  surface_<hash>_lod1.obj
  ...
```

## Workflows

Workflow execution records for reproducibility:

```json
{
  "id": "uuid",
  "graph": {
    "nodes": [
      {
        "id": "node1",
        "type": "import_las",
        "inputs": {"file": "well1.las"},
        "outputs": {"log_id": "uuid"},
        "parameters": {"well_name": "Well-1"}
      }
    ]
  },
  "status": "completed",
  "started_at": "2026-03-28T10:00:00Z",
  "finished_at": "2026-03-28T10:00:05Z"
}
```

## Portability

A StrataForge project folder is fully portable:

1. **Copy:** Copy entire folder to move project
2. **Archive:** Zip/tar for backup or transfer
3. **Share:** Share with collaborators

### Compact Command (planned)

```bash
sf compact --project MyField.sf --output MyField.sfz
```

Creates single-file archive with optional compression.

## Version Compatibility

| StrataForge Version | Format Version |
|---------------------|--------------|
| 0.1.x | 0.1.0 |
| 0.2.x | 0.2.0 (TBD) |

Format version changes trigger migration on open.

## Backup Recommendations

1. **Regular backups:** Copy project folder to backup location
2. **Version control:** Use git for project.yaml (not blobs)
3. **External storage:** Store blobs on object storage for large projects

## Security Notes

### v0.1
- No encryption at rest
- Local file permissions apply
- No access control

### Future
- Optional encryption (planned)
- Project-level access control (server mode)
