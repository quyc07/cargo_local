# Cargo Local Tools

`cargo_local` 是一个用于管理本地 Cargo 依赖的命令行工具。它可以列出所有已安装的 crates，并提供搜索功能。

## 🚀 安装

使用 `cargo install` 进行安装：

```sh
cargo install cargo_local_tools
```

## 📌 使用方法

```sh
cargo_local_tools <COMMAND> [OPTIONS]
```

### **可用命令**

| 命令                    | 说明                |
| --------------------- | ----------------- |
| `list`                | 显示所有本地已安装的 crates |
| `search <crate_name>` | 搜索本地 crates       |
| `help`                | 显示帮助信息            |

### **示例**

#### 1️⃣ 列出所有本地 crates

```sh
cargo_local_tools list
```

#### 2️⃣ 搜索指定 crate

```sh
cargo_local_tools search anyhow
```

#### 3️⃣ 显示帮助信息

```sh
cargo_local_tools help
```

## 🔧 贡献

欢迎提交 Issue 或 Pull Request 来改进本项目。

## 📜 许可证

本项目基于 MIT 许可证发布。

