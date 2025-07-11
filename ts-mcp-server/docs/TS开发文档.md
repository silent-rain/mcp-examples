# 环境搭建文档

## 系统环境

- 系统版本: Arch Linux x86_64
- Node版本: v24.4.0
- Pnpm版本: 10.13.1

## 搭建项目

### 创建项目

```shell
pnpm init -y
```

### 添加 MCP 库

```shell
# 安装 MCP 库与 zod 数据验证库
pnpm add @modelcontextprotocol/sdk zod
# 安装 typescript 编译器 和 Node.js 的 TypeScript 类型定义文件
pnpm add -D typescript @types/node
```

### 编译

```shell
# 编译
pnpm build
# 运行服务
pnpm start
```

### mcp-cli 测试 MCP服务

```shell
# 运行服务端且用于测试mcp服务接口
npx @wong2/mcp-cli node ./build/index.js args...
```

## 相关文档

- [modelcontextprotocol/typescript-sdk](https://github.com/modelcontextprotocol/typescript-sdk)
- [使用typescript-sdk构建一个Hello MCP！](https://www.5ee.net/archives/tmXJAgWz)
