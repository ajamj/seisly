import numpy as np

def execute(args):
    """
    Mock horizon tracker entry point.
    In a real plugin, 'args' would contain seismic data pointers.
    """
    print("Mock Horizon Tracker: Generating flat horizon at Z=500...")
    
    # Simulate some computation
    picks = []
    for i in range(0, 500, 50):
        for j in range(0, 500, 50):
            picks.append({
                "x": float(i),
                "y": float(j),
                "z": 500.0,
                "confidence": 1.0
            })
            
    print(f"Mock Horizon Tracker: Generated {len(picks)} picks.")
    return {
        "status": "success",
        "picks": picks
    }
