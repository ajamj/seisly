# Well-Seismic Tie Workflow

## Overview

Well-seismic tie integrates well log data with seismic volume, enabling:
- Correlation of geological formations with seismic reflections
- Synthetic seismogram generation
- Horizon picking guidance from well markers

## Prerequisites

- Seismic volume (SEG-Y format)
- Well log data (LAS 2.0 or 3.0 format)
- Checkshot or VSP data (optional, for accurate time-depth relationship)

## Step-by-Step Workflow

### 1. Import Well Data

```bash
sf import --project MyField.sf las --well "Well-1" well1.las
```

### 2. Import Formation Tops (Optional)

```bash
sf import --project MyField.sf tops --well "Well-1" tops.csv
```

CSV format:
```csv
well_id,name,depth_md,formation
Well-1,Top Reservoir,2500.0,Formation A
Well-1,Base Seal,2700.0,Formation B
```

### 3. Create Well-Seismic Tie

In the desktop application:
1. Select well from explorer
2. Right-click → "Create Well-Seismic Tie"
3. Choose velocity parameters (V0 and k)
4. Click "Generate"

### 4. View Synthetic Seismogram

The synthetic seismogram will be displayed alongside the seismic volume at the well location.

### 5. Pick Horizons

Use formation tops as guides for horizon picking:
1. Enable "Show Formation Tops" in viewport
2. Select auto-track mode
3. Click near formation top to seed horizon

## Time-Depth Conversion

### V0 + kZ Velocity Model

The well-seismic tie uses a linear velocity model:

```
v(z) = v0 + k * z

Where:
- v0 = surface velocity (typical: 2000 m/s)
- k = velocity gradient (typical: 0.5 1/s)
- z = depth below datum

TWT(depth) = (2/k) * ln((v0 + k*depth) / v0)
```

### Typical Parameters

| Lithology | v0 (m/s) | k (1/s) |
|-----------|----------|---------|
| Sandstone | 2000 | 0.5 |
| Shale | 2200 | 0.6 |
| Limestone | 3000 | 0.3 |
| Salt | 4500 | 0.1 |

## Export

Export well tie results:

```bash
sf export --project MyField.sf well-tie --well "Well-1" --output tie.json
```

Output JSON format:
```json
{
  "id": "uuid",
  "well_id": "uuid",
  "parameters": {
    "v0": 2000.0,
    "k": 0.5
  },
  "time_depth_pairs": [
    {"depth_md": 0.0, "twt": 0.0},
    {"depth_md": 10.0, "twt": 9.95},
    ...
  ]
}
```

## Troubleshooting

### Poor Tie Quality

**Problem:** Synthetic seismogram doesn't match seismic reflections

**Solutions:**
- Check velocity parameters (try v0=1800-2500 m/s, k=0.3-0.7 1/s)
- Verify well deviation (use deviated well correction)
- Check seismic polarity (normal vs reversed)
- Ensure well log quality (remove bad sections)

### Missing Data

**Problem:** Well logs not displaying

**Solutions:**
- Ensure LAS file has GR or DT log
- Verify LAS version (2.0 or 3.0 supported)
- Check curve mnemonics (standard: GR, DT, NPHI, RHOB)

### Time-Depth Mismatch

**Problem:** Formation tops don't align with seismic reflections

**Solutions:**
- Update velocity model parameters
- Use checkshot data if available
- Consider regional velocity trends
- Verify datum elevation consistency
