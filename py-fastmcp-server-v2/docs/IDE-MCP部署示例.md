# IDE-MCP部署示例

## 腾讯云代码助手 CodeBuddy

### CodeBuddy STDIO

```json
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/py-fastmcp-server-v2",
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
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "sse",
    "url": "http://127.0.0.1:8000/sse"
}
```

### CodeBuddy Streamable HTTP [失败]

> 启动服务端
> uv run fastmcp run app/server.py --transport http --port 8000

```json
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "streamable-http",
    "url": "http://127.0.0.1:8000/mcp"
}
```

### Python Package

将包发布到 Pypi 上, 自行拉取与本地运行

```json
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "stdio",
    "command": "uvx",
    "args": [
    "--directory",
    "py-fastmcp-server-v2",
    ]
}
```

## Cline

### Cline STDIO

```json
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "stdio",
    "command": "uv",
    "args": [
    "--directory",
    "/home/one/code/mcp-examples/py-fastmcp-server-v2",
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
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "sse",
    "url": "http://127.0.0.1:8000/sse"
}
```

### Cline Streamable HTTP

CodeBuddy 视乎不支持该模式。

> 启动服务端
> uv run fastmcp run app/server.py --transport http --port 8000

```json
"py-fastmcp-server-v2": {
    "name": "py-fastmcp-server-v2",
    "type": "streamableHttp",
    "url": "http://127.0.0.1:8000/mcp"
}
```
