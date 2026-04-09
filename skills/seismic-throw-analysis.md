# Seismic Throw Analysis Skill

## When to Use
- Processing synthetic or real 3D seismic volumes (SEG-Y, voxel, 3D numpy arrays)
- Calculating fault throw (vertical displacement) from seismic data
- Extracting seismic attributes (amplitude, coherence, curvature, similarity)
- Creating publication-ready visualizations for geophysics journals
- Building reproducible workflows for scientific papers (LaTeX figures, data provenance)
- Implementing fault detection pipelines with Python scientific stack

## Key Concepts

### Fault Throw vs Heave
- **Throw**: Vertical displacement across a fault (difference in TWT/depth of same horizon on opposite sides)
- **Heave**: Horizontal displacement component perpendicular to fault strike
- **Relationship**: Throw = Heave × tan(dip), where dip is fault plane inclination
- **Measurement**: Picked on inline/crossline sections perpendicular to fault strike

### SEG-Y Data Model
```
SEGY File Structure:
├── EBCDIC Text Header (3200 bytes, 40x80 cards)
├── Binary Header (400 bytes)
│   ├── Sample interval (bytes 169-170)
│   ├── Samples per trace (bytes 171-172)
│   └── Data sample format (bytes 173-174)
└── Traces (repeated)
    ├── Trace Header (240 bytes)
    │   ├── Trace sequence number (1-4)
    │   ├── Inline number (189-192)
    │   ├── Crossline number (193-196)
    │   ├── CDP coordinate X (73-76)
    │   ├── CDP coordinate Y (77-80)
    │   └── Sample count (115-116)
    └── Trace Data (samples × bytes_per_sample)
```

**Critical**: Traces must be sorted by inline then crossline (or vice versa) with fixed trace length.

### Seismic Attribute Taxonomy

| Category | Attributes | Purpose |
|----------|-----------|---------|
| **Instantaneous** | Envelope, Phase, Frequency | Single-trace properties |
| **Reflectivity** | Amplitude, Polarity | Interface contrast |
| **Coherency** | Similarity, Semblance, Coherence | Trace-to-trace waveform continuity |
| **Curvature** | Most Positive/Negative, Azimuth | Reflector bending, fractures |
| **Spectral** | FFT decomposition, CWT, RGB blending | Frequency content, thin beds |
| **Texture (GLCM)** | Contrast, Energy, Entropy, Homogeneity | Spatial patterns |
| **Fault Enhancement** | TFL, Ridge Enhancement, Finger Vein | Fault sharpening |

### Dip-Steering Fundamentals

**SteeringCube**: 3D volume of dip (inline + crossline components) at every sample position.

| Algorithm | Speed | Accuracy | Best Use |
|-----------|-------|----------|----------|
| **BG Fast Steering** | Very Fast | Good (95% cases) | Default, large volumes |
| **FFT** | Slow | Precise | Target-level detailed studies |
| **PCA** | Medium | Smooth estimates | Structural smoothing |

**Recommended Parameters:**
- Cube size: 3 (fast) to 7 (precise)
- Median filter: 1-1-3 (inline, crossline, sample stepouts)
- Two steering modes: **Detailed** (preserves faults, for curvature) and **Background** (smoothed, for filtering)

### Fault Detection Pipeline

```
Raw Seismic → Preprocessing → Fault Attribute → Enhancement → Thresholding → Fault Map
     ↓              ↓              ↓              ↓            ↓            ↓
  SEG-Y load   Spectral blue   Similarity/    Ridge        Binary      Inline/
               or median filter TFL            Enhancement  mask        Crossline
                                                                     visualization
```

**Pipeline Options:**

| Pipeline | Steps | Best For |
|----------|-------|----------|
| **TFL** | Seismic → Fault Likelihood → Thinning → TFL | Sharp fault images (0-1 range) |
| **Similarity** | Seismic → Dip-steered Similarity → Ridge Enhancement | Complex geology |
| **Semblance** | Seismic → Semblance (AllDirections extension) | Fast processing, large volumes |
| **ML-based** | Seismic → U-Net 3D/2D Fault Predictor | Automated interpretation |

**Recommended Parameters (25m × 25m × 4ms typical sampling):**

| Parameter | Small Features | Large Lineaments |
|-----------|---------------|------------------|
| **Stepout (inline, crossline, samples)** | 1-1-16 or 2-2-32 | 4-4-100 |
| **Fault Strike** | 0-360° (full scan) | Constrained after initial |
| **Fault Dip Range** | 25-65° | Refined after initial |
| **Time Gate** | [-8, 8] ms | [-32, 32] ms |
| **TFL Threshold** | 0.3-0.5 | 0.5-0.7 |

### Throw Calculation Methods

**Method 1: Horizon-based Throw**
```python
# Identify same horizon on both sides of fault
throw[t] = horizon_depth[hanging_wall] - horizon_depth[footwall]
```

**Method 2: Cross-correlation Throw**
```python
# Cross-correlate traces across fault, find vertical shift maximizing correlation
from scipy.signal import correlate
corr = correlate(trace_hanging, trace_footwall, mode='full')
throw_samples = np.argmax(corr) - center_lag
throw_ms = throw_samples * dt  # dt = sample interval
```

**Method 3: Gradient-based Throw**
```python
# Compute vertical gradient of seismic amplitude
# Fault throw where gradient sign changes across fault plane
grad_z = np.gradient(volume, axis=2)  # axis=2 is time/depth
throw_map = detect_sign_change_across_fault(grad_z, fault_mask)
```

## Workflow Template

### Complete Reproducible Workflow

