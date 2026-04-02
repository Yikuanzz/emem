# Mini Grep

## 项目规划

```shell
minigrep/
├── Cargo.toml          # 项目配置清单
├── src/
│   ├── main.rs         # 程序入口 (仅负责调用逻辑)
│   ├── lib.rs          # 核心库入口 (暴露公共API)
│   └── args/           # 参数处理模块 (新建文件夹)
│       └── mod.rs      # 参数模块的实现
└── tests/              # 集成测试目录
    └── integration.rs  # 集成测试文件
```
