# StrataForge UI Redesign Specification

## Overview
Modern hybrid UI design combining ribbon-style efficiency with minimalist aesthetics.

## Design Principles
1. **Workflow-First** - Tools grouped by task (Explore, Interpret, Analyze)
2. **Progressive Disclosure** - Simple for beginners, powerful for experts
3. **Visual Hierarchy** - Clear distinction between primary/secondary actions
4. **Consistent Spacing** - 8px grid system throughout
5. **Context-Aware** - UI adapts to current task

## Layout Structure

### Top Ribbon (60px height)
```
┌─────────────────────────────────────────────────────────────┐
│  StrataForge  [Quick Access]    File  Edit  View  Tools  ?  │
├─────────────────────────────────────────────────────────────┤
│  [New] [Open] [Save] │ [Pick] [Track] [Sketch] │ [Depth]    │
└─────────────────────────────────────────────────────────────┘
```

**Sections:**
- **Menu Bar:** File, Edit, View, Tools, Help
- **Quick Access:** Save, Undo, Redo (always visible)
- **Context Ribbon:** Changes based on active tool

### Left Panel - Project Tree (250px width, collapsible)
```
┌──────────────────────┐
│ ▼ Project Data       │
│                      │
│ 📊 Seismic           │
│   ├─ Full Stack [✓]  │
│   └─ RGB Blend       │
│                      │
│ 🌈 Horizons (3)      │
│   ├─ Horizon A [✓]   │
│   ├─ Horizon B [✓]   │
│   └─ Horizon C [ ]   │
│                      │
│ ⚡ Faults (2)        │
│   ├─ Fault 1 [✓]     │
│   └─ Fault 2 [✓]     │
│                      │
│ 🛢 Wells (1)         │
│   └─ Demo Well [✓]   │
│                      │
│ [+ Add] [Import]     │
└──────────────────────┘
```

### Right Panel - Properties (300px width, collapsible)
```
┌──────────────────────┐
│ Properties           │
│                      │
│ ▼ Horizon A          │
│   Name: [_______]    │
│   Color: [████] ▼    │
│   Visible: [✓]       │
│   Picks: 15          │
│                      │
│   [Export XYZ]       │
│   [Export JSON]      │
│   [Delete]           │
│                      │
│ ───────────────────  │
│                      │
│ ▼ Velocity Model     │
│   Type: [Gradient▼]  │
│   V0: [2000] m/s     │
│   k: [0.5] 1/s       │
│                      │
│   [✓] Depth Mode     │
│                      │
│ ───────────────────  │
│                      │
│ ▼ Analysis           │
│   Gain: [━━━━━] 1.0  │
│   Clip: [━━━━━] 1.0  │
│   Opacity: [━━] 0.5  │
│   Colormap: [Seismic▼│
└──────────────────────┘
```

### Bottom Panel - Logs (200px height, collapsible)
```
┌──────────────────────────────────────────────────────────────┐
│ Well Logs & Crossplots                            [▲ Close]  │
├──────────────────────────────────────────────────────────────┤
│  Well: [Demo Well ▼]   Log: [GR ▼]   Range: [0-3000] m      │
│                                                              │
│  [Graph area - log curve visualization]                      │
└──────────────────────────────────────────────────────────────┘
```

### Status Bar (28px height)
```
┌──────────────────────────────────────────────────────────────┐
│ X: 250.5  Y: 312.8  Z: 1523m  │  TWT: 1.250s  │  Auto-Tracking: [████░░] 50%  │
└──────────────────────────────────────────────────────────────┘
```

## Color Scheme

### Primary Colors
- **Background:** `#1e1e1e` (dark theme)
- **Panel Background:** `#252525`
- **Hover Background:** `#3e3e42`
- **Selected Background:** `#094771`
- **Border:** `#3e3e42`

### Accent Colors
- **Seismic:** `#4fc3f7` (light blue)
- **Horizon:** `#81c784` (green)
- **Fault:** `#e57373` (red)
- **Well:** `#ffb74d` (orange)
- **Active:** `#64b5f6` (bright blue)

### Text Colors
- **Primary Text:** `#ffffff`
- **Secondary Text:** `#b0b0b0`
- **Disabled Text:** `#6e6e6e`

## Typography
- **Headings:** 14px Semi-Bold
- **Body:** 12px Regular
- **Labels:** 11px Regular
- **Status Bar:** 11px Medium

## Spacing (8px Grid)
- **Panel Padding:** 8px
- **Section Spacing:** 16px
- **Item Spacing:** 4px
- **Button Padding:** 8px 16px

## Component Specifications

### Ribbon Buttons
```rust
// Primary action button
Button::new("📂 Open")
    .min_size(vec2(80.0, 32.0))
    .fill(Color32::from_rgb(0, 120, 215))

// Secondary action button
Button::new("💾 Save")
    .min_size(vec2(40.0, 32.0))
    .stroke(Stroke::new(1.0, Color32::GRAY))
```

### Tree View Items
```rust
// Tree node with icon
ui.horizontal(|ui| {
    let icon = match item_type {
        "Seismic" => "📊",
        "Horizon" => "🌈",
        "Fault" => "⚡",
        "Well" => "🛢",
    };
    ui.label(icon);
    ui.label(&name);
    ui.checkbox(&mut visible, "");
});
```

### Property Controls
```rust
// Labeled input
ui.horizontal(|ui| {
    ui.label("V0 (m/s):");
    ui.add(DragValue::new(&mut v0).speed(10.0));
});

// Color picker with preview
ui.horizontal(|ui| {
    ui.label("Color:");
    let color_rect = ui.allocate_response(vec2(24.0, 24.0), Sense::click());
    painter.rect_filled(color_rect.rect, 4.0, color);
    if color_rect.clicked() {
        // Open color picker dialog
    }
});
```

## Interaction Patterns

### Selection
- **Single Click:** Select item
- **Ctrl+Click:** Add to selection
- **Shift+Click:** Range selection
- **Double Click:** Zoom to item / Open properties

### Drag & Drop
- **Reorder:** Drag items in tree
- **Color Change:** Drag color to item
- **Import:** Drag file to viewport

### Context Menus
- **Right-Click:** Show context menu
- **Common Actions:** Rename, Delete, Export, Properties

## Responsive Behavior

### Panel States
- **Expanded:** Full width/height
- **Collapsed:** Icon-only (50px)
- **Auto-Hide:** Slide out on hover

### Viewport Priority
- Viewport always gets maximum space
- Panels collapse if window < 1024px width
- Bottom panel auto-hides if window < 768px height

## Accessibility
- **Keyboard Navigation:** Tab through all controls
- **Shortcuts:** Ctrl+S (Save), Ctrl+Z (Undo), Ctrl+Y (Redo)
- **High Contrast:** Support for high contrast themes
- **Tooltips:** All icons have descriptive tooltips

## Implementation Priority

### Phase 1: Core Layout (This Session)
1. New panel structure with proper sizing
2. Modern color scheme
3. Improved spacing and typography
4. Status bar with coordinates

### Phase 2: Enhanced Controls (Next Session)
1. Custom tree view with icons
2. Improved property editors
3. Context menus
4. Keyboard shortcuts

### Phase 3: Polish (Future)
1. Animations (smooth collapse/expand)
2. Drag & drop
3. Custom tooltips
4. Theme support (light/dark)
