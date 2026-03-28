# StrataForge v0.2 Phase C: AI Microservice & gRPC Bridge Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the Python-based AI service using PyTorch and establish a gRPC bridge for communication with the Rust app.

**Architecture:** Python service (`sf_ai`) runs a gRPC server. Rust app acts as a client. We'll use `prost` and `tonic` in Rust and `grpcio` in Python.

**Tech Stack:** Python, PyTorch, gRPC, ProtoBuf, Tonic (Rust).

---

### Task 1: Defining the gRPC Service Contract

**Files:**
- Create: `crates/sf_core/proto/analysis.proto`

- [ ] **Step 1: Define the Detection service**

```protobuf
syntax = "proto3";
package strataforge.analysis;

service Detection {
  rpc DetectFaults (SliceRequest) returns (DetectionResponse);
}

message SliceRequest {
  bytes data = 1;
  uint32 width = 2;
  uint32 height = 3;
}

message DetectionResponse {
  bytes mask = 1;
  float confidence = 2;
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/sf_core/proto/analysis.proto
git commit -m "chore: define gRPC service contract"
```

---

### Task 2: Scaffolding Python AI Service

**Files:**
- Create: `crates/sf_ai/requirements.txt`
- Create: `crates/sf_ai/server.py`

- [ ] **Step 1: Add dependencies to requirements.txt**

```text
torch
grpcio
grpcio-tools
numpy
```

- [ ] **Step 2: Implement a basic gRPC server in server.py**

```python
import grpc
from concurrent import futures
# ... imports ...

class DetectionServicer(analysis_pb2_grpc.DetectionServicer):
    def DetectFaults(self, request, context):
        # Placeholder: Return dummy mask
        return analysis_pb2.DetectionResponse(mask=request.data, confidence=0.9)

# ... main ...
```

- [ ] **Step 3: Commit**

```bash
git add crates/sf_ai/
git commit -m "feat: scaffold Python AI service"
```

---

### Task 3: Implementing Rust gRPC Client

**Files:**
- Modify: `crates/sf_app/Cargo.toml`
- Create: `crates/sf_app/src/ai_client.rs`

- [ ] **Step 1: Add tonic and prost to Cargo.toml**

- [ ] **Step 2: Implement the client logic in ai_client.rs**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/ai_client.rs
git commit -m "feat: implement Rust gRPC client for AI analysis"
```
