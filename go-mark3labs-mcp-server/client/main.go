package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/mark3labs/mcp-go/client"
	"github.com/mark3labs/mcp-go/mcp"
)

func main() {
	c, err := client.NewStdioMCPClient(
		"./server/server",
		[]string{},    // Empty ENV
		[]string{}..., // Empty Args
	)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}
	defer c.Close()

	// Create context with timeout
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Initialize the client
	fmt.Println("Initializing client...")
	initRequest := mcp.InitializeRequest{}
	initRequest.Params.ProtocolVersion = mcp.LATEST_PROTOCOL_VERSION
	initRequest.Params.ClientInfo = mcp.Implementation{
		Name:    "example-client",
		Version: "1.0.0",
	}

	initResult, err := c.Initialize(ctx, initRequest)
	if err != nil {
		log.Fatalf("Failed to initialize: %v", err)
	}
	fmt.Printf(
		"Initialized with server: %s %s\n\n",
		initResult.ServerInfo.Name,
		initResult.ServerInfo.Version,
	)

	// List Resources
	fmt.Println("Listing available resources...")
	listRequest := mcp.ListResourcesRequest{}
	listResult, err := c.ListResources(ctx, listRequest)
	if err != nil {
		log.Fatalf("Failed to list resources: %v", err)
	}
	for _, resource := range listResult.Resources {
		fmt.Printf("- %s: %s\n", resource.URI, resource.MIMEType)
	}

	// Read Resource
	fmt.Println("Reading resource...")
	readRequest := mcp.ReadResourceRequest{}
	readRequest.Params.URI = "docs://readme"
	readResults, err := c.ReadResource(ctx, readRequest)
	if err != nil {
		log.Fatalf("Failed to read resource: %v", err)
	}
	readResult := readResults.Contents
	fmt.Printf("Read resource: %s\n", readResult)

	// List Parameters
	fmt.Println("Listing available parameters...")
	paramsRequest := mcp.ListPromptsRequest{}
	params, err := c.ListPrompts(ctx, paramsRequest)
	if err != nil {
		log.Fatalf("Failed to list parameters: %v", err)
	}
	for _, param := range params.Prompts {
		fmt.Printf("- %s: %s\n", param.Name, param.Description)
	}

	// Read Parameters
	fmt.Println("Reading parameters...")
	readParamsRequest := mcp.GetPromptRequest{
		Params: mcp.GetPromptParams{
			Name: "greeting",
			Arguments: map[string]string{
				"name": "zs",
			},
		},
	}
	readParamsResult, err := c.GetPrompt(ctx, readParamsRequest)
	if err != nil {
		log.Fatalf("Failed to read parameters: %v", err)
	}
	fmt.Printf("Read parameters description: %v\n", readParamsResult.Description)

	// List Tools
	fmt.Println("Listing available tools...")
	toolsRequest := mcp.ListToolsRequest{}
	tools, err := c.ListTools(ctx, toolsRequest)
	if err != nil {
		log.Fatalf("Failed to list tools: %v", err)
	}
	for _, tool := range tools.Tools {
		fmt.Printf("- %s: %s\n", tool.Name, tool.Description)
	}
	fmt.Println()

	// Call Tools
	fmt.Println("Calling tools...")
	callRequest := mcp.CallToolRequest{
		Params: mcp.CallToolParams{
			Name: "calculate",
			Arguments: map[string]string{
				"operation": "add",
				"x":         "1",
				"y":         "2",
			},
		},
	}
	callResult, err := c.CallTool(ctx, callRequest)
	if err != nil {
		log.Fatalf("Failed to call tool: %v", err)
	}
	if callResult.IsError {
		log.Fatalf("Call tool failed: %v", callResult.Result)
	}
	fmt.Printf("Call tool result: %v\n", callResult.Content)

}
