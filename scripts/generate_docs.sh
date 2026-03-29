#!/bin/bash
# Auto-generate documentation from code
# Run this script to update documentation automatically

set -e

echo "🚀 Generating documentation from code..."

# Generate Rust API documentation
echo "📚 Generating Rust API docs..."
cargo doc --workspace --no-deps --output-dir docs/api

# Generate markdown summary
echo "📝 Generating feature summary..."

cat > docs/AUTO_GENERATED_FEATURES.md << 'EOF'
# Auto-Generated Features Documentation

This document is automatically generated from code.
Last updated: $(date)

## Available Crates

EOF

# List all crates
for crate_dir in crates/sf_*; do
    crate_name=$(basename "$crate_dir")
    echo "### $crate_name" >> docs/AUTO_GENERATED_FEATURES.md
    echo "" >> docs/AUTO_GENERATED_FEATURES.md
    
    # Extract description from Cargo.toml
    if grep -q "description" "$crate_dir/Cargo.toml"; then
        desc=$(grep "description" "$crate_dir/Cargo.toml" | cut -d'"' -f2)
        echo "$desc" >> docs/AUTO_GENERATED_FEATURES.md
    fi
    echo "" >> docs/AUTO_GENERATED_FEATURES_FEATURES.md
done

echo "✅ Documentation generated successfully!"
echo "📁 Output: docs/api/"
echo "📄 Summary: docs/AUTO_GENERATED_FEATURES.md"
