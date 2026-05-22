# Locale Config

## Language configuration

+ ID:`locale.language`
+ Format:`language[_territory][.codeset][@modifier]`
+ TOML example:`core.language = "zh_CN.UTF-8"`

### Example

```toml
[locale]
base = "zh_CN.UTF-8"
```

After [Automatic Completion](../configs.md#automatic-completion),

```toml
[locale]
base = "zh_CN.UTF-8"
language = "zh"
territory = "CN"
codeset = "UTF-8"
modifier = "pinyin"
```
