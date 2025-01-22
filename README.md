## 包、create、模块

- 绝对路径：从 crate 根开始，使用 crate::module::submodule::item;
- 相对路径：从当前模块开始，使用 self::module::item 或 super::module::item;
- 使用use和as：use outer_module::inner_module::inner_function as inner;
- 导入所有公共元素：use outer_module::inner_module::*;

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

##