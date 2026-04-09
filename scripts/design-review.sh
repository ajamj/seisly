#!/bin/bash
# Design Review Gate - 5 Parallel Agents
# Usage: Run from project root with design doc path
# Example: bash scripts/design-review.sh docs/plans/opentect-parity-roadmap-design.md

DESIGN_DOC="${1:?Usage: $0 <path-to-design-doc.md>}"

echo "## Design Review Gate"
echo "Design: $DESIGN_DOC"
echo "Running 5 parallel reviews..."
echo ""

# Read agent definitions
PM_AGENT=$(cat agents/product-manager.md)
ARCH_AGENT=$(cat agents/architect.md)
DESIGNER_AGENT=$(cat agents/designer.md)
SECURITY_AGENT=$(cat agents/security-design.md)
CTO_AGENT=$(cat agents/cto.md)

echo "Agents loaded:"
echo "  1. Product Manager (agents/product-manager.md)"
echo "  2. Architect (agents/architect.md)"
echo "  3. Designer (agents/designer.md)"
echo "  4. Security Design (agents/security-design.md)"
echo "  5. CTO (agents/cto.md)"
echo ""
echo "To run in Qwen Code, invoke 5 parallel agent calls with:"
echo "  - subagent_type: general-purpose"
echo "  - prompt: \"<agent definition> + design doc: $DESIGN_DOC\""
echo ""
echo "Results will appear in the conversation when all agents complete."