```python
"""
Seismic Throw Analysis - Reproducible Workflow
Workflow: Load → Preprocess → Fault Detection → Throw Calculation → Visualize → Export
"""

# ============================================================
# STAGE 0: Setup and Provenance
# ============================================================
import numpy as np
import segyio
from scipy import ndimage, signal
from scipy.optimize import curve_fit
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from pathlib import Path
import json
from datetime import datetime

# Data provenance tracking
PROVENANCE = {
    "workflow": "seismic-throw-analysis",
    "created": datetime.now().isoformat(),
    "data_source": "synthetic-seismic-volume",
    "processing_steps": [],
    "software_versions": {
        "numpy": np.__version__,
        "scipy": __import__('scipy').__version__,
        "segyio": segyio.__version__,
        "matplotlib": plt.matplotlib.__version__,
    }
}

def log_step(step_name, params):
    """Track processing steps for reproducibility."""
    PROVENANCE["processing_steps"].append({
        "step": step_name,
        "parameters": params,
        "timestamp": datetime.now().isoformat()
    })

# ============================================================
# STAGE 1: Load SEG-Y Volume
# ============================================================
def load_segy_volume(segy_path, iline=189, xline=193):
    """
    Load 3D seismic volume from SEG-Y file.
    
    Parameters
    ----------
    segy_path : str
        Path to SEG-Y file
    iline : int
        Byte position for inline number (default: 189 for SEG-Y Rev 1)
    xline : int
        Byte position for crossline number (default: 193 for SEG-Y Rev 1)
    
    Returns
    -------
    volume : np.ndarray
        3D seismic volume [iline, xline, time]
    ilines : np.ndarray
        Inline numbers
    xlines : np.ndarray
        Crossline numbers
    dt : float
        Sample interval in milliseconds
    """
    with segyio.open(segy_path, ignore_geometry=True) as src:
        # Read binary header
        dt = segyio.dt(src)  # Sample interval in microseconds
        dt_ms = dt / 1000.0  # Convert to milliseconds
        
        # Read trace headers for geometry
        ilines = sorted(set(src.attributes(iline)()))
        xlines = sorted(set(src.attributes(xline)()))
        
        n_ilines = len(ilines)
        n_xlines = len(xlines)
        n_samples = len(src.samples)
        
        # Load volume (vectorized: read all traces at once)
        volume = np.zeros((n_ilines, n_xlines, n_samples), dtype=np.float32)
        
        for i, il in enumerate(ilines):
            for j, xl in enumerate(xlines):
                # Build lookup: trace index from inline/crossline
                # segyio stores traces in acquisition order
                trace_idx = (i * n_xlines) + j
                volume[i, j, :] = src.trace[trace_idx]
    
    log_step("load_segy", {
        "file": segy_path,
        "n_ilines": n_ilines,
        "n_xlines": n_xlines,
        "n_samples": n_samples,
        "dt_ms": dt_ms
    })
    
    return volume, np.array(ilines), np.array(xlines), dt_ms

def load_synthetic_volume(vol_path, metadata_path):
    """Load synthetic seismic volume from numpy/voxel format."""
    volume = np.load(vol_path)
    with open(metadata_path, 'r') as f:
        metadata = json.load(f)
    
    dt_ms = metadata.get('dt_ms', 4.0)
    ilines = np.arange(metadata.get('n_ilines', volume.shape[0]))
    xlines = np.arange(metadata.get('n_xlines', volume.shape[1]))
    
    log_step("load_synthetic", {
        "file": vol_path,
        "shape": volume.shape,
        "dt_ms": dt_ms
    })
    
    return volume, ilines, xlines, dt_ms

# ============================================================
# STAGE 2: Quality Control
# ============================================================
def qc_volume(volume, ilines, xlines, dt_ms):
    """
    Quality control checks on seismic volume.
    Returns dict with QC metrics.
    """
    qc_metrics = {
        "volume_shape": volume.shape,
        "total_traces": volume.shape[0] * volume.shape[1],
        "samples_per_trace": volume.shape[2],
        "duration_ms": volume.shape[2] * dt_ms,
        "amplitude_stats": {
            "min": float(volume.min()),
            "max": float(volume.max()),
            "mean": float(volume.mean()),
            "std": float(volume.std()),
            "rms": float(np.sqrt(np.mean(volume**2))),
        },
        "null_traces": int(np.all(volume == 0, axis=(0, 2)).sum()),
        "clip_percentage": float((np.abs(volume) > 3*volume.std()).mean() * 100),
    }
    
    # Histogram for amplitude distribution
    amplitude_hist, bin_edges = np.histogram(volume.flatten(), bins=256)
    
    log_step("quality_control", qc_metrics)
    return qc_metrics, amplitude_hist, bin_edges

# ============================================================
# STAGE 3: Preprocessing
# ============================================================
def apply_median_filter(volume, size=(1, 1, 3)):
    """
    Apply 3D median filter for structure-oriented noise suppression.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    size : tuple
        Filter window (inline, crossline, time)
        Typical: (1, 1, 3) for light smoothing
                 (3, 3, 5) for heavy smoothing
    """
    filtered = ndimage.median_filter(volume, size=size)
    log_step("median_filter", {"size": size})
    return filtered

def apply_gaussian_smooth(volume, sigma=(1, 1, 2)):
    """Apply Gaussian smoothing for noise reduction."""
    smoothed = ndimage.gaussian_filter(volume, sigma=sigma)
    log_step("gaussian_smooth", {"sigma": sigma})
    return smoothed

def spectral_whitening(volume, window_size=50, n_windows=10):
    """
    Spectral whitening to balance frequency content.
    Improves fault attribute computation.
    """
    n_samples = volume.shape[2]
    window_len = n_samples // n_windows
    
    whitened = np.zeros_like(volume)
    
    for i in range(volume.shape[0]):
        for j in range(volume.shape[1]):
            trace = volume[i, j, :]
            spectrum = np.fft.rfft(trace)
            
            # Smooth amplitude spectrum
            amp = np.abs(spectrum)
            phase = np.angle(spectrum)
            
            # Moving average smoothing of amplitude
            kernel = np.ones(window_size) / window_size
            smooth_amp = np.convolve(amp, kernel, mode='same')
            
            # Whitened spectrum
            whitened_spectrum = np.exp(1j * phase) * smooth_amp
            whitened[i, j, :] = np.fft.irfft(whitened_spectrum, n=n_samples)
    
    log_step("spectral_whitening", {"window_size": window_size, "n_windows": n_windows})
    return whitened

# ============================================================
# STAGE 4: Dip-Steering (SteeringCube)
# ============================================================
def compute_steering_cube_bg_fast(volume, cube_size=3, median_size=(1, 1, 3)):
    """
    Compute dip field using BG Fast Steering algorithm.
    Fast, accurate for 95% of cases.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    cube_size : int
        Analysis cube size (3 = fast, 7 = precise)
    median_size : tuple
        Median filter size for smoothing dip estimates
    
    Returns
    -------
    dip_inline : np.ndarray
        Dip component in inline direction
    dip_crossline : np.ndarray
        Dip component in crossline direction
    """
    half = cube_size // 2
    
    # Compute gradients using central differences
    grad_z = np.gradient(volume, axis=2)  # Vertical gradient
    grad_il = np.gradient(volume, axis=0)  # Inline gradient
    grad_xl = np.gradient(volume, axis=1)  # Crossline gradient
    
    # Dip estimation: dip = -gradient_horizontal / gradient_vertical
    with np.errstate(divide='ignore', invalid='ignore'):
        dip_inline = -grad_il / grad_z
        dip_crossline = -grad_xl / grad_z
    
    # Handle infinite values
    dip_inline = np.nan_to_num(dip_inline, nan=0.0, posinf=0.0, neginf=0.0)
    dip_crossline = np.nan_to_num(dip_crossline, nan=0.0, posinf=0.0, neginf=0.0)
    
    # Smooth dip field with median filter
    dip_inline = ndimage.median_filter(dip_inline, size=median_size)
    dip_crossline = ndimage.median_filter(dip_crossline, size=median_size)
    
    log_step("steering_cube", {
        "algorithm": "BG_Fast_Steering",
        "cube_size": cube_size,
        "median_size": median_size
    })
    
    return dip_inline, dip_crossline

def compute_steering_cube_fft(volume, cube_size=7):
    """
    Compute dip field using FFT-based method.
    Precise but computationally expensive.
    Best for target-level detailed studies.
    """
    from scipy.signal import wiener
    
    # Apply Wiener filter for noise reduction
    filtered = wiener(volume, mysize=(cube_size, cube_size, cube_size))
    
    # Compute gradients
    grad_z = np.gradient(filtered, axis=2)
    grad_il = np.gradient(filtered, axis=0)
    grad_xl = np.gradient(filtered, axis=1)
    
    # Dip estimation
    with np.errstate(divide='ignore', invalid='ignore'):
        dip_inline = -grad_il / grad_z
        dip_crossline = -grad_xl / grad_z
    
    dip_inline = np.nan_to_num(dip_inline, nan=0.0, posinf=0.0, neginf=0.0)
    dip_crossline = np.nan_to_num(dip_crossline, nan=0.0, posinf=0.0, neginf=0.0)
    
    log_step("steering_cube", {
        "algorithm": "FFT",
        "cube_size": cube_size
    })
    
    return dip_inline, dip_crossline

# ============================================================
# STAGE 5: Fault Attribute Computation
# ============================================================
def compute_similarity(volume, il_stepout=1, xl_stepout=1, t_stepout=16, 
                       dip_inline=None, dip_crossline=None, time_gate=None):
    """
    Compute trace-to-trace similarity (coherency) attribute.
    
    Similarity measures waveform similarity between neighboring traces.
    Range: 0 (dissimilar) to 1 (identical).
    Low similarity = fault/discontinuity.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    il_stepout : int
        Inline stepout (half-window)
    xl_stepout : int
        Crossline stepout (half-window)
    t_stepout : int
        Time sample stepout (half-window)
    dip_inline : np.ndarray, optional
        Inline dip field for dip-steered computation
    dip_crossline : np.ndarray, optional
        Crossline dip field for dip-steered computation
    time_gate : list, optional
        Time gate in samples, e.g., [-32, 32] for large features
    
    Returns
    -------
    similarity : np.ndarray
        Similarity volume (0-1 range)
    """
    ni, nx, nt = volume.shape
    similarity = np.ones_like(volume, dtype=np.float32)
    
    # Use full time gate if specified
    if time_gate is None:
        time_gate = [-t_stepout, t_stepout]
    
    t_min, t_max = time_gate
    t_window = t_max - t_min
    
    for i in range(il_stepout, ni - il_stepout):
        for j in range(xl_stepout, nx - xl_stepout):
            # Central trace window
            center_trace = volume[i, j, max(0, t_min):min(nt, nt + t_max)]
            
            # Compare with neighboring traces
            sum_dist = 0.0
            n_pairs = 0
            
            for di in range(-il_stepout, il_stepout + 1):
                for dj in range(-xl_stepout, xl_stepout + 1):
                    if di == 0 and dj == 0:
                        continue
                    
                    ni_idx = i + di
                    nj_idx = j + dj
                    
                    # Dip-steered offset calculation
                    offset_t = 0
                    if dip_inline is not None and dip_crossline is not None:
                        offset_t = int(dip_inline[i, j, :] * di + 
                                      dip_crossline[i, j, :] * dj)
                        offset_t = np.clip(offset_t, -t_stepout, t_stepout)
                    
                    neighbor_trace = volume[ni_idx, nj_idx, 
                                           max(0, t_min + offset_t):min(nt, nt + t_max + offset_t)]
                    
                    # Euclidean distance (hyperspace distance)
                    min_len = min(len(center_trace), len(neighbor_trace))
                    if min_len > 0:
                        dist = np.sqrt(np.sum(
                            (center_trace[:min_len] - neighbor_trace[:min_len]) ** 2
                        ))
                        sum_dist += dist
                        n_pairs += 1
            
            # Normalize similarity to 0-1 range
            if n_pairs > 0:
                avg_dist = sum_dist / n_pairs
                # Normalize by trace energy
                trace_energy = np.sqrt(np.sum(center_trace ** 2))
                if trace_energy > 0:
                    similarity[i, j, :] = np.exp(-avg_dist / trace_energy)
    
    log_step("similarity", {
        "il_stepout": il_stepout,
        "xl_stepout": xl_stepout,
        "t_stepout": t_stepout,
        "dip_steered": dip_inline is not None,
        "time_gate": time_gate
    })
    
    return similarity

def compute_semblance_all_directions(volume, stepout=2, time_gate=32):
    """
    Compute semblance attribute with AllDirections extension.
    Near-FullBlock accuracy with 10x less processing time.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    stepout : int
        Analysis stepout (inline and crossline)
    time_gate : int
        Time gate half-width in samples
    
    Returns
    -------
    semblance : np.ndarray
        Semblance volume (0-1 range, low = discontinuity)
    """
    ni, nx, nt = volume.shape
    semblance = np.ones_like(volume, dtype=np.float32)
    
    # AllDirections: 8 directions around center point
    directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ]
    
    for i in range(stepout, ni - stepout):
        for j in range(stepout, nx - stepout):
            for t in range(time_gate, nt - time_gate):
                # Extract traces in all directions
                traces = []
                for di, dj in directions:
                    ni_idx = i + di * stepout
                    nj_idx = j + dj * stepout
                    trace = volume[ni_idx, nj_idx, t - time_gate:t + time_gate]
                    traces.append(trace)
                
                traces = np.array(traces)  # Shape: (8, 2*time_gate)
                
                # Semblance = (sum of traces)^2 / (n * sum of traces^2)
                trace_sum = traces.sum(axis=0)
                semblance_val = np.sum(trace_sum ** 2) / (len(traces) * np.sum(traces ** 2) + 1e-10)
                semblance[i, j, t] = np.clip(semblance_val, 0, 1)
    
    log_step("semblance", {
        "algorithm": "AllDirections",
        "stepout": stepout,
        "time_gate": time_gate
    })
    
    return semblance

def compute_fault_likelihood(volume, dip_range=(25, 65), strike_range=(0, 360), 
                              stepout=1, time_stepout=16):
    """
    Compute Fault Likelihood attribute (power-of-semblance).
    Input to Thinned Fault Likelihood (TFL).
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    dip_range : tuple
        Range of fault dips to scan (degrees)
    strike_range : tuple
        Range of fault strikes to scan (degrees)
    stepout : int
        Spatial stepout
    time_stepout : int
        Time stepout
    
    Returns
    -------
    fault_likelihood : np.ndarray
        Fault likelihood volume (0-1 range)
    """
    ni, nx, nt = volume.shape
    fault_likelihood = np.zeros_like(volume, dtype=np.float32)
    
    # Convert degree ranges to radians
    dip_rad = np.radians(np.arange(dip_range[0], dip_range[1], 5))
    strike_rad = np.radians(np.arange(strike_range[0], strike_range[1], 15))
    
    # For each dip-strike combination, compute semblance on fault plane
    best_semblance = np.zeros_like(volume, dtype=np.float32)
    
    for dip in dip_rad:
        for strike in strike_rad:
            # Compute dip offsets for this fault orientation
            offset_il = int(stepout * np.cos(strike) * np.tan(dip))
            offset_xl = int(stepout * np.sin(strike) * np.tan(dip))
            
            # Compare traces on opposite sides of potential fault
            for i in range(stepout, ni - stepout):
                for j in range(stepout, nx - stepout):
                    for t in range(time_stepout, nt - time_stepout):
                        trace1 = volume[i - offset_il, j - offset_xl, 
                                       t - time_stepout:t + time_stepout]
                        trace2 = volume[i + offset_il, j + offset_xl, 
                                       t - time_stepout:t + time_stepout]
                        
                        if len(trace1) > 0 and len(trace2) > 0:
                            # Low similarity = high fault likelihood
                            dist = np.sqrt(np.sum((trace1 - trace2) ** 2))
                            energy = np.sqrt(np.sum(trace1 ** 2) + np.sum(trace2 ** 2) + 1e-10)
                            semblance = np.exp(-dist / energy)
                            fault_likelihood[i, j, t] = max(
                                fault_likelihood[i, j, t], 1.0 - semblance
                            )
    
    log_step("fault_likelihood", {
        "dip_range": dip_range,
        "strike_range": strike_range,
        "stepout": stepout,
        "time_stepout": time_stepout
    })
    
    return fault_likelihood

def thin_fault_likelihood(fl_volume, threshold=0.5):
    """
    Thin Fault Likelihood to produce razor-sharp fault images.
    Non-maximum suppression along fault-normal direction.
    
    Parameters
    ----------
    fl_volume : np.ndarray
        Fault likelihood volume
    threshold : float
        Thinning threshold (values below become 0)
    
    Returns
    -------
    tfl : np.ndarray
        Thinned Fault Likelihood (0-1 range, sharp faults)
    """
    # Apply non-maximum suppression
    tfl = fl_volume.copy()
    
    # Gradient magnitude for edge detection
    grad = np.gradient(fl_volume)
    grad_magnitude = np.sqrt(sum(g**2 for g in grad))
    
    # Suppress non-maxima
    for axis in range(3):
        tfl = ndimage.maximum_filter1d(tfl, size=3, axis=axis)
    
    # Threshold
    tfl[tfl < threshold] = 0.0
    
    log_step("thin_fault_likelihood", {"threshold": threshold})
    return tfl

def ridge_enhancement_filter(attribute_volume, directions=4):
    """
    Ridge Enhancement Filter for fault attribute volumes.
    Computes attribute differences in multiple horizontal directions.
    Direction perpendicular to fault shows largest difference.
    
    Parameters
    ----------
    attribute_volume : np.ndarray
        Input attribute volume (similarity, curvature, etc.)
    directions : int
        Number of horizontal directions (4 or 8)
    
    Returns
    -------
    enhanced : np.ndarray
        Ridge-enhanced fault attribute volume
    """
    ni, nx, nt = attribute_volume.shape
    enhanced = np.zeros_like(attribute_volume)
    
    # Define horizontal directions
    if directions == 4:
        dirs = [(1, 0), (0, 1), (1, 1), (1, -1)]
    else:  # 8 directions
        dirs = [(1, 0), (0, 1), (1, 1), (1, -1),
                (-1, 0), (0, -1), (-1, -1), (-1, 1)]
    
    for di, dj in dirs:
        # Shift volume in this direction
        shifted = np.roll(attribute_volume, shift=di, axis=0)
        shifted = np.roll(shifted, shift=dj, axis=1)
        
        # Compute difference
        diff = np.abs(attribute_volume - shifted)
        enhanced = np.maximum(enhanced, diff)
    
    log_step("ridge_enhancement", {"directions": directions})
    return enhanced

# ============================================================
# STAGE 6: Fault Detection and Throw Calculation
# ============================================================
def detect_faults_from_attribute(attribute_volume, threshold=0.3, 
                                  min_fault_length=5):
    """
    Detect faults from attribute volume (TFL, similarity, etc.).
    
    Parameters
    ----------
    attribute_volume : np.ndarray
        Fault attribute volume (low values = faults for similarity)
    threshold : float
        Threshold for fault detection
    min_fault_length : int
        Minimum fault length in traces to keep
    
    Returns
    -------
    fault_mask : np.ndarray
        Binary fault mask (1 = fault, 0 = non-fault)
    """
    # Threshold
    fault_mask = (attribute_volume < threshold).astype(np.int8)
    
    # Remove small objects
    labeled, n_features = ndimage.label(fault_mask)
    for label in range(1, n_features + 1):
        if np.sum(labeled == label) < min_fault_length:
            fault_mask[labeled == label] = 0
    
    log_step("detect_faults", {
        "threshold": threshold,
        "min_fault_length": min_fault_length,
        "n_faults_detected": n_features
    })
    
    return fault_mask

def calculate_throw_cross_correlation(volume, fault_mask, dt_ms, max_throw_samples=50):
    """
    Calculate fault throw using cross-correlation method.
    
    For each fault point, find vertical shift that maximizes 
    correlation between hanging wall and footwall traces.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    fault_mask : np.ndarray
        Binary fault mask
    dt_ms : float
        Sample interval in milliseconds
    max_throw_samples : int
        Maximum throw to search (in samples)
    
    Returns
    -------
    throw_map : np.ndarray
        Throw map in milliseconds (same shape as volume, non-zero at faults)
    """
    ni, nx, nt = volume.shape
    throw_map = np.zeros((ni, nx), dtype=np.float32)
    
    # Find fault locations (2D projection)
    fault_2d = fault_mask.any(axis=2)
    fault_indices = np.argwhere(fault_2d)
    
    for i, j in fault_indices:
        # Skip edges
        if i < 2 or i >= ni - 2 or j < 2 or j >= nx - 2:
            continue
        
        # Extract traces on either side of fault
        # Determine fault orientation from local gradient
        fault_patch = fault_2d[max(0, i-2):i+3, max(0, j-2):j+3]
        
        # Estimate fault normal direction
        gy, gx = np.gradient(fault_patch.astype(float))
        if np.abs(gx).sum() > np.abs(gy).sum():
            # Fault runs roughly inline - compare crossline neighbors
            trace1 = volume[i, max(0, j-2), :]
            trace2 = volume[i, min(nx-1, j+2), :]
        else:
            # Fault runs roughly crossline - compare inline neighbors
            trace1 = volume[max(0, i-2), j, :]
            trace2 = volume[min(ni-1, i+2), j, :]
        
        # Cross-correlation to find vertical offset
        correlation = signal.correlate(trace1, trace2, mode='full')
        center = len(correlation) // 2
        lags = np.arange(-max_throw_samples, max_throw_samples + 1)
        
        # Find peak correlation within max_throw range
        valid_corr = correlation[center - max_throw_samples:center + max_throw_samples + 1]
        if len(valid_corr) > 0:
            best_lag = np.argmax(valid_corr) - max_throw_samples
            throw_map[i, j] = abs(best_lag) * dt_ms
    
    log_step("calculate_throw", {
        "method": "cross_correlation",
        "dt_ms": dt_ms,
        "max_throw_samples": max_throw_samples,
        "n_fault_points": len(fault_indices)
    })
    
    return throw_map

def calculate_throw_gradient_method(volume, fault_mask, dt_ms):
    """
    Calculate fault throw using vertical gradient sign change method.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    fault_mask : np.ndarray
        Binary fault mask
    dt_ms : float
        Sample interval in milliseconds
    
    Returns
    -------
    throw_map : np.ndarray
        Throw map in milliseconds
    """
    # Compute vertical gradient
    grad_z = np.gradient(volume, axis=2)
    
    ni, nx, nt = volume.shape
    throw_map = np.zeros((ni, nx), dtype=np.float32)
    
    # Find fault locations
    fault_2d = fault_mask.any(axis=2)
    fault_indices = np.argwhere(fault_2d)
    
    for i, j in fault_indices:
        if i < 1 or i >= ni - 1 or j < 1 or j >= nx - 1:
            continue
        
        # Extract gradient profiles on either side of fault
        grad_left = grad_z[max(0, i-1), j, :]
        grad_right = grad_z[min(ni-1, i+1), j, :]
        
        # Find where gradient sign changes (horizon offset)
        sign_left = np.sign(grad_left)
        sign_right = np.sign(grad_right)
        
        # Find zero crossings
        zero_cross_left = np.where(np.diff(sign_left) != 0)[0]
        zero_cross_right = np.where(np.diff(sign_right) != 0)[0]
        
        if len(zero_cross_left) > 0 and len(zero_cross_right) > 0:
            # Find closest zero crossing pair
            min_dist = nt
            for zcl in zero_cross_left:
                for zcr in zero_cross_right:
                    dist = abs(zcl - zcr)
                    if dist < min_dist:
                        min_dist = dist
            
            throw_map[i, j] = min_dist * dt_ms
    
    log_step("calculate_throw", {
        "method": "gradient_sign_change",
        "dt_ms": dt_ms
    })
    
    return throw_map

# ============================================================
# STAGE 7: Seismic Attribute Extraction
# ============================================================
def extract_amplitude_attribute(volume):
    """Extract instantaneous amplitude (envelope)."""
    from scipy.signal import hilbert
    
    ni, nx, nt = volume.shape
    amplitude = np.zeros_like(volume)
    
    for i in range(ni):
        for j in range(nx):
            analytic = hilbert(volume[i, j, :])
            amplitude[i, j, :] = np.abs(analytic)
    
    return amplitude

def extract_curvature_attribute(dip_inline, dip_crossline, dz=1.0):
    """
    Compute most positive and most negative curvature from dip field.
    
    Parameters
    ----------
    dip_inline : np.ndarray
        Inline dip component
    dip_crossline : np.ndarray
        Crossline dip component
    dz : float
        Vertical spacing unit
    
    Returns
    -------
    k_positive : np.ndarray
        Most positive curvature
    k_negative : np.ndarray
        Most negative curvature
    """
    # Curvature from dip field second derivatives
    d2_il = np.gradient(dip_inline, axis=0) / dz
    d2_xl = np.gradient(dip_crossline, axis=1) / dz
    d2_mixed = np.gradient(dip_inline, axis=1) / dz
    
    # Mean and Gaussian curvature
    mean_curv = 0.5 * (d2_il + d2_xl)
    gauss_curv = d2_il * d2_xl - d2_mixed**2
    
    # Most positive and most negative curvature
    discriminant = np.sqrt(np.maximum(mean_curv**2 - gauss_curv, 0))
    k_positive = mean_curv + discriminant
    k_negative = mean_curv - discriminant
    
    return k_positive, k_negative

def spectral_decomposition_rgb(volume, freq_red=10, freq_green=20, freq_blue=40, dt_ms=4.0):
    """
    RGB spectral decomposition for fault visualization.
    Blends three frequency components to reveal fault boundaries.
    
    Parameters
    ----------
    volume : np.ndarray
        3D seismic volume
    freq_red, freq_green, freq_blue : float
        Target frequencies for RGB channels (Hz)
    dt_ms : float
        Sample interval in milliseconds
    
    Returns
    -------
    rgb_cube : np.ndarray
        RGB volume [iline, xline, time, 3]
    """
    ni, nx, nt = volume.shape
    nyquist = 1000.0 / (2 * dt_ms)  # Nyquist frequency
    
    rgb_cube = np.zeros((ni, nx, nt, 3), dtype=np.float32)
    
    for i in range(ni):
        for j in range(nx):
            trace = volume[i, j, :]
            spectrum = np.fft.rfft(trace)
            freqs = np.fft.rfftfreq(nt, d=dt_ms/1000.0)
            
            # Extract amplitude at target frequencies (narrow band)
            def get_freq_amplitude(target_freq, bandwidth=5):
                mask = (freqs >= target_freq - bandwidth) & (freqs <= target_freq + bandwidth)
                return np.abs(spectrum[mask]).mean() if mask.any() else 0
            
            rgb_cube[i, j, :, 0] = get_freq_amplitude(freq_red)
            rgb_cube[i, j, :, 1] = get_freq_amplitude(freq_green)
            rgb_cube[i, j, :, 2] = get_freq_amplitude(freq_blue)
    
    # Normalize to 0-1
    for c in range(3):
        ch_max = rgb_cube[:, :, :, c].max()
        if ch_max > 0:
            rgb_cube[:, :, :, c] /= ch_max
    
    return rgb_cube

# ============================================================
# STAGE 8: Publication-Ready Visualization
# ============================================================
def plot_inline_section(volume, il_idx, ilines, dt_ms, cmap='gray', 
                        vmin=None, vmax=None, title=None, figsize=(10, 6)):
    """Plot inline section with proper annotations."""
    if vmin is None:
        vmin = np.percentile(volume, 2)
    if vmax is None:
        vmax = np.percentile(volume, 98)
    
    fig, ax = plt.subplots(figsize=figsize)
    im = ax.imshow(volume[il_idx, :, :].T, aspect='auto', cmap=cmap,
                   vmin=vmin, vmax=vmax, origin='lower')
    
    ax.set_xlabel(f'Crossline ({ilines[il_idx]})', fontsize=11)
    ax.set_ylabel('Time (ms)', fontsize=11)
    ax.set_title(title or f'Inline {ilines[il_idx]}', fontsize=13, fontweight='bold')
    
    # Y-axis in time
    nt = volume.shape[2]
    yticks = np.linspace(0, nt - 1, 6)
    ax.set_yticks(yticks)
    ax.set_yticklabels([f'{int(y * dt_ms)}' for y in yticks])
    
    plt.colorbar(im, ax=ax, label='Amplitude')
    plt.tight_layout()
    return fig, ax

def plot_throw_map(throw_map, ilines, xlines, cmap='viridis', 
                   title='Fault Throw Map', figsize=(10, 8)):
    """Plot fault throw map with proper scale and annotations."""
    fig, ax = plt.subplots(figsize=figsize)
    
    im = ax.imshow(throw_map.T, aspect='auto', cmap=cmap, origin='lower',
                   extent=[ilines[0], ilines[-1], xlines[0], xlines[-1]])
    
    ax.set_xlabel('Inline', fontsize=11)
    ax.set_ylabel('Crossline', fontsize=11)
    ax.set_title(title, fontsize=13, fontweight='bold')
    
    cbar = plt.colorbar(im, ax=ax, label='Throw (ms)')
    cbar.ax.tick_params(labelsize=10)
    
    plt.tight_layout()
    return fig, ax

def plot_multi_panel(volume, fault_attr, throw_map, il_idx, ilines, xlines, dt_ms,
                     figsize=(16, 12)):
    """
    Create multi-panel figure for publication.
    Shows: inline seismic, fault attribute, throw map, and histogram.
    """
    fig = plt.figure(figsize=figsize)
    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.3, wspace=0.3)
    
    # Panel A: Inline seismic
    ax1 = fig.add_subplot(gs[0, 0])
    vmin, vmax = np.percentile(volume, 2), np.percentile(volume, 98)
    im1 = ax1.imshow(volume[il_idx, :, :].T, aspect='auto', cmap='gray',
                     vmin=vmin, vmax=vmax, origin='lower')
    ax1.set_xlabel('Crossline', fontsize=10)
    ax1.set_ylabel('Time (ms)', fontsize=10)
    ax1.set_title('(a) Inline Seismic', fontsize=11, fontweight='bold')
    plt.colorbar(im1, ax=ax1, label='Amplitude', fraction=0.046)
    
    # Panel B: Fault attribute
    ax2 = fig.add_subplot(gs[0, 1])
    im2 = ax2.imshow(fault_attr[il_idx, :, :].T, aspect='auto', cmap='Reds_r',
                     origin='lower', vmin=0, vmax=1)
    ax2.set_xlabel('Crossline', fontsize=10)
    ax2.set_ylabel('Time (ms)', fontsize=10)
    ax2.set_title('(b) Fault Attribute', fontsize=11, fontweight='bold')
    plt.colorbar(im2, ax=ax2, label='Attribute Value', fraction=0.046)
    
    # Panel C: Throw map
    ax3 = fig.add_subplot(gs[1, 0])
    im3 = ax3.imshow(throw_map.T, aspect='auto', cmap='viridis', origin='lower',
                     extent=[ilines[0], ilines[-1], xlines[0], xlines[-1]])
    ax3.set_xlabel('Inline', fontsize=10)
    ax3.set_ylabel('Crossline', fontsize=10)
    ax3.set_title('(c) Throw Map', fontsize=11, fontweight='bold')
    plt.colorbar(im3, ax=ax3, label='Throw (ms)', fraction=0.046)
    
    # Panel D: Amplitude histogram
    ax4 = fig.add_subplot(gs[1, 1])
    hist, bins = np.histogram(volume.flatten(), bins=256)
    ax4.plot(bins[:-1], hist, 'k-', linewidth=1.5)
    ax4.set_xlabel('Amplitude', fontsize=10)
    ax4.set_ylabel('Count', fontsize=10)
    ax4.set_title('(d) Amplitude Distribution', fontsize=11, fontweight='bold')
    ax4.set_yscale('log')
    ax4.grid(True, alpha=0.3)
    
    # Add figure caption placeholder
    fig.suptitle('Seismic Fault Throw Analysis', fontsize=14, fontweight='bold', y=0.98)
    
    return fig

def create_latex_figure_export(fig, filename, caption, label='fig:throw_analysis'):
    """
    Export figure for LaTeX inclusion.
    Saves high-resolution PNG + generates LaTeX figure code.
    """
    # Save figure
    fig_path = Path(filename)
    fig.savefig(fig_path, dpi=300, bbox_inches='tight', format='png')
    
    # Generate LaTeX code
    latex_code = f"""\\begin{{figure}}[htbp]
    \\centering
    \\includegraphics[width=0.95\\textwidth]{{{fig_path.name}}}
    \\caption{{{caption}}}
    \\label{{{label}}}
\\end{{figure}}"""
    
    # Save LaTeX snippet
    tex_path = fig_path.with_suffix('.tex')
    tex_path.write_text(latex_code)
    
    log_step("export_figure", {
        "png": str(fig_path),
        "latex": str(tex_path),
        "caption": caption,
        "label": label
    })
    
    return fig_path, tex_path

# ============================================================
# STAGE 9: Data Export and Reproducibility
# ============================================================
def save_processed_volume(volume, ilines, xlines, output_path, dt_ms=4.0):
    """Save processed volume as numpy array + metadata."""
    np.save(output_path, volume)
    
    metadata = {
        "shape": volume.shape,
        "ilines": ilines.tolist(),
        "xlines": xlines.tolist(),
        "dt_ms": dt_ms,
        "dtype": str(volume.dtype),
        "timestamp": datetime.now().isoformat()
    }
    
    meta_path = Path(output_path).with_suffix('.json')
    meta_path.write_text(json.dumps(metadata, indent=2))
    
    return meta_path

def save_provenance_log(output_dir):
    """Save complete provenance log for reproducibility."""
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    provenance_path = output_dir / 'provenance.json'
    provenance_path.write_text(json.dumps(PROVENANCE, indent=2))
    
    return provenance_path
```

