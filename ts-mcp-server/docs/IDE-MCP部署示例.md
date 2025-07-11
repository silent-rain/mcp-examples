# IDE-MCP 部署示例

## 腾讯云代码助手 CodeBuddy

### CodeBuddy STDIO

- 运行一个已安装的 NPM 包

```json
"ts-mcp-server": {
    "type": "stdio",
    "command": "npx",
    "args": ["-y", "ts-mcp-server"]
}
```

- Node.js 运行本地项目

```json
"ts-mcp-server": {
    "type": "stdio",
    "command": "node",
    "args": ["/home/one/code/mcp-examples/ts-mcp-server/dist/index.js"]
}
```

- Npx 运行本地项目

```json
"ts-mcp-server": {
    "type": "stdio",
    "command": "npx",
    "args": ["ts-node", "/home/one/code/mcp-examples/ts-mcp-server/src/index.ts"]
}
```

## Cline

### Cline STDIO

```json
"ts-mcp-server": {
    "type": "stdio",
    "command": "node",
    "args": ["/home/one/code/mcp-examples/ts-mcp-server/dist/index.js"]
}
```

```json
"ts-mcp-server": {
    "type": "stdio",
    "command": "npx",
    "args": ["-y", "ts-mcp-server"]
}
```
