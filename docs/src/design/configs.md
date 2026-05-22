# Configs

1. Configure the configuration file location according to the software configuration  
   [Core Config](./configs/core.md): `./configs/core.toml`
2. Prioritize user configuration
3. Next, system configuration

## Automatic Completion

TOML Example:

```toml
base = "base_type"
```

After each `view` runs at a time,

```toml
base = "base_type"
something_a = "..."
something_b = "..."
```

### Software Upgrade and downgrade protection:

+ Upgrade: Auto-complete
+ Downgrade: Prompts for extra parameters
