# StrataForge Pro - Strategic Blueprint (2026-2028)

> **Vision:** Transform StrataForge from v0.1.1 Beta into "StrataForge Pro" - an open-core competitor to Petrel, DUG Insight, and OpendTect Pro.

---

## 1. Vision & Positioning

### Product Name
- **StrataForge OSS** - Free open-source version (academic & small companies)
- **StrataForge Pro** - Commercial version with advanced features

### Tagline
> "Rust-powered Seismic Studio – Lightning fast, fully open, no license hell."

### Unique Selling Points (USP) vs Competitors

| Feature | Petrel (C#) | DUG Insight | OpendTect Pro | **StrataForge Pro** |
|---------|-------------|-------------|---------------|---------------------|
| Performance | .NET overhead | C++ legacy | C++ legacy | **Rust + GPU-native** |
| All-in-One | ✅ | ✅ | ⚠️ Plugin-based | **✅ Processing + Interpretation + QI** |
| Plugin System | Limited | Limited | ✅ Python | **✅ Rust + Python (PyO3)** |
| License Model | Expensive | Subscription | Open-core | **Open-core (free academic)** |
| Cloud-ready | ⚠️ | ⚠️ | ❌ | **✅ Cloud-first + HPC** |
| Install Size | ~5GB | ~3GB | ~2GB | **<500MB** |
| Startup Time | 30-60s | 15-30s | 10-20s | **<5s** |

### Target Market
- **Primary:** Indonesia & Malaysia (oil & gas, geothermal, CCUS)
- **Secondary:** Global indie & mid-size exploration companies
- **Academic:** Universities & research institutions (free forever)

---

## 2. High-Level Architecture

### Modular Crate Structure (Enhanced)

```
StrataForge (Monorepo)
├── crates/
│   ├── sf_core          ← Domain logic (seismic, horizon, fault, velocity)
│   ├── sf_io            ← SEG-Y, LAS 2/3, ZGY, SEG-D, well trajectory
│   ├── sf_compute       ← Attributes, inversion, ML inference [NEW]
│   ├── sf_render        ← wgpu 3D + multi-volume blending + VR/AR ready
│   ├── sf_storage       ← SQLite + Blob + OSDU/S3 connector [ENHANCED]
│   ├── sf_app           ← egui desktop (Windows/Linux/macOS)
│   ├── sf_cli           ← sf CLI (existing)
│   ├── sf_server        ← REST + gRPC server [NEW]
│   ├── sf_plugin        ← Plugin system (Rust + Python via PyO3) [NEW]
│   └── sf_ml            ← ONNX + candle (Rust ML) [NEW]
├── plugins/             ← Community plugins repository
├── python/              ← Python bindings (PyO3/maturin) [NEW]
├── docs/                ← User & developer documentation
├── website/             ← Marketing site [NEW]
└── .github/workflows/   ← CI/CD multi-platform + auto-release
```

### Technology Stack

| Category | Technology | Purpose |
|----------|------------|---------|
| **Core** | Rust 1.70+ | Performance & safety |
| **GPU** | wgpu | Cross-platform rendering |
| **UI** | egui/eframe | Immediate mode GUI |
| **Database** | rusqlite | Local metadata storage |
| **ML** | candle (Rust), ONNX Runtime | Auto-tracking, fault detection |
| **Python** | PyO3 + maturin | Plugin ecosystem |
| **Geospatial** | PROJ, GDAL (rust-binding) | CRS transforms |
| **Server** | axum, tonic | REST + gRPC APIs |
| **Cloud** | Kubernetes, GPU operator | StrataForge Cloud |
| **Formats** | SEG-Y, LAS 2/3, ZGY, SEG-D | Industry standard I/O |

### Integration Points

```
┌─────────────────────────────────────────────────────────────┐
│                    StrataForge Desktop                       │
│                    (egui + wgpu)                             │
├─────────────────────────────────────────────────────────────┤
│                      Plugin Layer                            │
│            (Rust plugins + Python via PyO3)                  │
├─────────────────────────────────────────────────────────────┤
│                    Core Engine                               │
│   sf_core │ sf_io │ sf_compute │ sf_render │ sf_ml          │
├─────────────────────────────────────────────────────────────┤
│                    Storage Layer                             │
│      SQLite (local) │ PostgreSQL (server) │ S3 (cloud)      │
├─────────────────────────────────────────────────────────────┤
│                    External Connectors                       │
│    Petrel (.pet) │ OpendTect (.dgb) │ OSDU │ ZGY (Zion)    │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Feature Roadmap (4 Phases)

### Phase 0 - Foundation (v0.1.1 → v0.2.0)
**Timeline:** 4-6 weeks | **Priority:** HIGH ✅

| Feature | Status | Crate | Notes |
|---------|--------|-------|-------|
| LAS 2.0/3.0 complete | ⚠️ Partial | sf_io | Add v3.0 support |
| Well-seismic tie | ❌ | sf_compute | New module |
| Formation tops | ❌ | sf_core | New domain model |
| SEG-Y full reader/writer | ⚠️ Partial | sf_io | Complete implementation |
| Project format v2 | ❌ | sf_storage | Enhanced manifest |

**Deliverables:**
- ✅ Complete well data workflow (import → visualize → tie)
- ✅ Full SEG-Y support (binary + textual headers)
- ✅ Formation tops mapping on wells
- ✅ Well-seismic tie visualization

---

### Phase 1 - MVP Pro (v0.3.0)
**Timeline:** 3 months | **Priority:** HIGH

| Feature | Status | Crate | Notes |
|---------|--------|-------|-------|
| Advanced auto-tracking (ML) | ❌ | sf_ml | CNN-based picker |
| Multi-volume blending | ⚠️ Basic | sf_render | Enhance RGB shader |
| Surface clipping | ❌ | sf_compute | Plane + mesh cutting |
| Volumetrics (GRV) | ❌ | sf_compute | Grid-based integration |
| Seismic attributes (100+) | ❌ | sf_compute | Amplitude, frequency, phase |
| Plugin system scaffold | ❌ | sf_plugin | Rust + Python API |

**Deliverables:**
- ✅ ML-powered horizon auto-tracking
- ✅ Advanced attribute computation
- ✅ Surface analysis tools
- ✅ Plugin API v1

---

### Phase 2 - Full Interpretation (v1.0)
**Timeline:** 5-6 months | **Priority:** HIGH

| Feature | Status | Crate | Notes |
|---------|--------|-------|-------|
| Pre-stack QI | ❌ | sf_compute | AVO analysis |
| Simultaneous inversion | ❌ | sf_compute | Elastic impedance |
| Rock physics templates | ❌ | sf_app | Crossplot tools |
| 4D time-lapse | ❌ | sf_render | 4D difference viz |
| Fault seal analysis | ❌ | sf_compute | SGR, CSP calculations |
| Geosteering module | ❌ | sf_app | Real-time well navigation |

**Deliverables:**
- ✅ Complete QI workflow
- ✅ Inversion & rock physics
- ✅ 4D monitoring capabilities
- ✅ Fault analysis suite

---

### Phase 3 - Enterprise (v2.0)
**Timeline:** 8-10 months | **Priority:** MEDIUM

| Feature | Status | Crate | Notes |
|---------|--------|-------|-------|
| Processing & Imaging | ❌ | sf_compute | FWI lite, depth migration |
| Reservoir modeling | ❌ | sf_core | Grid-based reservoir |
| Petrel connector | ❌ | sf_io | Bidirectional .pet format |
| Cloud collaboration | ❌ | sf_server | Multi-user sessions |
| HPC cluster support | ❌ | sf_compute | Distributed computing |
| Enterprise SSO | ❌ | sf_server | SAML, OIDC |

**Deliverables:**
- ✅ Processing module (entry-level)
- ✅ Reservoir modeling tools
- ✅ Petrel interoperability
- ✅ Cloud collaboration platform

---

### Phase 4 - Killer Features (v2.x)
**Timeline:** 12+ months | **Priority:** FUTURE

| Feature | Status | Crate | Notes |
|---------|--------|-------|-------|
| AI Co-Pilot | ❌ | sf_ml | Natural language commands |
| VR mode | ❌ | sf_render | VR headset support |
| Real-time HPC | ❌ | sf_compute | Live processing |
| Autonomous interpretation | ❌ | sf_ml | AI-driven horizon picking |

**Deliverables:**
- ✅ AI assistant ("pick horizon A at inline 500")
- ✅ VR visualization mode
- ✅ Real-time processing feedback

---

## 4. Business & Community Model (Open-Core)

### Licensing Tiers

| Tier | Price | Features | Target |
|------|-------|----------|--------|
| **OSS (Free)** | $0 | Core interpretation, basic I/O, synthetic data | Students, academics, small companies |
| **Pro** | $1,999-2,999/user/year | ML auto-tracking, advanced attributes, Petrel connector | Indie & mid-size companies |
| **Enterprise** | Custom | Cloud hosting, multi-user, SSO, custom plugins | Major oil companies, service companies |
| **Academic** | Free forever | All Pro features | Universities, research institutions |

### Revenue Streams

1. **Pro Licenses** - Annual subscriptions
2. **Enterprise Contracts** - Custom deployments
3. **Training & Certification** - Official courses
4. **Consulting** - Custom plugin development
5. **Plugin Marketplace** - Revenue share (20%)

### Community Building

- **GitHub Discussions** - Q&A, feature requests
- **Discord/Slack** - Real-time community chat
- **YouTube Channel** - Tutorial videos, webinars
- **Annual Conference** - StrataForge User Meeting
- **University Program** - Free licenses + curriculum

---

## 5. Action Plan

### Week 1 (Immediate)

- [x] Create `blueprint-v1` branch
- [ ] Commit this blueprint as `docs/blueprint.md`
- [ ] Update `README.md` with new vision + badge "Seeking contributors"
- [ ] Create GitHub Project board with 4 phases
- [ ] Post announcement on LinkedIn, X, Reddit (r/geophysics, IndoGeophys)

### Weeks 2-3 (Phase 0 Sprint)

- [ ] Complete SEG-Y reader/writer in `sf_io`
  - Use `segy-rs` crate or enhance existing implementation
  - Support textual headers (EBCDIC)
  - Support binary headers
  - Support trace-level metadata
- [ ] Complete LAS 2.0/3.0 support
  - Add LAS 3.0 parser
  - Well-seismic tie module
  - Formation tops mapping
- [ ] Update documentation
  - User guide for well workflow
  - API documentation

### Month 1 (Phase 0 Completion)

- [ ] PyO3 Python bindings scaffold
  - `pip install stratforge-py`
  - Basic API: load seismic, load well, create surface
- [ ] 10 basic seismic attributes in `sf_compute`
  - RMS amplitude, mean amplitude, max amplitude
  - Instantaneous amplitude, phase, frequency
  - Sweetness, envelope, zero-crossing rate
- [ ] Demo video (3 minutes)
  - Seismic 3D visualization
  - Horizon picking real-time
  - Well-seismic tie

### Months 2-4 (Phase 1 Start)

- [ ] ML auto-tracking development
  - Dataset preparation (synthetic + public data)
  - CNN model training (PyTorch → ONNX)
  - Rust inference (candle/ONNX Runtime)
- [ ] Plugin system implementation
  - Rust plugin API
  - Python plugin API (PyO3)
  - Plugin marketplace scaffold
- [ ] Advanced attributes (100+)
  - Geometric attributes (dip, azimuth, curvature)
  - Spectral decomposition
  - Coherence, chaos

### Months 5-10 (Phase 2-3)

- [ ] QI & inversion module
- [ ] Reservoir modeling
- [ ] Cloud server development
- [ ] Petrel connector (reverse engineering)

---

## 6. Marketing Strategy

### Content Calendar

| Month | Content | Channel |
|-------|---------|---------|
| Month 1 | "StrataForge Pro Announcement" | LinkedIn, Twitter, Reddit |
| Month 2 | "Well-Seismic Tie Tutorial" | YouTube, Blog |
| Month 3 | "ML Auto-Tracking Demo" | YouTube, LinkedIn |
| Month 4 | "Plugin Development Guide" | Blog, GitHub |
| Month 6 | "StrataForge vs Petrel Benchmark" | LinkedIn, Twitter |
| Month 12 | "Year 1 Review + Roadmap 2028" | All channels |

### Key Messages

- **Performance:** "10x faster than Petrel, 5x lighter"
- **Open:** "No license server, no dongle, no BS"
- **Modern:** "Built for cloud, AI, and collaboration"
- **Community:** "By geoscientists, for geoscientists"

### Metrics to Track

- GitHub stars (target: 1k in Year 1)
- Downloads (target: 10k in Year 1)
- Pro licenses sold (target: 50 in Year 1)
- Community contributions (PRs, plugins)

---

## 7. Success Criteria

### Year 1 (2026)
- [ ] 1,000 GitHub stars
- [ ] 10,000 downloads
- [ ] 50 Pro licenses sold
- [ ] 10 community plugins
- [ ] 5 university partnerships

### Year 2 (2027)
- [ ] 5,000 GitHub stars
- [ ] 50,000 downloads
- [ ] 200 Pro licenses sold
- [ ] 50 community plugins
- [ ] First enterprise customer

### Year 3 (2028)
- [ ] 10,000+ GitHub stars
- [ ] 100,000+ downloads
- [ ] 500+ Pro licenses sold
- [ ] Sustainable revenue ($500k+ ARR)
- [ ] Recognized as "Petrel alternative" in industry

---

## Appendix A: Competitor Analysis

### Petrel (Schlumberger)
**Strengths:**
- Industry standard
- Comprehensive workflow
- Strong support

**Weaknesses:**
- Expensive ($10k+/user/year)
- Heavy (.NET, slow startup)
- Closed ecosystem
- License server required

### DUG Insight
**Strengths:**
- All-in-one (processing + interpretation)
- Good performance
- Academic discounts

**Weaknesses:**
- Still expensive
- Limited plugin ecosystem
- Cloud features immature

### OpendTect (dGB Earth Sciences)
**Strengths:**
- Open-core model (proven)
- Python plugin system
- Free version available

**Weaknesses:**
- Aging codebase (C++)
- UI feels dated
- Performance limitations

---

## Appendix B: Technical Debt to Address

### Current Issues (v0.1.1)
- [ ] Incomplete SEG-Y implementation
- [ ] LAS 3.0 not supported
- [ ] Limited test coverage (<50%)
- [ ] Documentation gaps
- [ ] No Python bindings

### Prevention Strategies
- TDD for all new features
- Automated benchmark tests
- Documentation-driven development
- Community review process

---

## Appendix C: Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Legal (patent infringement) | High | Low | Open-source prior art, legal review |
| Technical (Rust ecosystem immaturity) | Medium | Low | Use proven crates, contribute back |
| Market (competitor response) | Medium | Medium | Focus on underserved segments |
| Community (lack of contributors) | High | Medium | Invest in community building early |
| Financial (runway) | High | Medium | Bootstrap + grants + early customers |

---

**Document Version:** 1.0  
**Created:** 2026-03-29  
**Last Updated:** 2026-03-29  
**Owner:** @ajamj  
**Status:** Draft → Review → Approved