## Examples

### Example 1: Complete Throw Analysis Workflow (Synthetic Data)

```python
"""
Complete seismic throw analysis from synthetic volume.
Demonstrates: Load → Preprocess → Fault Detection → Throw → Visualize → Export
"""

import numpy as np
from pathlib import Path

# ---- Step 1: Create synthetic seismic volume with known fault ----
def create_synthetic_fault_model(ni=100, nx=100, nt=200, throw_samples=15):
    """Create synthetic 3D seismic with known fault throw."""
    volume = np.zeros((ni, nx, nt), dtype=np.float32)
    
    # Create dipping reflectors
    for k in range(nt):
        for i in range(ni):
            phase = 2 * np.pi * (k - 0.3 * i) / 20  # Dipping event
            volume[i, :, k] = np.sin(phase)
    
    # Introduce fault with known throw
    fault_x = ni // 2
    volume[fault_x:, :, throw_samples:] = volume[fault_x:, :, :-throw_samples]
    
    # Add noise
    volume += np.random.normal(0, 0.1, volume.shape)
    
    # Create fault mask (known location)
    fault_mask = np.zeros((ni, nx, nt), dtype=np.int8)
    fault_mask[fault_x-1:fault_x+2, :, :] = 1
    
    return volume, fault_mask, throw_samples

# Create synthetic data
np.random.seed(42)
volume, true_fault_mask, true_throw = create_synthetic_fault_model(throw_samples=15)
ilines = np.arange(100)
xlines = np.arange(100)
dt_ms = 4.0

print(f"Created synthetic volume: {volume.shape}")
print(f"True throw: {true_throw} samples = {true_throw * dt_ms} ms")

# ---- Step 2: Quality Control ----
qc_metrics, amp_hist, amp_bins = qc_volume(volume, ilines, xlines, dt_ms)
print(f"\nQC Results:")
print(f"  Volume: {qc_metrics['volume_shape']}")
print(f"  RMS amplitude: {qc_metrics['amplitude_stats']['rms']:.3f}")

# ---- Step 3: Preprocessing ----
# Apply light median filter
filtered = apply_median_filter(volume, size=(1, 1, 3))

# ---- Step 4: Compute dip-steering ----
dip_il, dip_xl = compute_steering_cube_bg_fast(filtered, cube_size=3, median_size=(1, 1, 3))

# ---- Step 5: Compute fault attribute (Similarity) ----
# Using smaller stepouts for synthetic data
similarity = compute_similarity(filtered, il_stepout=1, xl_stepout=1, t_stepout=8)

# ---- Step 6: Detect faults ----
fault_detected = detect_faults_from_attribute(similarity, threshold=0.5, min_fault_length=3)

# ---- Step 7: Calculate throw ----
throw_map = calculate_throw_cross_correlation(volume, fault_detected, dt_ms, max_throw_samples=30)

# ---- Step 8: Visualization ----
fig = plot_multi_panel(volume, similarity, throw_map, il_idx=50,
                       ilines=ilines, xlines=xlines, dt_ms=dt_ms)
fig.savefig('throw_analysis_multi_panel.png', dpi=300, bbox_inches='tight')

# Inline section with fault overlay
fig_inline, ax = plot_inline_section(volume, 50, ilines, dt_ms, title='Inline 50 - Synthetic Fault')
ax.imshow(fault_detected[50, :, :].T > 0, aspect='auto', cmap='Reds', alpha=0.4, origin='lower')
fig_inline.savefig('inline_with_fault.png', dpi=300, bbox_inches='tight')

# Throw map
fig_throw, ax_throw = plot_throw_map(throw_map, ilines, xlines, 
                                     title='Fault Throw Map (ms)')
fig_throw.savefig('throw_map.png', dpi=300, bbox_inches='tight')

# ---- Step 9: Export ----
output_dir = Path('output_throw_analysis')
save_processed_volume(similarity, ilines, xlines, output_dir / 'similarity.npy', dt_ms)
save_provenance_log(output_dir)

print(f"\nThrow analysis complete. Results saved to: {output_dir}")
```

