# Introduction to Seisly

**Seisly** (pronounced: /ˈsaɪzli/) is a modern, open-source seismic interpretation platform built in Rust.

> **🎯 Vision:** The fastest, most accessible seismic studio - from exploration to production.

Seisly aims to provide geoscientists with a high-performance, cross-platform tool for seismic data visualization, interpretation, and analysis. Leveraging the safety and speed of Rust, along with GPU acceleration via `wgpu`, Seisly handles large seismic datasets with ease.

## Key Features

- **Seismic Visualization**: 3D volume rendering with inline, crossline, and time slice viewing.
- **Horizon Interpretation**: Tools for manual picking, seed picking, and ML-assisted auto-tracking.
- **Fault Modeling**: Interactive fault sketching and RBF-based surface modeling.
- **Well Data Integration**: Support for LAS well logs, trajectory visualization, and well-seismic tie.
- **Velocity Modeling**: Real-time time-to-depth conversion with flexible velocity models.
- **Seismic Attributes**: Comprehensive suite of CPU and GPU-accelerated attributes.
- **Plugin System**: Extend Seisly's functionality with custom Rust or Python plugins.

## Why Seisly?

| Metric | Seisly | Traditional Tools |
|--------|--------|-------------------|
| **Startup** | < 2s | 30-60s |
| **Size** | < 100MB | 2GB - 5GB+ |
| **License** | MIT (Free) | Expensive Proprietary |
| **Engine** | Rust + GPU | Legacy C++/.NET |
