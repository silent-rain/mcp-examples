import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";

const transport = new StdioClientTransport({
  command: "node",
  args: ["./src/index.ts"],
});

const client = new Client({
  name: "example-client",
  version: "1.0.0",
});

// 启动客户端
async function main() {
  await client.connect(transport);

  // List prompts
  const prompts = await client.listPrompts();
  console.log(prompts)

  // Get a prompt
  const prompt = await client.getPrompt({
    name: "review-code",
    arguments: {
      code: "this is some code",
    },
  });
  console.log(prompt)

  // List resources
  const resources = await client.listResources();
  console.log(resources)

  // Read a resource
  const resource = await client.readResource({
    uri: "config://app",
  });
  console.log(resource)

  // Call a tool
  const result = await client.callTool({
    name: "calculate-bmi",
    arguments: {
      weightKg: 120,
      heightM: 165,
    },
  });
  console.log(result)

}

main().catch((error) => {
  console.error("客户端启动失败:", error);
  process.exit(1);
});
