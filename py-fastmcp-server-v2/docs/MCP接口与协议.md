# MCP 接口与协议

## MCP 接口介绍

- [官方介绍](https://github.com/modelcontextprotocol/python-sdk?tab=readme-ov-file#mcp-primitives)

### resource

资源公开只读数据源（类似GET请求）。使用@mcp.resource("your://uri")。{placeholders}在 URI 中使用 来创建接受参数的动态模板，允许客户端请求特定的数据子集。

### tool

这些工具允许 LLM 通过执行 Python 函数（同步或异步）来执行操作。非常适合计算、API 调用或副作用（例如POST/ PUT）。FastMCP 处理从类型提示和文档字符串生成的模式。工具可以返回各种类型，包括文本、JSON 序列化对象，甚至借助 FastMCP 媒体辅助类的图像或音频。

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

提示符定义可重用的消息模板，用于指导 LLM 交互。使用 修饰函数@mcp.prompt。返回字符串或Message对象。

### Context

通过添加参数，即可在您的工具、资源或提示中访问 MCP 会话功能ctx: Context。Context 提供以下方法：

日志记录：ctx.info()使用、ctx.error()等将消息记录到 MCP 客户端。
LLM 抽样：用于ctx.sample()请求客户的 LLM 完成。
HTTP 请求：用于ctx.http_request()向其他服务器发出 HTTP 请求。
资源访问：用于ctx.read_resource()访问服务器上的资源
进度报告：用于ctx.report_progress()向客户报告进度。
以及更多...
要访问上下文，请Context向任何 mcp 装饰的函数添加一个带注释的参数。FastMCP 会在调用该函数时自动注入正确的上下文对象。

## MCP 协议

### 可流式传输的 HTTP 传输

- 注意：在生产部署中，可流式 HTTP 传输正在取代 SSE 传输。

```py
from fastmcp import FastMCP

mcp = FastMCP("Demo 🚀")

# TDIO（默认）：最适合本地工具和命令行脚本。
mcp.run(transport="stdio")  # Default, so transport argument is optional

# 可流式传输的 HTTP：推荐用于 Web 部署。
mcp.run(transport="http", host="127.0.0.1", port=8000, path="/mcp")


# SSE：为了与现有的 SSE 客户端兼容。
mcp.run(transport="sse", host="127.0.0.1", port=8000)
```

### 在 ASGI 应用程序中集成 FastMCP

- 集成 FastMCP

```py
from fastmcp import FastMCP
import uvicorn

mcp = FastMCP("MyServer")

app = mcp.http_app()

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

- 运行

```shell
uvicorn app.server:app --host 0.0.0.0 --port 8000
```
