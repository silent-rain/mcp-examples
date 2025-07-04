# Python FastMCP Server

## 环境搭建

### 系统环境

- 系统版本: Arch Linux x86_64
- Python版本: 3.13.1
- UV版本: 0.7.18

### UV 安装

```shell
curl -LsSf https://astral.sh/uv/install.sh | sh
# or
pipx install uv
```

## 项目文档

- [IDE-MCP部署示例](./docs/IDE-MCP部署示例.md)
- [接口与协议](./docs/接口与协议.md)

## 开发文档

### 开发模式

- 创建并同步环境

```shell
cd py-fastmcp-server

uv sync
```

- 服务端

```shell
# 运行服务端
uv run mcp dev app/server.py

# Environment variables
uv run mcp dev app/server.py -v API_KEY=abc123 -v DB_URL=postgres://...

# 其他运行方式
# python app/server.py
# uvicorn app.server:mcp --reload
```

- 客户端

```shell
uv run app/client.py
# or
python app/client.py
```

### 服务部署

```shell
# 运行服务端
uv run mcp run app/server.py

# Environment variables
uv run mcp run app/server.py -v API_KEY=abc123 -v DB_URL=postgres://...
```

### gunicorn+uvicorn 部署

- ASGI  包装
  - 注意：SSE 传输正在被Streamable HTTP 传输取代。

```py
from starlette.applications import Starlette
from starlette.routing import Mount, Host
from mcp.server.fastmcp import FastMCP


mcp = FastMCP("My App")

# Mount the SSE server to the existing ASGI server
app = Starlette(
    routes=[
        Mount('/', app=mcp.sse_app()),
    ]
)

# or dynamically mount as host
app.router.routes.append(Host('mcp.acme.corp', app=mcp.sse_app()))
```

- 验证是否为 ASGI 应用
  - Gunicorn 需要明确的 ASGI 实例
  - 如果输出 False，说明 mcp 不是有效的 ASGI 应用。

```shell
python -c "from app.server import mcp; print(callable(mcp))"
```

- 通常配合 gunicorn + uvicorn 多进程

```shell
gunicorn -w 4 -k uvicorn.workers.UvicornWorker app.server:mcp
```

## 相关文档

- [uv getting-started](https://docs.astral.sh/uv/getting-started)
- [modelcontextprotocol/python-sdk](https://github.com/modelcontextprotocol/python-sdk)
