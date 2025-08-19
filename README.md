# abuild

> a tool for building

## Language Support

1. **Rust**
2. **C++ Modules**
3. **C++**
4. **C**
5. **C#**

> [!NOTE]
>
> Sort by development priority

## Usage

```shell
$ abuild new -w my-worksapce
...
$ cd my-workspace
$ abuild new -m my-module
...
$ vi ./my-module/main.rs # edit your code
$ abuild build
...
$ ./target/debug/my-module
Hello, world!
$ abuild clean
...
$ 
```

## Config

### Module

```rust
// abuild.rs
use ::abuild::{module::Module, profile::Profiles, target::Targets};
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Module::default().parse_args().run()?;
  Module::builder()
    .src_dir("./src")
    .src_filter("*.rs")
    .rc_dir("./rc")
    .profiles(
      // Profiles::default()?
      Profiles::builder()
        .dev("debug")
        .release("release")
        .build()?
    )
    .targets(
      // Targets::host()?
      Targets::builder()
        .host()
        .build()?
    )
    .build()?
    .parse_args()
    .run()?;
  Ok(())
}
```

### Workspace

```rust
// abuild.rs
// use ::abuild::prelude::*;
use ::abuild::{workspace::Workspace, module::Modules};
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Workspace::default().parse_args().run()?;
  Workspace::builder()
    .modules(
      Modules::with_dirs([
        "./module_a",
        "./module_b"
      ])
        .build()?
    )
    .build()?
    .parse_args()
    .run()?;
  Ok(())
}
```

### Workspace & Module

```rust
// abuild.rs
// use ::abuild::prelude::*;
use ::abuild::workspace::Workspace;
mod module_a;
mod module_b;
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Workspace::with_projects([project_a::project()?, project_b::project()?]).parse_args().run()?;
  Workspace::builder()
    .modules(
      // ::abuild::marcos::modules![module_a, module_b]
      Modules::from_slice([
        module_a::module()?,
        module_b::module()?,
      ])
    )
    .build()?
    .parse_args()
    .run()?;
  Ok(())
}
```

```rust
// module_a.abuild.rs
use ::abuild::{module::Module, marcos::module};
pub fn module() -> Result<Module, Box<dyn std::error::Error>> {
  todo!()
}
```

### Targets

```rust
// use ::abuild::prelude::*;
use ::abuild::target::Targets;
fn targets() -> Result<Targets, Box<dyn std::error::Error>> {
  Ok(
    // Targets::host()?
    Targets::builder()
      .host()
      .build()?
  )
}
```

### Profiles

```rust
// use ::abuild::prelude::*;
use ::abuild::profile::Profiles;
fn profiles() -> Result<Profiles, Box<dyn std::error::Error>> {
  Ok(
    // Profiles::default()?
    Profiles::builder()
      .dev("debug")
      .release("release")
      .target_dir("./target")
      .build_dir("./target/build")
      .deps_dir("./target/deps")
      .bin_dir("./target/bin")
      .build()?
  )
}
```

### Hooks

```rust
use ::abuild::{module::Module, hook::Hook};
fn hook() -> Result<Module, Box<dyn std::error::Error>> {
  Ok(Module::builder()
    .hook_build(|info| println!("[HOOK] build info: {info:?}"))
    .build()?)
}
```
