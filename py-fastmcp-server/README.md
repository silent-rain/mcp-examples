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

## MCP 接口介绍

- [官方介绍](https://github.com/modelcontextprotocol/python-sdk?tab=readme-ov-file#mcp-primitives)

### resource

资源是你向 LLM 公开数据的方式。它们类似于 REST API 中的 GET 端点——它们提供数据，但不应执行大量计算或产生副作用

### tool

工具允许 LLM 通过你的服务器执行操作。与资源不同，工具需要执行计算并产生副作用。

### 结构化输出

如果返回类型注释兼容，默认情况下工具将返回结构化结果。否则，它们将返回非结构化的结果。

结构化输出支持以下返回类型：

- Pydantic模型（BaseModel子类）
- 类型图片
- 数据类和其他具有类型提示的类
- dict[str，T]（其中T是任何JSON可序列化类型）
- 原始类型（str、int、float、bool、bytes、None）-包装在{“result”：value}中
- 泛型类型（列表、元组、联合、可选等）-包装在{“result”：value}中

没有类型提示的类不能序列化为结构化输出。只有具有正确注释属性的类才会转换为Pydantic模型，用于模式生成和验证。
结构化结果会根据注释生成的输出模式自动验证。这确保了该工具返回类型正确、经过验证的数据，客户端可以轻松处理。

### Prompts

提示是可重复使用的模板，可帮助 LLM 有效地与您的服务器交互.

### Images

FastMCP 提供了一个Image自动处理图像数据的类

```py
from mcp.server.fastmcp import FastMCP, Image
from PIL import Image as PILImage

mcp = FastMCP("My App")


@mcp.tool()
def create_thumbnail(image_path: str) -> Image:
    """Create a thumbnail from an image"""
    img = PILImage.open(image_path)
    img.thumbnail((100, 100))
    return Image(data=img.tobytes(), format="png")
```

### Context

Context 对象使您的工具和资源能够访问 MCP 功能

```py
from mcp.server.fastmcp import FastMCP, Context

mcp = FastMCP("My App")


@mcp.tool()
async def long_task(files: list[str], ctx: Context) -> str:
    """Process multiple files with progress tracking"""
    for i, file in enumerate(files):
        ctx.info(f"Processing {file}")
        await ctx.report_progress(i, len(files))
        data, mime_type = await ctx.read_resource(f"file://{file}")
    return "Processing complete"
```

### Completions

MCP 支持为提示参数和资源模板参数提供补全建议。使用 context 参数，服务器可以根据先前解析的值提供补全建议

## MCP 协议

### 可流式传输的 HTTP 传输

- 注意：在生产部署中，可流式 HTTP 传输正在取代 SSE 传输。

```py
from mcp.server.fastmcp import FastMCP

# Stateful server (maintains session state)
mcp = FastMCP("StatefulServer")

# Stateless server (no session persistence)
mcp = FastMCP("StatelessServer", stateless_http=True)

# Stateless server (no session persistence, no sse stream with supported client)
mcp = FastMCP("StatelessServer", stateless_http=True, json_response=True)

# Run server with streamable_http transport
mcp.run(transport="streamable-http")
```

### 挂载到现有的 ASGI 服务器

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

## 相关文档

- [uv getting-started](https://docs.astral.sh/uv/getting-started)
- [modelcontextprotocol/python-sdk](https://github.com/modelcontextprotocol/python-sdk)