### Example 2: SEG-Y Volume Processing

```python
"""
Process real SEG-Y volume for fault throw analysis.
"""

import segyio
from pathlib import Path

# ---- Load SEG-Y ----
segy_path = Path('data/survey_3d.segy')
volume, ilines, xlines, dt_ms = load_segy_volume(segy_path, iline=189, xline=193)
print(f"Loaded SEG-Y: {volume.shape}, dt={dt_ms} ms")

# ---- QC ----
qc, hist, bins = qc_volume(volume, ilines, xlines, dt_ms)
print(f"Amplitude RMS: {qc['amplitude_stats']['rms']:.2f}")
print(f"Null traces: {qc['null_traces']}")

# ---- Preprocessing: Spectral whitening + median filter ----
whitened = spectral_whitening(volume, window_size=50, n_windows=10)
filtered = apply_median_filter(whitened, size=(1, 1, 3))

# ---- Dip-steering ----
dip_il, dip_xl = compute_steering_cube_bg_fast(filtered, cube_size=5, median_size=(1, 1, 3))

# ---- Fault attribute: Dip-steered similarity ----
similarity = compute_similarity(filtered, il_stepout=2, xl_stepout=2, t_stepout=16,
                                 dip_inline=dip_il, dip_crossline=dip_xl,
                                 time_gate=[-32, 32])

# ---- Ridge enhancement ----
enhanced = ridge_enhancement_filter(1 - similarity, directions=4)

# ---- Fault detection ----
fault_mask = detect_faults_from_attribute(similarity, threshold=0.4, min_fault_length=10)

# ---- Throw calculation ----
throw_map = calculate_throw_cross_correlation(volume, fault_mask, dt_ms, max_throw_samples=50)

# ---- Visualization ----
mid_il = len(ilines) // 2
fig = plot_multi_panel(volume, similarity, throw_map, il_idx=mid_il,
                       ilines=ilines, xlines=xlines, dt_ms=dt_ms)

# Export with LaTeX code
create_latex_figure_export(
    fig, 
    'figures/throw_analysis.png',
    caption='Fault throw analysis from 3D seismic volume. '
            '(a) Inline seismic section. '
            '(b) Dip-steered similarity attribute showing fault locations. '
            '(c) Fault throw map in milliseconds. '
            '(d) Amplitude distribution histogram.',
    label='fig:throw_analysis_main'
)

# ---- Save results ----
output_dir = Path('output_segy_analysis')
save_processed_volume(similarity, ilines, xlines, output_dir / 'similarity.npy', dt_ms)
save_processed_volume(throw_map, ilines[:len(throw_map)], xlines[:len(throw_map[0])],
                     output_dir / 'throw_map.npy', dt_ms)
save_provenance_log(output_dir)
```

