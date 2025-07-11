# TS MCP Server

## 开发文档

- [TS开发文档](./docs/TS开发文档.md)
- [IDE-MCP部署示例](./docs/IDE-MCP部署示例.md)

## 编译与运行

```shell
# 调试运行
pnpm dev


# 编译
pnpm build
# 运行服务
pnpm start
```

## 客户端

```shell
# 调试运行
pnpm client
```

## IDE 部署

发布后可以直接拉取与运行。

```json
{
  "mcpServers": {
    "ts-mcp-server": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "ts-mcp-server"]
    }
  }
}
```
