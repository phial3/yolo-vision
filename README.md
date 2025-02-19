# rsmpeg-vcpkg-demo 

This crate contains the demo code of [rsmpeg](https://github.com/larksuite/rsmpeg) and is configured to build ffmpeg with vcpkg, as an alternative to relying on system installations or manually built libraries.
A generic guide on how to use this build modality can be found in the original project's README.

## Cargo.toml 中 `[profile.release]` 的配置选项：

```toml
[profile.release]
# 优化级别
opt-level = 3          # 0-3, 'z', 's' - 控制编译优化级别
# opt-level = 'z'      # 优化大小，牺牲运行性能
# opt-level = 's'      # 优化大小，比'z'温和一些

# 链接时优化
lto = true            # 启用链接时优化，可以是 "true", "false", "thin", "fat"
# lto = "thin"        # 启用快速但不太彻底的 LTO
# lto = "fat"         # 启用更彻底但更慢的 LTO

# 二进制文件大小优化
strip = true          # 去除符号信息和调试信息，减小二进制大小
# strip = "symbols"   # 仅去除符号信息
# strip = "debuginfo" # 仅去除调试信息

# panic 处理
panic = "abort"       # panic 时直接终止程序，不进行展开
# panic = "unwind"    # panic 时展开调用栈(默认行为)

# 调试信息
debug = false         # 是否包含调试信息
# debug = 1          # 少量调试信息
# debug = 2          # 完整调试信息

# 运行时检查
debug-assertions = false     # 禁用调试断言
overflow-checks = false      # 禁用整数溢出检查
incremental = false         # 禁用增量编译

# 代码生成
codegen-units = 1          # 编译单元数量，1可以带来更好的优化但编译更慢
rpath = false             # 是否设置运行时库搜索路径

# 依赖项编译设置
[profile.release.build-override]
opt-level = 0            # 为构建脚本和自定义构建依赖项设置优化级别

# 特定包的设置
[profile.release.package."*"]
opt-level = 3            # 为所有依赖包设置优化级别

# 具体包的设置
[profile.release.package.example-package]
opt-level = 2            # 为特定包设置优化级别
```

常见优化组合示例：

1. 最小二进制大小:
```toml
[profile.release]
opt-level = 'z'
lto = true
strip = true
panic = "abort"
codegen-units = 1
```

2. 最佳性能:
```toml
[profile.release]
opt-level = 3
lto = true
panic = "abort"
codegen-units = 1
debug = false
```

3. 调试发布版本:
```toml
[profile.release]
opt-level = 3
debug = true
debug-assertions = true
overflow-checks = true
lto = false
```

4. 平衡的配置:
```toml
[profile.release]
opt-level = 2
lto = "thin"
strip = true
debug = false
```

重要说明：

1. `opt-level`:
    - 0: 无优化，编译最快
    - 1: 基本优化
    - 2: 一些优化
    - 3: 全部优化，编译最慢
    - 's': 优化大小，轻微影响性能
    - 'z': 最大程度优化大小，可能显著影响性能

2. `lto`（Link Time Optimization）：
    - 在链接时进行跨模块优化
    - 可以提高性能但增加编译时间
    - "thin" 是一个好的折中选择

3. `strip`:
    - 减小二进制大小
    - 移除调试信息和符号
    - 可能影响错误追踪能力

4. `panic`:
    - "abort" 可以减小二进制大小
    - 但会失去错误追踪能力

5. `codegen-units`:
    - 更小的值意味着更好的优化但更慢的编译
    - 1 提供最好的优化但编译最慢

使用建议：

1. 开发时使用默认配置
2. 发布时根据需求选择:
    - 需要小体积: 使用最小二进制大小配置
    - 需要性能: 使用最佳性能配置
    - 需要调试: 使用调试发布版本配置

这些配置可以根据你的具体需求进行调整和组合。建议在修改配置后进行充分测试，确保性能和稳定性符合预期。