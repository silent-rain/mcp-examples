# IDE-MCP部署示例

## 腾讯云代码助手 CodeBuddy

### CodeBuddy STDIO

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/rs-rmcp/target/x86_64-unknown-linux-gnu/debug/rs-mcpr",
    "run",
    "fastmcp",
    "run",
    "app/server.py"
    ]
}
```

### CodeBuddy SSE

> 启动服务端
> uv run fastmcp run app/server.py --transport sse --port 8000

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "sse",
    "url": "http://127.0.0.1:8000/sse"
}
```

### CodeBuddy Streamable HTTP [失败]

> 启动服务端
> uv run fastmcp run app/server.py --transport http --port 8000

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "streamable-http",
    "url": "http://127.0.0.1:8000/mcp"
}
```

### Python Package

将包发布到 Pypi 上, 自行拉取与本地运行

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "stdio",
    "command": "uvx",
    "args": [
    "--directory",
    "rust-mcp-server",
    ]
}
```

## Cline

### Cline STDIO

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/rs-rmcp/target/x86_64-unknown-linux-gnu/debug/rs-mcpr",
    "run",
    "mcp",
    "run",
    "app/server.py"
    ]
}
```

### Cline SSE

> 启动服务端
> uv run fastmcp run app/server.py --transport sse --port 8000

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "sse",
    "url": "http://127.0.0.1:8000/sse"
}
```

### Cline Streamable HTTP

CodeBuddy 视乎不支持该模式。

> 启动服务端
> uv run fastmcp run app/server.py --transport http --port 8000

```json
"rust-mcp-server": {
    "name": "rust-mcp-server",
    "type": "streamableHttp",
    "url": "http://127.0.0.1:8000/mcp"
}
```
