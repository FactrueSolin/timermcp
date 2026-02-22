# timermcp

一个基于 Rust 和 rmcp 0.16.0 构建的 MCP（Model Context Protocol）时间服务。

## 项目介绍

timermcp 是一个轻量级的 MCP 时间服务，通过 StreamableHTTP 协议提供时间相关的工具功能。服务支持获取指定时区的当前时间以及等待功能。

## 技术栈

- **语言**: Rust
- **框架**: rmcp 0.16.0
- **协议**: StreamableHTTP
- **Web 框架**: Axum 0.8
- **时间处理**: chrono, chrono-tz

## 功能特性

### MCP Tools

#### 1. `get_time` - 获取当前时间

获取指定时区的当前时间。

**参数**:
| 参数名 | 类型 | 必填 | 默认值 | 说明 |
|--------|------|------|--------|------|
| timezone | string | 否 | Asia/Shanghai | 时区名称，如 "Asia/Shanghai", "America/New_York" |

**返回示例**:
```
当前时间（Asia/Shanghai）: 2026-02-22 20:05:00 CST
```

#### 2. `wait` - 等待功能

等待指定的秒数，然后返回等待的开始时间、结束时间和持续时间。

**参数**:
| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| seconds | number | 是 | 等待秒数 |

**返回示例**:
```
等待开始时间：2026-02-22 20:05:00 CST
等待结束时间：2026-02-22 20:05:05 CST
等待时长：5 秒
```

## 部署方式

### 方式一：Docker Compose（推荐）

```bash
docker compose up --build
```

服务将在 `http://localhost:18003/mcp` 启动。

### 方式二：本地运行

```bash
# 克隆项目后，在项目根目录执行
cargo run
```

服务将在 `http://localhost:8000/mcp` 启动。

## 环境变量配置

在项目根目录创建 `.env` 文件，配置以下环境变量：

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `BIND_ADDRESS` | 服务绑定地址 | 127.0.0.1:8000 |
| `RUST_LOG` | 日志级别（trace, debug, info, warn, error） | info |

### 示例 `.env` 文件

```env
# MCP 时间服务配置

# 服务绑定地址
BIND_ADDRESS=0.0.0.0:8000

# 日志级别 (trace, debug, info, warn, error)
RUST_LOG=info
```

## 服务端点

- **MCP 端点**: `http://localhost:8000/mcp`
- **协议**: StreamableHTTP

## 使用示例

### 通过 MCP 客户端调用

配置 MCP 客户端连接到 `http://localhost:8000/mcp` 后，可以调用以下工具：

```json
// 获取当前时间（默认时区）
{
  "name": "get_time",
  "arguments": {}
}

// 获取纽约时区的当前时间
{
  "name": "get_time",
  "arguments": {
    "timezone": "America/New_York"
  }
}

// 等待 5 秒
{
  "name": "wait",
  "arguments": {
    "seconds": 5
  }
}
```

## 项目结构

```
timermcp/
├── src/
│   └── main.rs          # 主程序入口
├── .env                 # 环境变量配置
├── .gitignore
├── Cargo.toml           # Rust 项目配置
├── docker-compose.yml   # Docker Compose 配置
├── Dockerfile           # Docker 镜像配置
└── README.md            # 项目文档
```

## 开发

### 构建检查

```bash
cargo check
```

### 运行测试

```bash
cargo test
```

## 许可证

本项目采用 MIT 许可证。
