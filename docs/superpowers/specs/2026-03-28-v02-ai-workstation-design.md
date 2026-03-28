# StrataForge v0.2: The AI-First Workstation Design

**Date:** 2026-03-28  
**Phase:** v0.2 - Interactive Seismic & AI Integration  
**Status:** Approved by User

## 1. Overview
StrataForge v0.2 transitions the project from a CLI-based toolset to a functional 3D interpretation workstation. The primary goal is to provide a unified environment for visualizing 3D seismic data and running AI-based fault detection using a hybrid Rust-Python architecture.

## 2. Architecture: Hybrid Microservice
To leverage the best of both worlds, we employ a "Microservice" approach:
- **Frontend/Core (Rust):** High-performance rendering (wgpu), large-scale data management (SEG-Y), and the primary desktop user interface (egui).
- **AI Service (Python):** Deep learning inference using PyTorch, specifically optimized for 3D convolutional neural networks (CNNs).
- **Communication Bridge:** gRPC over local loopback for low-latency, structured data exchange (ProtoBuf).

## 3. Key Components

### 3.1. Desktop Application (`sf_app`)
- **UI Framework:** `egui` for a responsive, dark-themed interpretation environment.
- **3D Engine:** `wgpu`-based scene manager capable of rendering seismic slices (Inline, Crossline, Z-slice) and well trajectories.
- **Project Explorer:** Tree-view to manage seismic volumes, wells, and surfaces.

### 3.2. Seismic Engine (`sf_compute` + `sf_io`)
- **SEG-Y Support:** Efficient parsing and memory-mapped access to 3D seismic volumes.
- **Volume Slicing:** Real-time extraction of 2D slices for visualization and AI analysis.

### 3.3. AI Microservice (`sf_ai`)
- **Framework:** PyTorch.
- **Model:** Initial focus on 3D/2D U-Net or similar architecture for **Fault Detection**.
- **Service:** A Python server using `grpcio` to listen for inference requests.

### 3.4. gRPC Bridge
- **Contract:** Defined in `sf_core/proto/analysis.proto`.
- **Flow:** 
    1. Rust app sends a seismic slice (tensor) and metadata to Python.
    2. Python returns a probability mask (tensor) for the detected feature (faults).
    3. Rust app overlays the mask on the 3D scene.

## 4. Data Flow
1. **Load:** User imports/opens a SEG-Y volume in the Rust app.
2. **View:** Rust renders the 3D slices using `wgpu`.
3. **Analyze:** User navigates to a slice and clicks "Run Fault Detection".
4. **Bridge:** Rust sends the slice data via gRPC to the Python service.
5. **Inference:** Python runs the PyTorch model on the GPU (if available).
6. **Overlay:** Python returns the fault mask; Rust renders it as a semi-transparent overlay in the 3D viewer.

## 5. Error Handling & Constraints
- **Connectivity:** Handle "Service Not Found" gracefully if the Python AI service isn't running.
- **Performance:** Limit slice sizes for real-time analysis to prevent UI hangs.
- **Platform:** Initial support for Windows (win32) with a focus on local execution.

## 6. Success Criteria
- [ ] Rust desktop app launches with a functional 3D viewer.
- [ ] Successfully load and slice a 3D SEG-Y volume.
- [ ] Python AI service can receive a slice and return a (mocked or real) fault mask.
- [ ] End-to-end visualization of AI results over seismic data.
