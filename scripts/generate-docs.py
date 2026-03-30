#!/usr/bin/env python3
"""
Seisly Documentation Generator
Automatically generates HTML documentation from markdown files
"""

import os
import re
from pathlib import Path
from datetime import datetime

# Configuration
REPO_ROOT = Path(__file__).parent.parent
DOCS_DIR = REPO_ROOT / "docs"
OUTPUT_DIR = REPO_ROOT  # gh-pages root
ASSETS_DIR = OUTPUT_DIR / "assets"

def convert_markdown_to_html(md_content, title="Seisly Documentation"):
    """Simple markdown to HTML converter"""
    html = md_content
    
    # Headers
    html = re.sub(r'^# (.+)$', r'<h1>\1</h1>', html, flags=re.MULTILINE)
    html = re.sub(r'^## (.+)$', r'<h2>\1</h2>', html, flags=re.MULTILINE)
    html = re.sub(r'^### (.+)$', r'<h3>\1</h3>', html, flags=re.MULTILINE)
    
    # Bold and italic
    html = re.sub(r'\*\*(.+?)\*\*', r'<strong>\1</strong>', html)
    html = re.sub(r'\*(.+?)\*', r'<em>\1</em>', html)
    
    # Code blocks
    html = re.sub(r'```(\w+)?\n(.*?)```', r'<pre><code class="language-\1">\2</code></pre>', html, flags=re.DOTALL)
    html = re.sub(r'`([^`]+)`', r'<code>\1</code>', html)
    
    # Links
    html = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'<a href="\2">\1</a>', html)
    
    # Images
    html = re.sub(r'!\[([^\]]*)\]\(([^)]+)\)', r'<img src="\2" alt="\1">', html)
    
    # Lists
    html = re.sub(r'^- (.+)$', r'<li>\1</li>', html, flags=re.MULTILINE)
    html = re.sub(r'(<li>.+</li>\n)+', r'<ul>\n\g<0></ul>\n', html)
    
    # Tables (simple conversion)
    html = re.sub(r'^\|(.+)\|$', r'<table>\n<tr>\1</tr>', html, flags=re.MULTILINE)
    
    # Line breaks
    html = html.replace('\n\n', '</p><p>')
    
    return html

def generate_index_html():
    """Generate main index.html from template"""
    
    # Get all markdown files
    md_files = list(DOCS_DIR.glob("*.md")) if DOCS_DIR.exists() else []
    
    # Count crates from api directory
    api_dir = OUTPUT_DIR / "api"
    crate_count = len(list(api_dir.glob("seisly_*"))) if api_dir.exists() else 18
    
    html_content = f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Seisly - Rust-Powered Seismic Studio</title>
    <link rel="stylesheet" href="/assets/css/style.css">
    <meta name="generator" content="Seisly Docs Generator">
    <meta name="generated" content="{datetime.now().isoformat()}">
</head>
<body>
    <nav class="navbar">
        <div class="container">
            <div class="nav-brand">
                <svg class="logo" viewBox="0 0 40 40" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 20 L15 10 L25 30 L35 15" stroke="#3B82F6" stroke-width="3" fill="none"/>
                    <circle cx="5" cy="20" r="3" fill="#3B82F6"/>
                    <circle cx="15" cy="10" r="3" fill="#3B82F6"/>
                    <circle cx="25" cy="30" r="3" fill="#3B82F6"/>
                    <circle cx="35" cy="15" r="3" fill="#3B82F6"/>
                </svg>
                <span class="brand-name">Seisly</span>
            </div>
            <div class="nav-links">
                <a href="/">Home</a>
                <a href="/QUICKSTART.html">Quick Start</a>
                <a href="/api/README.html">API Docs</a>
                <a href="https://github.com/ajamj/seisly" target="_blank">GitHub</a>
            </div>
        </div>
    </nav>

    <main class="hero">
        <div class="container">
            <div class="hero-content">
                <h1>Seisly</h1>
                <p class="tagline">Rust-Powered Seismic Studio</p>
                <p class="description">
                    The fastest, most accessible seismic interpretation platform - 
                    from exploration to production.
                </p>
                <div class="hero-buttons">
                    <a href="/QUICKSTART.html" class="btn btn-primary">Get Started</a>
                    <a href="/api/README.html" class="btn btn-secondary">API Documentation</a>
                </div>
            </div>
            
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value">&lt;2s</div>
                    <div class="stat-label">Startup Time</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">&lt;100MB</div>
                    <div class="stat-label">Install Size</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">{crate_count}</div>
                    <div class="stat-label">Crates</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">MIT</div>
                    <div class="stat-label">Open Source</div>
                </div>
            </div>
        </div>
    </main>

    <section class="features">
        <div class="container">
            <h2>Features</h2>
            <div class="features-grid">
                <div class="feature-card">
                    <div class="feature-icon">🎨</div>
                    <h3>Seismic Interpretation</h3>
                    <p>Interactive horizon picking, fault modeling, and auto-tracking with ML</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">⚡</div>
                    <h3>GPU Acceleration</h3>
                    <p>wgpu-based rendering and compute shaders for 10x speedup</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">🤖</div>
                    <h3>Machine Learning</h3>
                    <p>CNN-based auto-tracking and U-Net fault detection</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">🔬</div>
                    <h3>Quantitative Interpretation</h3>
                    <p>AVO analysis, FWI, and elastic parameter estimation</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">🌊</div>
                    <h3>4D Monitoring</h3>
                    <p>Time-lapse seismic analysis and CCUS monitoring</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">🔌</div>
                    <h3>Plugin System</h3>
                    <p>Extensible with Rust and Python plugins</p>
                </div>
            </div>
        </div>
    </section>

    <section class="docs-list">
        <div class="container">
            <h2>Documentation</h2>
            <div class="docs-grid">