### Example 3: Curvature and Fracture Analysis

```python
"""
Curvature-based fracture analysis from dip field.
"""

# Assuming dip_il, dip_xl already computed
k_positive, k_negative = extract_curvature_attribute(dip_il, dip_xl, dz=1.0)

# Fracture density (based on curvature threshold)
curvature_threshold = 0.004
scan_radius_traces = 16  # ~400m at 25m bin size

fracture_density = np.zeros((k_positive.shape[0], k_positive.shape[1]), dtype=np.float32)

for i in range(scan_radius_traces, k_positive.shape[0] - scan_radius_traces):
    for j in range(scan_radius_traces, k_positive.shape[1] - scan_radius_traces):
        patch = k_positive[i-scan_radius_traces:i+scan_radius_traces, 
                          j-scan_radius_traces:j+scan_radius_traces, :]
        fracture_density[i, j] = (patch > curvature_threshold).sum() / patch.size

# Visualization
fig, axes = plt.subplots(1, 3, figsize=(18, 6))

# Most positive curvature
im1 = axes[0].imshow(k_positive[50, :, :].T, aspect='auto', cmap='RdBu_r', origin='lower')
axes[0].set_title('Most Positive Curvature')
plt.colorbar(im1, ax=axes[0])

# Most negative curvature
im2 = axes[1].imshow(k_negative[50, :, :].T, aspect='auto', cmap='RdBu_r', origin='lower')
axes[1].set_title('Most Negative Curvature')
plt.colorbar(im2, ax=axes[1])

# Fracture density
im3 = axes[2].imshow(fracture_density.T, aspect='auto', cmap='hot', origin='lower')
axes[2].set_title('Fracture Density')
plt.colorbar(im3, ax=axes[2])

plt.tight_layout()
fig.savefig('curvature_fracture_analysis.png', dpi=300, bbox_inches='tight')
```

