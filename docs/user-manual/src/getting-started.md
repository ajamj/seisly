# Getting Started

Follow these steps to quickly start using Seisly.

## 1. Launch the Application

Run the application from your launcher or via command line:

```bash
cargo run --release --bin seisly-app
```

## 2. Create or Open a Project

1. Go to **File → New Project...** to create a new Seisly project.
2. Specify the project name and the coordinate reference system (CRS).

## 3. Import Data

### Seismic Data

1. In the Project Explorer, right-click on **Seismic** and select **Import SEG-Y...**.
2. Browse to your `.sgy` or `.segy` file and follow the import wizard to define header locations.

### Well Data

1. Right-click on **Wells** and select **Import LAS...**.
2. Select your LAS file and define the column mappings for Depth, Gamma Ray, Sonic, and Density.

## 4. Basic Visualization

1. Double-click on an imported seismic volume in the Project Explorer.
2. Use the **Inline**, **Crossline**, and **Time Slice** sliders in the viewing panel to navigate the volume.

## 5. Horizon Interpretation

1. Select **Interpret** from the main toolbar.
2. Choose a picking mode: **Seed**, **Manual**, or **Auto**.
3. Click on the seismic view to add picks.
4. Go to **Surfaces → Generate Surface** to model a horizon surface from your picks.

## 6. Fault Sketching

1. Select **Sketch Fault** from the toolbar.
2. Click and drag in the seismic viewer to draw fault sticks.
3. Seisly will automatically model the fault surface from the sticks using RBF interpolation.

## 7. Next Steps

- Explore more advanced features like **Seismic Attributes**, **ML Auto-Tracking**, and **4D Monitoring** in the subsequent chapters of this manual.
- Learn about building custom extensions in the [Plugin Development](../development/plugin-development.md) guide.
