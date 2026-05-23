# Architecture

> It is recommended to use dark themes

## 1. Overall Architecture

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  STD:::std

  Core:::core

  Tools:::tool

  Functions:::function

  View:::view

  Crates:::crate

  View --> Functions --> Tools & STD & Crates --> Core
```

`Functions` allows `STD` and `Crate` to be used by `Tools` through interfaces within `Core`.

## 2. Core

### 2.1 Profile

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  Profile{Profile}:::interface

  subgraph Language Profiles
    
    CProfile:::core
    CPPProfile:::core
    RustProfile:::core
    GDScriptProfile:::core

  end

  CProfile & CPPProfile & RustProfile & GDScriptProfile --o Profile

```

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  Profile{Profile}:::interface

  subgraph Target Profiles
    
    TargetProfile:::core
    GDExtensionExportProfile:::core

  end

  TargetProfile & GDExtensionExportProfile --o Profile

```

### 2.2 Module

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  subgraph Modules

    ModuleModel:::core
  
    Module{Module}:::interface

    ModuleFiles{ModuleFiles}:::interface
    ModuleChilds{ModuleChilds}:::interface
  
  end

  ModuleFiles --> AsyncRead & AsyncWrite

  ModuleModel --o Module

  Module --o ModuleChilds & ModuleFiles

```

## 3. Tools

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  Core:::core

  subgraph Tools

    Build:::tool
    Clean:::tool
    Run:::tool
    Init:::tool

  end

  Tools --> Core
```

### 3.1 Build

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  Core:::core

  subgraph Build
  
    BuildPipelines[Build Pipelines]:::tool
    Compilers:::tool
    Linkers:::tool
    BuildBackends[Build Backends]:::tool

  end

  Build --> Core
```

#### 3.1.1 Build Pipelines

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  Profile:::core

  subgraph Core
    ModuleFiles{ModuleFiles}:::interface
  end

  CompilerFactory:::tool
  LinkerFactory:::tool

  subgraph Build Pipelines

    BuildPipeline{BuildPipeline}:::interface

    DefaultBuildPipeline:::tool

    BuildPipelineFactory:::tool

  end

  BuildPipelineFactory --> DefaultBuildPipeline ---> CompilerFactory & LinkerFactory

  BuildPipeline -----> Profile & ModuleFiles

  DefaultBuildPipeline --o BuildPipeline
```

#### 3.1.2 Build Backends

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  subgraph Backend

    CargoBackend:::tool
    MakefileBackend:::tool
    NinjaBackend:::tool
    CMakeBackend:::tool

  end
```

#### 3.1.3 Compilers

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}


  subgraph Compilers

    Compiler{Compiler}:::interface

    GCC:::tool
    Clang:::tool
    MSVC:::tool
    Rustc:::tool

    CompilerFactory:::tool
            
  end

  CompilerFactory --> GCC & Clang & MSVC & Rustc --o Compiler
```

#### 3.1.4 Linkers

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  subgraph Linkers

    Linker{Linker}:::interface

    LLVMLinker:::tool

    LinkerFactory:::tool

  end

  LinkerFactory --> LLVMLinker --o Linker
```

### 3.2 Clean

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  subgraph Core
    ModuleFiles{ModuleFiles}:::interface
  end

  subgraph Clean

    ModuleCleaner{ModuleCleaner}:::interface

    DefaultCleaner:::tool

    ModuleCleanerFactory:::tool

  end

  ModuleCleanerFactory --> DefaultCleaner --o ModuleCleaner ---> ModuleFiles

```

### 3.3 Run

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  subgraph Core
    ModuleFiles{ModuleFiles}:::interface
  end

  subgraph Run

    ModuleRunner{ModuleRunner}:::interface

    DefaultRunner:::tool

    ModuleRunnerFactory:::tool

  end

  ModuleRunnerFactory --> DefaultRunner --o ModuleRunner ---> ModuleFiles

```

### 3.4 Init

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  subgraph Core
    Module{Module}:::interface
  end

  subgraph Init

    ModuleInit{ModuleInit}:::interface

    DefaultInit:::tool
    GDExtensionInit:::tool

    ModuleInitFactory:::tool

  end

  ModuleInitFactory --> DefaultInit & GDExtensionInit --o ModuleInit --> Module
```

## 4. Functions

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  ModuleInitFactory:::tool
  ModuleRunnerFactory:::tool
  ModuleCleanerFactory:::tool
  BuildPipelineFactory:::tool

  subgraph Functions

    ModuleManager:::function
 
  end

  ModuleManager ---> ModuleInitFactory & BuildPipelineFactory & ModuleCleanerFactory & ModuleRunnerFactory
```

## 5. Views

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  ModuleManager:::function

  subgraph Views

    Presenter:::view

    CLI:::view

    ConsoleView:::view

    VSCodeExtension:::view

  end

  CLI & Presenter --> ModuleManager

  ConsoleView & VSCodeExtension --> Presenter
```
