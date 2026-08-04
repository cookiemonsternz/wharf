# Planning

The core goals behind this engine are to create something which can easily handle stylized 3d rendering.

I'd especially like to have built in support for distance field rendering but also the regular rasterization pipeline. Ease of use of shader configurability is one of my main goals and I'd also like it to be easy to use compute shaders.

## Roadmap
- [x] Entry point
- [ ] Application layer
- [ ] Window layer
  - [ ] Input
  - [ ] Events
- [ ] Renderer
- [ ] Render API abstraction
- [ ] Debugging support
- [ ] Scripting language
- [ ] Memory systems
- [ ] Entity component system (ECS)
- [ ] File IO, VFS
- [ ] Build system

## Window architecture

Example client code
```rust
use wharf::*;

struct Sandbox;
impl Application for Sandbox {
    fn new() -> Self
    where
        Self: Sized,
    {
        Sandbox {}
    }
}

entrypoint!(Sandbox);
```

```rust
entrypoint!(<Application>);
```
Expands to
```rust
crate::init(); // Initialize winit (logging, etc)
let mut app = T::new();
let mut engine = Engine::new(app); // Engine owns app
crate::platform::run(engine); // Platform dependent run code. 
// In this case, with winit: Create window / event loop, 
// bind engine and then events are passed to Engine, 
// handled, then passed to application.

crate::shutdown();
```

```rust
crate::platform::run(app) // passes on to platform impl, by default calls crate::platform::winit::run(app);
```