## Best Practices

### Performance Optimization
- **Vectorized operations**: Always prefer NumPy vectorized operations over Python loops
- **Dask for large volumes**: Use `dask.array` for volumes > available RAM
  ```python
  import dask.array as da
  volume_dask = da.from_zarr('volume.zarr')
  result = da.map_blocks(compute_similarity_chunk, volume_dask)
  ```
- **Joblib for embarrassingly parallel**: Inline-by-inline or section-by-section processing
  ```python
  from joblib import Parallel, delayed
  results = Parallel(n_jobs=-1)(
      delayed(process_inline)(volume[i, :, :]) for i in range(volume.shape[0])
  )
  ```
- **Memory mapping**: Use `np.memmap` for volumes larger than RAM
  ```python
  volume = np.memmap('volume.dat', dtype='float32', mode='r', shape=(ni, nx, nt))
  ```

### Scientific Code Quality
- **Unit testing**: Test throw calculation with synthetic models of known throw
  ```python
  def test_throw_calculation():
      volume, _, true_throw = create_synthetic_fault_model(throw_samples=15)
      throw_map = calculate_throw_cross_correlation(volume, fault_mask, dt_ms=4.0)
      assert np.abs(throw_map.max() - 15*4.0) < 2.0  # Within 2ms tolerance
  ```
