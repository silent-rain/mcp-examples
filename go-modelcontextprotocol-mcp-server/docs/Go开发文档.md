# 环境搭建文档

## 系统环境

- 系统版本: Arch Linux x86_64
- Go版本: 1.23.4

## 创建项目

```shell
# 创建项目
mkdir go-modelcontextprotocol-mcp-server
cd go-modelcontextprotocol-mcp-server

# 初始化工作空间
go work init

# 创建server服务
mkdir server
cd server
# 初始化server服务
go mod init github.com/silent-rain/mcp-examples/go-modelcontextprotocol-mcp-server/server
# 添加到工作空间
cd ..
go work use ./server


# 创建client客户端
mkdir client
cd client
# 初始化client客户端
go mod init github.com/silent-rain/mcp-examples/go-modelcontextprotocol-mcp-server/client
# 添加到工作空间
cd ..
go work use ./client


# 运行服务
go run server/main.go
```

## 运行客户端

```shell
cd go-modelcontextprotocol-mcp-server

go build -o server/server server/main.go && \
go run client/main.go
```

## IDE 部署

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

- 使用绝对路径

```json
{
    "mcpServers": {
        "golang-mcp-server": {
            "command": "/home/one/code/mcp-examples/go-modelcontextprotocol-mcp-server/server/server",
            "args": [],
            "env": {}
        }
    }
}
```

## 相关文档

- [MCP Go SDK](https://github.com/modelcontextprotocol/go-sdk)
