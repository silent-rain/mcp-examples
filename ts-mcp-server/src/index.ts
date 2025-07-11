#!/usr/bin/env node
import { completable } from "@modelcontextprotocol/sdk/server/completable.js";
import {
  McpServer,
  ResourceTemplate,
} from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";

// Create an MCP server
// McpServer 是 MCP 协议的核心接口。它负责连接管理、协议合规性和消息路由
const server = new McpServer({
  name: "demo-server",
  version: "1.0.0",
});

// Static resource
// 资源是你向 LLM 公开数据的方式。它们类似于 REST API 中的 GET 端点——它们提供数据，但不应执行大量计算或产生副作用
server.registerResource(
  "config",
  "config://app",
  {
    title: "Application Config",
    description: "Application configuration data",
    mimeType: "text/plain",
  },
  async (uri) => ({
    contents: [
      {
        uri: uri.href,
        text: "App configuration here",
      },
    ],
  })
);

// Dynamic resource with parameters
server.registerResource(
  "user-profile",
  new ResourceTemplate("users://{userId}/profile", { list: undefined }),
  {
    title: "User Profile",
    description: "User profile information",
  },
  async (uri, { userId }) => ({
    contents: [
      {
        uri: uri.href,
        text: `Profile data for user ${userId}`,
      },
    ],
  })
);

// Resource with context-aware completion
server.registerResource(
  "repository",
  new ResourceTemplate("github://repos/{owner}/{repo}", {
    list: undefined,
    complete: {
      // Provide intelligent completions based on previously resolved parameters
      repo: (value, context) => {
        if (context?.arguments?.["owner"] === "org1") {
          return ["project1", "project2", "project3"].filter((r) =>
            r.startsWith(value)
          );
        }
        return ["default-repo"].filter((r) => r.startsWith(value));
      },
    },
  }),
  {
    title: "GitHub Repository",
    description: "Repository information",
  },
  async (uri, { owner, repo }) => ({
    contents: [
      {
        uri: uri.href,
        text: `Repository: ${owner}/${repo}`,
      },
    ],
  })
);

// Prompt
// 提示是可重复使用的模板，可帮助 LLM 有效地与您的服务器交互
server.registerPrompt(
  "review-code",
  {
    title: "Code Review",
    description: "Review code for best practices and potential issues",
    argsSchema: { code: z.string() },
  },
  ({ code }) => ({
    messages: [
      {
        role: "user",
        content: {
          type: "text",
          text: `Please review this code:\n\n${code}`,
        },
      },
    ],
  })
);

// Prompt with context-aware completion
server.registerPrompt(
  "team-greeting",
  {
    title: "Team Greeting",
    description: "Generate a greeting for team members",
    argsSchema: {
      department: completable(z.string(), (value) => {
        // Department suggestions
        return ["engineering", "sales", "marketing", "support"].filter((d) =>
          d.startsWith(value)
        );
      }),
      name: completable(z.string(), (value, context) => {
        // Name suggestions based on selected department
        const department = context?.arguments?.["department"];
        if (department === "engineering") {
          return ["Alice", "Bob", "Charlie"].filter((n) => n.startsWith(value));
        } else if (department === "sales") {
          return ["David", "Eve", "Frank"].filter((n) => n.startsWith(value));
        } else if (department === "marketing") {
          return ["Grace", "Henry", "Iris"].filter((n) => n.startsWith(value));
        }
        return ["Guest"].filter((n) => n.startsWith(value));
      }),
    },
  },
  ({ department, name }) => ({
    messages: [
      {
        role: "assistant",
        content: {
          type: "text",
          text: `Hello ${name}, welcome to the ${department} team!`,
        },
      },
    ],
  })
);

// Add an addition tool
// 工具允许 LLM 通过你的服务器执行操作。与资源不同，工具需要执行计算并产生副作用
// Simple tool with parameters
server.registerTool(
  "calculate-bmi",
  {
    title: "BMI Calculator",
    description: "Calculate Body Mass Index",
    inputSchema: {
      weightKg: z.number().describe("Weight in kilograms"),
      heightM: z.number().describe("Height in meters"),
    },
  },
  async ({ weightKg, heightM }) => ({
    content: [
      {
        type: "text",
        text: String(weightKg / (heightM * heightM)),
      },
    ],
  })
);

// Async tool with external API call
server.registerTool(
  "fetch-weather",
  {
    title: "Weather Fetcher",
    description: "Get weather data for a city",
    inputSchema: { city: z.string().describe("A city name") },
  },
  async ({ city }) => {
    const response = await fetch(`https://api.weather.com/${city}`);
    const data = await response.text();
    return {
      content: [{ type: "text", text: data }],
    };
  }
);

// Tool that returns ResourceLinks
server.registerTool(
  "list-files",
  {
    title: "List Files",
    description: "List project files",
    inputSchema: { pattern: z.string().describe("A glob pattern") },
  },
  async ({ pattern }) => ({
    content: [
      { type: "text", text: `Found files matching "${pattern}":` },
      // ResourceLinks let tools return references without file content
      {
        type: "resource_link",
        uri: "file:///project/README.md",
        name: "README.md",
        mimeType: "text/markdown",
        description: "A README file",
      },
      {
        type: "resource_link",
        uri: "file:///project/src/index.ts",
        name: "index.ts",
        mimeType: "text/typescript",
        description: "An index file",
      },
    ],
  })
);

// 添加一个简单的打招呼工具, 旧的写法
server.tool(
  "say_hello",
  { name: z.string().describe("要问候的名字") },
  async (params: { name: string }) => ({
    content: [{ type: "text", text: `你好，${params.name}！欢迎使用MCP!` }],
  })
);

// 添加一个加法工具, 旧的写法
server.tool(
  "add",
  {
    a: z.number().describe("第一个数字"),
    b: z.number().describe("第二个数字"),
  },
  async (params: { a: number; b: number }) => ({
    content: [
      {
        type: "text",
        text: `${params.a} + ${params.b} = ${params.a + params.b}`,
      },
    ],
  })
);

// sampling
// Tool that uses LLM sampling to summarize any text
// MCP 服务器可以向支持采样的连接客户端请求 LLM 完成。
server.registerTool(
  "summarize",
  {
    description: "Summarize any text using an LLM",
    inputSchema: {
      text: z.string().describe("Text to summarize"),
    },
  },
  async ({ text }) => {
    // Call the LLM through MCP sampling
    const response = await server.server.createMessage({
      messages: [
        {
          role: "user",
          content: {
            type: "text",
            text: `Please summarize the following text concisely:\n\n${text}`,
          },
        },
      ],
      maxTokens: 500,
    });

    return {
      content: [
        {
          type: "text",
          text: response.content.type === "text" ? response.content.text : "Unable to generate summary",
        },
      ],
    };
  }
);



// 启动服务器
async function main() {
  // 标准输入输出
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("MCP 服务器已启动");
}

main().catch((error) => {
  console.error("服务器启动失败:", error);
  process.exit(1);
});