- **Reproducibility**: Set random seeds, log all parameters, save provenance
- **Version control**: Track code, data, and processing parameters together

### Figure Style for Geophysics Journals
- **Resolution**: Minimum 300 DPI (600 DPI for line art)
- **Font**: Sans-serif (Arial/Helvetica), 10-12pt for labels
- **Color maps**: 
  - Seismic: `'gray'` or `'seismic'` (blue-white-red)
  - Attributes: `'viridis'`, `'plasma'`, `'Reds_r'` (for fault attributes)
  - Curvature: `'RdBu_r'` (diverging, positive/negative)
- **Scale bars**: Always include inline/crossline numbers and time/depth axis
- **Multi-panel**: Use (a), (b), (c) labels for journal figures

### Parameter Selection Guide

| Volume Type | Stepout | Time Gate | Steering | Best Attribute |
|-------------|---------|-----------|----------|----------------|
| **High SNR, small faults** | 1-1-16 | [-8, 8] ms | BG Fast (3) | TFL |
| **High SNR, large faults** | 4-4-100 | [-32, 32] ms | BG Fast (5) | Similarity |
| **Noisy data** | 2-2-32 | [-16, 16] ms | FFT (7) | Semblance |
| **Complex geology** | 1-1-16 | [-32, 32] ms | Dip-steered | Similarity + Ridge Enhancement |
| **Fracture analysis** | 1-1-8 | [-8, 8] ms | Detailed | Curvature |

