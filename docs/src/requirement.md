# abuild - 通用模块化构建工具需求文档

## 一、设计目标 (Design Goals)

设计一款可拓展、符合人体工程学且具备高度通用性的现代构建工具，旨在统一多语言、多平台的开发与构建体验。

## 二、模块管理 (Module Management)

### 核心特性

- **模块**：支持按模块（Module-by-module）进行独立编译与依赖管理
- **子模块支持**：支持嵌套子模块（Submodules），允许构建复杂的层级项目结构
- **依赖解析**：自动解析模块间的依赖关系，构建最优编译拓扑图

## 三、人体工程学与工作流 (Ergonomics & Workflows)

### 统一命令接口

- 提供简洁的CLI命令以选择脚本或配置
- 统一功能拓展，一份代码多视图通用
  - 拓展接口
    - 标识符 (id)：命令的唯一标识（如 `build`, `deploy-aws`）
    - 作者 (author)
    - 描述 (destination)
      - 短描述 (destination)：用于列表展示（CLI 的 help 列表或 IDE 的 Command Palette）
      - 长描述 (destination)：用于详细文档或鼠标悬停提示（Tooltip）
    - 参数 (params)：依赖反射
      - 类型安全：自动识别参数类型（String, Int, bool, Enum, Path）
      - 元数据：包括参数名、别名（Short flag）、默认值、是否必填、参数校验规则（如正则匹配）
      - 分组：支持将参数分组
    - 回调函数 (fn)
      - 执行逻辑：命令触发的核心业务逻辑
      - 上下文注入 (Context Injection)：运行时自动注入当前项目路径、环境变量、日志句柄等上下文信息
      - 生命周期钩子：支持 pre-run（执行前检查）和 post-run（执行后清理/通知）

### 配置

#### 多种配置方式

+ `TOML`：`config.toml`
+ `JSON`：`config.json`, `config.schema.json`
+ `cargo-script`：`config.rs`

```toml
# abuild.toml
[module]
name = "my_module"
author = "author <example.com>"
version = "0.1.0"
edition = "2026" # 指定构建工具的“语言”版本

[module.lib]
types = ["lib", "dylib", "clib", "cdylib"]

[[submodule]]
name = "submodule1"
path = "modules/submodule1"

[dependencies]
core_lib = { path = "../core", version = "1.0" }
# 支持 Git 依赖
utils = { git = "https://github.com/example/utils.git", branch = "main" }

[profile.rust]
file = "rust.toml"

[profile.dev.rust]
file = "dev.rust.toml"

```

### 全生命周期支持

- `new`：创建项目
- `build`：构建项目（支持Debug/Release模式）
- `run`：编译并直接运行目标程序
- `debug`：生成调试符号并启动调试器
- `deploy`：打包并部署到目标环境
  - Github：推送到 github.com
- `test`：运行单元测试与集成测试
- `clean`：清理构建缓存与产物
  - `-r`/`--recursive`
    - 递归清理。不仅清理当前模块的构建产物，还会级联清理其所有依赖的子模块（Submodules）的缓存与产物
  - `--module <name>`
    - 指定模块。仅清理指定名称的特定模块
  - `--profile <profile>`
    - 指定配置。仅清理特定构建配置

## 四、高性能热重载与编译 (High-Performance Hot Reloading)

### 核心功能

- **模块级热重载 (Module-level Hot Reload)**：仅重新编译发生变动的模块及其依赖
- **文件级热重载 (File-level Hot Reload)**：监听单文件变动，实现代码更新与重载
- **自定义热更新策略 (Custom Hot Update Policies)**：允许用户配置特定文件或目录的监听规则及重载触发逻辑
- **增量编译缓存**：智能缓存中间产物，避免重复编译未修改的代码

## 五、通用性 (Universality)

### 国际化 (I18n)

- 支持多语言界面切换
- 允许用户自定义本地化文本与错误提示信息
  - 配置文件：`~/.config/abuild/languages/en_US.toml`

### 多语言支持 (Languages)

- **原生支持**：Rust, C, C++, Zig
- **扩展支持**：预留接口以支持未来其他系统级编程语言

### 跨平台支持 (Platforms)

- Linux (GNU/Musl)
  - Arch Linux
  - Ubuntu
  - Android
- Windows (MSVC/MinGW)
- macOS (Apple Silicon)
- Unix-like

## 六、视图与交互接口 (Views & Interfaces)

### API 层 (abuild-lib)

提供核心构建逻辑的编程接口，允许第三方工具集成abuild的能力

```rust
use std::path::Path;

use super::{prelude::v1::*, *};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[tokio::test]
async fn example_1()
{
  use crate::langs::RustModuleExt;
  let module = module::Module::binary()
    .identifier("example_1")
    .name("example 1")
    .description("an example for test abuild")
    .version("0.0.1")
    .dir(Path::new(MANIFEST_DIR).join("tests/example_1"))
    .source_dir("src")
    .output_dir("output")
    .target_dir("output/target")
    .cache_dir("output/cache")
    .main("main.rs")
    .rust_edition(langs::rust::Edition::_2024)
    .rustc(langs::rust::ComplierSelector::Host)
    .cargo(langs::rust::CargoSelector::Host);

  // (prelude) build::ModuleHostBuilderExt
  let builder = module.host_builder().unwarp();
  builder.build().await.unwarp();

  // (prelude) run::ModuleHostRunnerExt
  let runner = module.host_runner().unwarp();
  runner.run().await.unwarp(); // run ${target_dir}/host/example_1
}
```

### 命令行界面 (abuild-cli)

- 提供标准的控制台交互视图
- 支持彩色日志、进度条及交互式配置向导

### IDE 扩展生态

- **VS Code 扩展 (abuild/vscode-extension)**：提供任务集成、代码补全及调试支持
- **JetBrains IDE 扩展 (abuild/idea-extension)**：适配IntelliJ IDEA, CLion, RustRover等，实现无缝构建集成

## 七、可拓展（Extensibility）

### 外部生态

+ [IDE 拓展生态](#ide-扩展生态)

### 插件

+ [多语言支持](#多语言支持-languages)
+ [统一命令接口](#统一命令接口)
+ 平台特色功能支持