'''
    
    # Add documentation files dynamically
    doc_categories = {
        "📦 Guides": ["QUICKSTART.md", "blueprint.md", "architecture.md"],
        "📊 Features": ["PHASE_1_FEATURES.md", "PHASE_2_FEATURES.md", "GPU_ACCELERATION.md"],
        "🔬 Advanced": ["QI_GUIDE.md", "4D_MONITORING.md", "ml_auto_tracking.md"],
        "🛠️ Development": ["plugin_development.md", "seismic_attributes.md"]
    }
    
    for category, files in doc_categories.items():
        html_content += f'''                <div class="doc-category">
                    <h3>{category}</h3>
                    <ul>
'''
        for file in files:
            md_path = DOCS_DIR / file
            if md_path.exists() or (OUTPUT_DIR / file.replace('.md', '.html')).exists():
                name = file.replace('.md', '').replace('_', ' ').title()
                html_name = file.replace('.md', '.html')
                html_content += f'''                        <li><a href="/{html_name}">{name}</a></li>
'''
        
        html_content += '''                    </ul>
                </div>
'''
    
    html_content += '''            </div>
        </div>
    </section>

    <footer class="footer">
        <div class="container">
            <div class="footer-content">
                <div class="footer-section">
                    <h4>Seisly</h4>
                    <p>Rust-Powered Seismic Studio</p>
                </div>
                <div class="footer-section">
                    <h4>Links</h4>
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/QUICKSTART.html">Quick Start</a></li>
                        <li><a href="/api/README.html">API Docs</a></li>
                    </ul>
                </div>
                <div class="footer-section">
                    <h4>Community</h4>
                    <ul>
                        <li><a href="https://github.com/ajamj/seisly" target="_blank">GitHub</a></li>
                        <li><a href="https://github.com/ajamj/seisly/issues" target="_blank">Issues</a></li>
                    </ul>
                </div>
                <div class="footer-section">
                    <h4>License</h4>
                    <p>MIT OR Apache-2.0</p>
                </div>
            </div>
            <div class="footer-bottom">
                <p>&copy; 2026 Seisly Contributors. Built with ❤️ using Rust.</p>
                <p class="generated">Generated: ''' + datetime.now().strftime("%Y-%m-%d %H:%M:%S") + '''</p>
            </div>
        </div>
    </footer>
</body>
</html>
'''
    
    return html_content

def main():
    print("🔧 Seisly Documentation Generator")
    print("=" * 40)
    
    # Ensure output directory exists
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    ASSETS_DIR.mkdir(parents=True, exist_ok=True)
    
    # Generate index.html
    print("📄 Generating index.html...")
    index_html = generate_index_html()
    index_path = OUTPUT_DIR / "index.html"
    index_path.write_text(index_html, encoding='utf-8')
    print(f"✅ Generated: {index_path}")
    
    # Count files
    md_count = len(list(DOCS_DIR.glob("*.md"))) if DOCS_DIR.exists() else 0
    print(f"\n📊 Statistics:")
    print(f"   Markdown files: {md_count}")
    print(f"   Output: {index_path}")
    
    print("\n✨ Documentation generated successfully!")
    print("🌐 View at: https://ajamj.github.io/seisly/")

if __name__ == "__main__":
    main()
