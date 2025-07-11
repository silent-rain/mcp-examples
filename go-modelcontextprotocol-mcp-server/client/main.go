package main

import (
	"context"
	"log"
	"os/exec"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func main() {
	ctx := context.Background()

	// Create a new client, with no features.
	client := mcp.NewClient(&mcp.Implementation{Name: "mcp-client", Version: "v1.0.0"}, nil)

	// Connect to a server over stdin/stdout
	transport := mcp.NewCommandTransport(exec.Command("server/server"))

	session, err := client.Connect(ctx, transport)
	if err != nil {
		log.Fatal(err)
	}
	defer session.Close()

	// List Prompts
	listPromptsResult, err := session.ListPrompts(ctx, nil)
	if err != nil {
		log.Fatalf("PromptHandler failed: %v", err)
	}
	for _, c := range listPromptsResult.Prompts {
		log.Print(c)
	}

	// list resources
	listResourcesResult, err := session.ListResources(ctx, nil)
	if err != nil {
		log.Fatalf("ResourceHandler failed: %v", err)
	}
	for _, c := range listResourcesResult.Resources {
		log.Print(c)
	}

	// list tools
	listToolsResult, err := session.ListTools(ctx, nil)
	if err != nil {
		log.Fatalf("ToolHandler failed: %v", err)
	}
	for _, c := range listToolsResult.Tools {
		log.Print(c)
	}

	// Call a tool on the server.
	params := &mcp.CallToolParams{
		Name:      "greet",
		Arguments: map[string]any{"name": "you"},
	}
	res, err := session.CallTool(ctx, params)
	if err != nil {
		log.Fatalf("CallTool failed: %v", err)
	}
	if res.IsError {
		log.Fatal("tool failed")
	}
	for _, c := range res.Content {
		log.Print(c.(*mcp.TextContent).Text)
	}
}
