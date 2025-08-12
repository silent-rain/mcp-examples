# Go 官方 MCP Server

- 当前SDK尚未稳定, 使用的是指定的commit版本

## 开发文档

[Go开发文档](docs/Go开发文档.md)

## 编译与部署

### 编译

```shell
go build -o server/server server/main.go
```

### IDE 部署

```json
{
    "mcpServers": {
        "golang-mcp-server": {
            "command": "<your path to golang MCP server go executable>",
            "args": [],
            "env": {}
        }
    }
}
```
