---
layout: home
title: Home
nav_order: 1
description: "The Professional open-source seismic interpretation platform, powered by Rust."
---

# Seisly Documentation

**Seisly** is a modern, high-performance subsurface interpretation workstation built for geoscientists who need speed, security, and extensibility.

{: .fs-6 .fw-300 }
[Get Started]({% link docs/getting-started/quickstart.md %}){: .btn .btn-primary .fs-5 .mb-4 .mb-md-0 .mr-2 } [View on GitHub](https://github.com/ajamj/seisly){: .btn .fs-5 .mb-4 .mb-md-0 }

---

## 🚀 Version 1.0.0 is Live!

Seisly has reached production readiness with a completely modernized architecture.

### 🏢 Professional UI/UX
Enjoy an industry-standard dockable panel architecture using `egui_dock`. Customize your workspace with persistent layouts that remember your setup.

### 🛡️ Secure AI Plugins
Run your custom Deep Learning models safely. Seisly uses a process-isolated Python worker model to ensure that even a crashing plugin won't affect your main interpretation session.

### ⚡ Extreme Performance
Handle massive seismic volumes beyond your RAM capacity with our LRU brick caching system. Powered by Rust and WGPU for lightning-fast rendering and computation.

---

## 📚 Explore the Docs

| Section | Description |
|:--- |:--- |
| [**User Manual**](./user-manual/) | Comprehensive guide for end-users (mdBook). |
| [**API Reference**](./api/) | Technical documentation for Seisly crates and modules (RustDoc). |
| [**Getting Started**]({% link docs/getting-started/quickstart.md %}) | Set up Seisly in less than 5 minutes. |
| [**Core Features**]({% link docs/README.md %}) | Deep dives into Horizon Picking, Fault Modeling, and more. |
| [**Architecture**]({% link docs/architecture.md %}) | Technical overview of the crate structure and data flow. |
| [**Plugin Development**]({% link docs/development/plugin-development.md %}) | Learn how to build and integrate your own AI models. |

---

## 🤝 Contributing

We welcome contributions from geoscientists and engineers alike. Check out our [Architecture guide]({% link docs/architecture.md %}) to understand how Seisly is built.

---

**Note:** Seisly was formerly known as *StrataForge*. All core components have been rebranded for the v1.0.0 release.
