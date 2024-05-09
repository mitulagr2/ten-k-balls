# ten-k-balls

## Summary

Physics simulation with 10K bouncing balls in your browser.

## Solution

This uses Rust and WebAssembly for a high-performance browser-based simulation. It integrates the Bevy engine and Rapier physics library to realistically simulate 10,000 balls colliding and bouncing in a box.

### Features

1. **Efficient Physics Simulation:** The core feature must be the efficient handling and smooth performance.

2. **WebAssembly Optimization:** Optimizing the Rust code to run efficiently as WebAssembly.

3. **Responsiveness and Scalability:** The simulation scales correctly across different devices and resolutions, offering a consistent experience.

4. **Interactive Controls:** Controls to change the gravity, apply forces to the balls, or alter conditions like the boundary walls' elasticity.

5. **Dynamic User Interface:** Enables users to easily understand and manipulate the simulation settings.

6. **Real-Time Performance Monitoring:** Display key performance metrics such as FPS and simulation step time on the screen.

7. **Shareable Setups:** Enable users to create, save, and share their own unique simulation settings with others.

8. **Visual Effects:** Enhanced visual effects options, like slow motion and color shifts based on velocity or collision intensity.

## Phases

### Phase 0 / V0 / Alpha

Run simulation within browser.

### Phase 1 / V1 / Beta (Optional)

Responsive website and interactive controls.

### Phase 2 / V2 / Release Candidate (Optional)

Real-time performance monitoring.

## Implementation / Technical Details

- Tech stack:

    - Rust
    - Bevy with rapier2d
    - WebAssembly
