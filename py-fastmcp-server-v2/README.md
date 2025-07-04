# Python FastMCP Server V2

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

## 开发文档

### 开发模式

- 创建并同步环境

```shell
cd py-fastmcp-server-v2

uv sync
```

- 运行服务端

```shell
# 运行服务端
uv run fastmcp dev app/server.py

# Environment variables
uv run fastmcp dev app/server.py -v API_KEY=abc123 -v DB_URL=postgres://...
```

- 客户端

```shell
uv run app/client.py
```

### 服务端部署

- 运行服务端

```shell
# STDIO
uv run fastmcp run app/server.py
# SSE
uv run fastmcp run app/server.py --transport sse --port 8000
# Streamable HTTP
uv run fastmcp run app/server.py --transport http --port 8000
# or
uv run fastmcp run app/server.py --transport streamable-http --port 8000
# ASGI, 同时支持 SSE 和 Streamable HTTP
uvicorn app.server:app --host 0.0.0.0 --port 8000


# Environment variables
uv run fastmcp run app/server.py --transport sse --port 8000 -v API_KEY=abc123 -v DB_URL=postgres://...
```

## 相关文档

- [uv getting-started](https://docs.astral.sh/uv/getting-started)
- [FastMCP v2](https://github.com/jlowin/fastmcp)
- [FastMCP v2 Get Started](https://gofastmcp.com/getting-started/welcome)
- [llms.txt format](https://llmstxt.org/)
