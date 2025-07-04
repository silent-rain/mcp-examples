# MCP 服务端

from fastmcp import Context, FastMCP
from fastmcp.prompts.prompt import Message, PromptMessage, Role
import httpx
from pydantic import BaseModel, Field
from PIL import Image as PILImage
from fastmcp.utilities.types import Image
import asyncio
from starlette.applications import Starlette
from starlette.routing import Mount
import uvicorn


# Create an MCP server
mcp = FastMCP(
    "Demo",
)

# Create the ASGI app
mcp_app = mcp.http_app(transport="http", path="/")

# For legacy SSE transport (deprecated)
sse_app = mcp.http_app(transport="sse", path="/")


# Create a Starlette app and mount the MCP server
app = Starlette(
    routes=[
        Mount("/mcp", app=mcp_app),
        Mount("/sse", app=sse_app),
    ],
    # 对于 Streamable HTTP 传输，您必须将生命周期上下文从 FastMCP 应用传递到生成的 Starlette 应用，因为嵌套生命周期无法识别。
    # 否则，FastMCP 服务器的会话管理器将无法正确初始化。
    lifespan=mcp_app.lifespan,
)


# Add a dynamic greeting resource
@mcp.resource("greeting://{name}")
def get_greeting(name: str) -> str:
    """Get a personalized greeting"""
    return f"Hello, {name}!"


@mcp.resource("config://app", title="Application Configuration")
def get_config() -> str:
    """Static configuration data"""
    return "App configuration here"


@mcp.resource("users://{user_id}/profile", title="User Profile")
def get_user_profile(user_id: str) -> str:
    """Dynamic user data"""
    return f"Profile data for user {user_id}"


@mcp.resource("file://some/path")
def get_file() -> str:
    """Static file data"""
    return "File data here"


# 默认情况下，提示名称取自函数名称。
@mcp.prompt
def review_code(code: str) -> str:
    return f"Please review this code:\n\n{code}"


# 虽然 FastMCP 从您的函数中推断出名称和描述，但您可以覆盖这些并使用@mcp.prompt装饰器的参数添加其他元数据：
@mcp.prompt(title="Debug Assistant")
def debug_error(error: str) -> list[Message]:
    return [
        PromptMessage(content="I'm seeing this error:", role="user"),
        PromptMessage(content=error, role="user"),
        PromptMessage(
            content="I'll help debug that. What have you tried so far?",
            role="assistant",
        ),
    ]


# Add an addition tool
@mcp.tool
def add(a: int, b: int) -> int:
    """Add two numbers"""
    return a + b


@mcp.tool(title="BMI Calculator")
def calculate_bmi(weight_kg: float, height_m: float) -> float:
    """Calculate BMI given weight in kg and height in meters"""
    return weight_kg / (height_m**2)


@mcp.tool(title="Weather Fetcher")
async def fetch_weather(city: str) -> str:
    """Fetch current weather for a city"""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"https://api.weather.com/{city}")
        return response.text


# Context
@mcp.tool
async def process_data(uri: str, ctx: Context):
    # Log a message to the client
    await ctx.info(f"Processing {uri}...")

    # Read a resource from the server
    data = await ctx.read_resource(uri)

    # Ask client LLM to summarize the data
    summary = await ctx.sample(f"Summarize: {data.content[:500]}")

    # Return the summary
    return summary.text


# Using Pydantic models for rich structured data
class WeatherData(BaseModel):
    temperature: float = Field(description="Temperature in Celsius")
    humidity: float = Field(description="Humidity percentage")
    condition: str
    wind_speed: float


@mcp.tool()
def get_weather(city: str) -> WeatherData:
    """Get structured weather data"""
    return WeatherData(
        temperature=22.5, humidity=65.0, condition="partly cloudy", wind_speed=12.3
    )


@mcp.tool()
def create_thumbnail(image_path: str) -> Image:
    """Create a thumbnail from an image"""
    img = PILImage.open(image_path)
    img.thumbnail((100, 100))
    return Image(data=img.tobytes(), format="png")


# async def main():
#     # Use run_async() in async contexts
#     # transport: ("stdio", "sse", or "streamable-http")
#     # await mcp.run_async(transport="stdio")
#     # await mcp.run_async(
#     #     transport="sse",
#     #     host="127.0.0.1",
#     #     port=8000,
#     #     path="/mcp",
#     # )
#     # await mcp.run_async(transport="streamable-http", host="0.0.0.0", port=8000)
#     uvicorn.run(app, host="0.0.0.0", port=8000)


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
    # uvicorn app.server:app --host 0.0.0.0 --port 8000
