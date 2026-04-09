# Product Context — Seisly

## What We're Building

**Seisly** — A professional open-source seismic interpretation platform for geoscientists, powered by Rust.

## Primary Users

- **Geophysicists** — Interpret 3D seismic volumes, pick horizons and faults, compute attributes
- **Geologists** — Build stratigraphic models, correlate wells, map depositional environments
- **Reservoir Engineers** — Monitor 4D time-lapse seismic, correlate production data with seismic anomalies
- **Research Institutions** — Subsurface characterization for geothermal, CCUS, and oil & gas exploration

## Main Goal

Provide a modern, high-performance alternative to proprietary seismic interpretation tools (OpenTect, Petrel) with:
- Real-time 3D visualization of massive seismic volumes beyond RAM capacity
- Secure, extensible AI/ML plugin system for automated interpretation
- Production-grade reliability with bounds-checked memory access and undo/redo

## Main Features

- **SEG-Y I/O** — Memory-mapped import/export with persistent sidecar indexing (O(1) access)
- **3D Visualization** — wgpu-based rendering with colormaps, wiggle traces, interactive slicing
- **Horizon Interpretation** — Manual picking, BFS auto-tracking, seed point expansion
- **Fault Modeling** — Interactive sketch mode with RBF surface generation
- **Well Integration** — LAS 2.0/3.0 support, formation tops, well-seismic tie
- **Advanced Attributes** — 20+ CPU and GPU-accelerated seismic attributes
- **Python Extensibility** — Process-isolated Python worker for safe AI/ML plugin execution
- **AVO/QI Analysis** — Class 1-4 AVO, elastic parameters, fluid substitution
- **4D Time-Lapse** — NRMS difference, time shifts, production correlation

## Current Status

- **Version:** 1.0.0 (production-grade)
- **License:** MIT OR Apache-2.0
- **Milestone:** v1.3 — Seismic Visualization & Plotting (planning complete, execution pending)