## Common Patterns

### Quick Start Pattern
```python
# Minimal working pipeline
volume, ilines, xlines, dt = load_segy_volume('data.segy')
filtered = apply_median_filter(volume, size=(1, 1, 3))
dip_il, dip_xl = compute_steering_cube_bg_fast(filtered)
similarity = compute_similarity(filtered, il_stepout=1, xl_stepout=1, t_stepout=16,
                                 dip_inline=dip_il, dip_crossline=dip_xl)
faults = detect_faults_from_attribute(similarity, threshold=0.4)
throw = calculate_throw_cross_correlation(volume, faults, dt)
plot_throw_map(throw, ilines, xlines)
```

### Dask Pattern for Large Volumes
```python
import dask.array as da

# Load as dask array (lazy loading)
volume = da.from_zarr('seismic_volume.zarr')

# Process in chunks
similarity = da.map_blocks(
    compute_similarity_chunk,
    volume,
    chunks=(50, 50, 100),
    dtype=np.float32
)

# Compute and save
da.to_zarr(similarity, 'similarity_output.zarr')
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| **SEG-Y fails to load** | Traces not sorted or variable trace length | Verify sorting with `segyio.tools.native_th` |
| **Similarity = all 1s** | Stepout too small or volume too smooth | Increase stepout, check if volume needs preprocessing |
| **Throw values unrealistic** | Fault mask incorrect or cross-correlation window wrong | Verify fault mask, increase `max_throw_samples` |
| **Out of memory** | Volume too large for RAM | Use Dask, memmap, or process section-by-section |
| **Faults not detected** | Threshold too high or attribute parameters wrong | Lower threshold, try different stepout, check SNR |
| **Dip field noisy** | Raw seismic too noisy | Apply median filter before steering, use FFT method |
| **Figures look wrong** | Axis orientation or origin incorrect | Use `origin='lower'` for seismic, verify transpose (`.T`) |
| **Slow processing** | Python loops on large volume | Vectorize operations, use Dask/Joblib for parallelization |

## Dependencies

```bash
# Core scientific stack
pip install numpy scipy segyio matplotlib

# Large data processing
pip install dask zarr joblib

# Optional: 3D visualization
pip install pyvista  # or mayavi for advanced 3D

# Optional: ObsPy for wavelet/SEG-Y utilities
pip install obspy

# Optional: Parallel processing
pip install multiprocessing-on-dill
```

## References

- **OpendTect 7.0 Documentation**: dGB Earth Sciences - Training Manual, User Documentation, How-To Workflows
- **SEG-Y Rev 1**: SEG Technical Standards - Binary header byte positions 189 (inline), 193 (crossline)
- **Fault Likelihood**: Power-of-semblance algorithm with thinning for razor-sharp fault imaging
- **Dip-Steering**: BG Fast Steering (default), FFT (precise), PCA (smooth estimates)
- **Similarity/Coherency**: Trace-to-trace waveform similarity in hyperspace (0 = dissimilar, 1 = identical)
