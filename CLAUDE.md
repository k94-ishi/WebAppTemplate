# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Setup
```bash
npm install
```

### Development
```bash
npm run dev          # Start development server with hot-reload
npm run preview      # Preview production build
```

### Building
```bash
npm run build        # Build WASM and Vue app for production
npm run build-only   # Build Vue app only (without WASM)
npm run wasm-build   # Build WASM module only
```

### Code Quality
```bash
npm run type-check   # Type checking with vue-tsc
npm run lint         # Run all linters (oxlint + eslint)
npm run format       # Format code with prettier
```

## Architecture Overview

This is a Vue 3 + TypeScript blog application with WebAssembly physics simulations. The project combines a traditional blog structure with interactive WASM-powered physics demos.

### Key Technologies
- **Frontend**: Vue 3 + TypeScript + Vite + TailwindCSS
- **Physics**: Rust + WebAssembly using rapier2d physics engine
- **Content**: Markdown posts with Vue Router
- **Styling**: TailwindCSS with typography plugin

### Project Structure
- `src/components/`: Vue components including physics simulations
- `src/posts/`: Markdown blog posts
- `src/router/`: Vue Router configuration
- `src/wasm/`: Generated WASM bindings and modules
- `src/lib.rs` + `src/liquid_sim.rs`: Rust physics simulation code

### WASM Integration
The project uses a custom build process that:
1. Compiles Rust code to WASM using `wasm-pack`
2. Generates TypeScript bindings in `src/wasm/`
3. Integrates with Vue components for interactive physics demos

### Physics Simulations
- **PhysicsSim**: Basic rigid body physics with balls and gravity
- **LiquidSim**: Particle-based liquid simulation using many small spheres
- Both simulations use the rapier2d physics engine compiled to WASM

### Content System
- Blog posts are written in Markdown and stored in `src/posts/`
- Posts are dynamically loaded via Vue Router using the slug parameter
- The `vite-plugin-md` enables importing .md files as Vue components

### Routing Structure
- `/`: Home page with post listings
- `/posts/:slug`: Individual blog post pages
- `/calc`: Basic WASM calculator demo
- `/physicssim`: Rigid body physics simulation
- `/liquidsim`: Liquid particle simulation

## Development Notes

- WASM modules are automatically rebuilt during `npm run build`
- Use `npm run wasm-build` for WASM-only rebuilds during development
- Physics simulations require the WASM module to be built before they function
- The build target is set to 'esnext' to support top-level await for WASM loading