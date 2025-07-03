# MCP 客户端
import json
from mcp import ClientSession, StdioServerParameters, types
from mcp.client.stdio import stdio_client
from pprint import pprint

# Create server parameters for stdio connection
server_params = StdioServerParameters(
    command="python",  # Executable
    args=["app/server.py"],  # Optional command line arguments
    env=None,  # Optional environment variables
)


# Optional: create a sampling callback
async def handle_sampling_message(
    message: types.CreateMessageRequestParams,
) -> types.CreateMessageResult:
    return types.CreateMessageResult(
        role="assistant",
        content=types.TextContent(
            type="text",
            text="Hello, world! from model",
        ),
        model="gpt-3.5-turbo",
        stopReason="endTurn",
    )


async def run():
    async with stdio_client(server_params) as (read, write):
        async with ClientSession(
            read, write, sampling_callback=handle_sampling_message
        ) as session:
            # Initialize the connection
            await session.initialize()

            # List available prompts
            prompts = await session.list_prompts()
            print("=== Available Prompts ===")
            print(prompts)
            print()

            # Get a prompt
            prompt = await session.get_prompt(
                "debug_error", arguments={"error": "this is py error, read file error"}
            )
            print("=== Prompt Details ===")
            print(prompt)
            print()

            # List available resources
            resources = await session.list_resources()
            print("=== Available Resources ===")
            print(resources)
            print()

            # List available tools
            tools = await session.list_tools()
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
            content, mime_type = await session.read_resource("file://some/path")
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
            result = await session.call_tool(
                "calculate_bmi", arguments={"weight_kg": 120, "height_m": 165}
            )
            print("=== Tool Result ===")
            print(result)
            print()


if __name__ == "__main__":
    import asyncio

    asyncio.run(run())
