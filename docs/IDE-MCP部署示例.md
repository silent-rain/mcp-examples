# IDE-MCP部署示例

## 腾讯云代码助手 CodeBuddy

### CodeBuddy STDIO

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/py-fastmcp-server",
    "run",
    "mcp",
    "run",
    "app/server.py"
    ]
}
```

### CodeBuddy SSE

> 启动服务端
> uv run app/server.py

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server2",
    "type": "streamable-http",
    "url": "http://127.0.0.1:8000/mcp"
}
```

### CodeBuddy Streamable HTTP [失败]

> 启动服务端
> uv run app/server.py

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server",
    "type": "streamable-http",
    "url": "http://127.0.0.1:8000/mcp"
}
```

### Python Package

将包发布到 Pypi 上, 自行拉取与本地运行

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server",
    "type": "stdio",
    "command": "uvx",
    "args": [
    "--directory",
    "py-fastmcp-server",
    ]
}
```

## Cline

### Cline STDIO

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/py-fastmcp-server",
    "run",
    "mcp",
    "run",
    "app/server.py"
    ]
}
```

### Cline SSE

> 启动服务端
> uv run app/server.py

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server2",
    "type": "streamable-http",
    "url": "http://127.0.0.1:8000/mcp"
}
```

### Cline Streamable HTTP

CodeBuddy 视乎不支持该模式。

> 启动服务端
> uv run app/server.py

```json
"py-fastmcp-server": {
    "name": "py-fastmcp-server",
    "type": "streamableHttp",
    "url": "http://127.0.0.1:8000/mcp"
}
```
