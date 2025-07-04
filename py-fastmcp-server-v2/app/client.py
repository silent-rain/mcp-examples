# MCP 客户端
from fastmcp.client.sampling import RequestContext, SamplingMessage, SamplingParams
from fastmcp import Client
from fastmcp.client.logging import LogMessage
import json
import marvin
from pprint import pprint
import asyncio


async def log_handler(message: LogMessage):
    print(f"Server log: {message.data}")


async def progress_handler(progress: float, total: float | None, message: str | None):
    print(f"Progress: {progress}/{total} - {message}")


async def sampling_handler(
    messages: list[SamplingMessage],
    params: SamplingParams,
    ctx: RequestContext,
) -> str:
    return await marvin.say_async(
        message=[m.content.text for m in messages],
        instructions=params.systemPrompt,
    )


# client = Client("https://api.example.com/mcp")

client = Client(
    "app/server.py",
    log_handler=log_handler,
    progress_handler=progress_handler,
    sampling_handler=sampling_handler,
    timeout=30.0,
)


async def run():
    async with client:
        # Basic server interaction
        await client.ping()

        print(f"Connected: {client.is_connected()}")

        # List available prompts
        prompts = await client.list_prompts()
        print("=== Available Prompts ===")
        print(prompts)
        print()

        # Get a prompt
        prompt = await client.get_prompt(
            "debug_error", arguments={"error": "this is py error, read file error"}
        )
        print("=== Prompt Details ===")
        print(prompt)
        print()

        # List available resources
        resources = await client.list_resources()
        print("=== Available Resources ===")
        print(resources)
        print()

        # List available tools
        tools = await client.list_tools()
        print("=== Available Tools ===")
        pprint(tools, width=80, indent=2)
        print()
        # 将工具列表转换为可序列化的字典格式
        tools_dict = (
            [
                {
                    "name": tool.name,
                    "title": tool.title,
                    "description": tool.description,
                    "input_schema": tool.inputSchema,
                    "output_schema": tool.outputSchema,
                }
                for tool in tools.tools
            ]
            if hasattr(tools, "tools")
            else []
        )
        print("=== Available Tools Json Print ===")
        print(json.dumps(tools_dict, indent=4, ensure_ascii=False))
        print()

        # Read a resource
        content, mime_type = await client.read_resource("file://some/path")
        print("=== Resource Content ===")
        print(f"MIME Type: {mime_type}")
        if mime_type == "application/json":
            try:
                print(json.dumps(json.loads(content), indent=4, ensure_ascii=False))
            except:
                print(content)
        else:
            print(content)
        print()

        # Call a tool
        result = await client.call_tool(
            "calculate_bmi", arguments={"weight_kg": 120, "height_m": 165}
        )
        print("=== Tool Result ===")
        print(result)
        print()


if __name__ == "__main__":
    asyncio.run(run())
