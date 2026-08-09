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

---

Wharf is multiplatform (desktop) 3D engine specifically targeting SDF based rendering (while still capable of mesh based 3D rendering). Scripted using rust. Has support for physics via Box3D or Jolt.

## Structure wise

There is no script runtime or anything because the game is just a cargo project with libwharf as a dependency. Custom nodes, etc are effectively just engine extensions.
The editor creates the cargo project with a default entry_point macro which just redirects the entry point to libwharf to handle actually starting stuff.

In terms of the SceneTree:
Custom scripts are used to add behaviour. In order to be used, they must impl Node. Then can be placed in editor like regular nodes. For example, to create a player, one might go:

```rust
#[derive(Node)]
pub struct Player {
    #[child(name = "mesh")] // Finds child node
    mesh: NodeRef<Mesh3D>,

    #[serialize] // Exposes property in editor + serializeable
    speed: Vector3,
}

impl Node for Player {
    fn ready(&self) {
        // Example code!
    }

    fn update(&mut self, delta: f64) {
        // woah more code
        self.mesh.transform.position += speed;
    }
}
```

Then add Player to the scene in editor, with a mesh as a child. It is likely worth it to try to find some way to automatically detect children and stuff such that you don't need to get references to them all the time.

Another core class is resources. As so:

```rust
#[derive(Resource)]
pub struct Fish {
    species: FishType,
    weight: f64
}
```

## Rendering

This engine is built with support for sdf rendering in mind. To this extent, some inbuilt optimisations will be necessary. For more information see [this video](https://www.youtube.com/watch?v=il-TXbn5iMA).

The benefits of an sdf based rendering approach are:

- An extremely editable environment (very cheap additive and destructive operations).
- Smooth blending between shapes :)
- Cheap soft surfaces (volumetrics, etc)
- Very cheap physics (if sdf is already available :O)

The downsides:

- Expensive rendering. With many objects the sdf function gets too expensive to call to the extent which is needed for raymarching. Requires a sample grid which has its downsides
- Resolution issues - when using a grid you're limited by memory so sharp edges get very tricky
- Unconventional. Not toooo much work to reuse.

## Services

Execution order of engine internals functions somewhat similarly to Hazel. Instead of layers (managing both rendering order and execution order), I've opted for _services_. E.g:

- ScriptService
- PhysicsService
- ImGUIService

Services register with a scheduler (part of the core engine class) which then runs execution hooks in order. For example:

```rust
Engine.register_service(&ScriptService);
Engine.register_service(&PhysicsService);

// Order of execution would be:
ScriptService.on_update();
PhysicsService.on_update();

ScriptService.pre_render();
PhysicsService.pre_render();
// etc...
```

In general, services shouldn't directly interact. Instead they should write to shared state. E.g the PhysicsService would just update the SceneTree.

### Service Lifecycle
Services are objects, and so inherit the basic init and deinit methods.
Alongside this, they have a lifecycle with the following hooks:
- update()
- fixed_update()
- pre_render()
- render()
- post_render()

## State

In the engine, data storage is necessary. Think of stuff like:
- SceneTree
- ResourceDB
- AssetDB
- PhysicsWorld
- RenderWorld // Render caches, etc

State handles general purpose data, like the SceneTree, which will be needed across multiple services. 
There isn't a trait so much as a semantic definition. State only stores data, is directly owned by Engine, and is typically acessed by multiple services. PhysicsWorld and RenderWorld therefore would not fall under this case, and instead be owned by their respective services and be referenced if needed by handles.

## Renderer

The renderer is set up as a simple service. Every frame it parses the SceneTree and calls the backend (currently only vulkan) to render.
