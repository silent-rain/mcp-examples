# 环境搭建文档

## 系统环境

- 系统版本: Arch Linux x86_64
- Rustc: v1.89.0
- Node: v24.4.0

## 创建项目

```shell
# 创建项目
cargo new rs-rmcp 
```

## MCP 可视化调试工具

```shell
npx -y @modelcontextprotocol/inspector@0.16.3

# or
npx -y @modelcontextprotocol/inspector@latest
```

## 运行服务端

```shell
# stdio
cargo run -- stdio

# sse
cargo run

# streamable-http
cargo run
```

## 运行客户端

```shell

```

## IDE 部署

```json
{
    "mcpServers": {
        "rust-mcp-server": {
            "command": "<your path to rust MCP server go executable>",
            "args": [],
            "env": {}
        }
    }
}
```

- 使用绝对路径

```json
{
    "mcpServers": {
        "rust-mcp-server": {
            "command": "/home/one/code/mcp-examples/rs-rmcp/target/x86_64-unknown-linux-gnu/debug/rs-mcpr",
            "args": ["stdio"], // stdio/http/sse
            "env": {}
        }
    }
}
```

## 相关文档

- [MCP Go SDK](https://github.com/modelcontextprotocol/go-sdk)
