package main

import (
	"context"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
)

func main() {
	// Create a new MCP server
	s := server.NewMCPServer(
		"Calculator Demo",
		"1.0.0",
		server.WithToolCapabilities(false),
		server.WithRecovery(),
	)

	// Static resource example - exposing a README file
	resource := mcp.NewResource(
		"docs://readme",
		"Project README",
		mcp.WithResourceDescription("The project's README file"),
		mcp.WithMIMEType("text/markdown"),
	)
	// Add resource with its handler
	s.AddResource(resource, staticResourceHandler)

	// Dynamic resource example - user profiles by ID
	s.AddResourceTemplate(
		mcp.NewResourceTemplate(
			"test://dynamic/resource/{id}",
			"Dynamic Resource",
		),
		resourceTemplateHandler,
	)

	s.AddPrompt(mcp.NewPrompt("simple_prompt",
		mcp.WithPromptDescription("A simple prompt"),
	), handleSimplePrompt)

	// Simple greeting prompt
	s.AddPrompt(mcp.NewPrompt("greeting",
		mcp.WithPromptDescription("A friendly greeting prompt"),
		mcp.WithArgument("name",
			mcp.ArgumentDescription("Name of the person to greet"),
		),
	), func(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
		name := request.Params.Arguments["name"]
		if name == "" {
			name = "friend"
		}

		return mcp.NewGetPromptResult(
			"A friendly greeting",
			[]mcp.PromptMessage{
				mcp.NewPromptMessage(
					mcp.RoleAssistant,
					mcp.NewTextContent(fmt.Sprintf("Hello, %s! How can I help you today?", name)),
				),
			},
		), nil
	})

	// Code review prompt with embedded resource
	s.AddPrompt(mcp.NewPrompt("code_review",
		mcp.WithPromptDescription("Code review assistance"),
		mcp.WithArgument("pr_number",
			mcp.ArgumentDescription("Pull request number to review"),
			mcp.RequiredArgument(),
		),
	), codeReviewPrompteHandler)

	// Database query builder prompt
	s.AddPrompt(mcp.NewPrompt("query_builder",
		mcp.WithPromptDescription("SQL query builder assistance"),
		mcp.WithArgument("table",
			mcp.ArgumentDescription("Name of the table to query"),
			mcp.RequiredArgument(),
		),
	), queryBuilderPrompteHandler)

	// Add tool
	helloTool := mcp.NewTool("hello_world",
		mcp.WithDescription("Say hello to someone"),
		mcp.WithString("name",
			mcp.Required(),
			mcp.Description("Name of the person to greet"),
		),
	)
	// Add tool handler
	s.AddTool(helloTool, helloHandler)

	// Add a calculator tool
	calculatorTool := mcp.NewTool("calculate",
		mcp.WithDescription("Perform basic arithmetic operations"),
		mcp.WithString("operation",
			mcp.Required(),
			mcp.Description("The operation to perform (add, subtract, multiply, divide)"),
			mcp.Enum("add", "subtract", "multiply", "divide"),
		),
		mcp.WithNumber("x",
			mcp.Required(),
			mcp.Description("First number"),
		),
		mcp.WithNumber("y",
			mcp.Required(),
			mcp.Description("Second number"),
		),
	)
	// Add the calculator handler
	s.AddTool(calculatorTool, calculatorHandler)

	// Add tool with complex schema
	greetingTool := mcp.NewTool("greeting",
		mcp.WithDescription("Generate a personalized greeting"),
		mcp.WithString("name",
			mcp.Required(),
			mcp.Description("Name of the person to greet"),
		),
		mcp.WithNumber("age",
			mcp.Description("Age of the person"),
			mcp.Min(0),
			mcp.Max(150),
		),
		mcp.WithBoolean("is_vip",
			mcp.Description("Whether the person is a VIP"),
			mcp.DefaultBool(false),
		),
		mcp.WithArray("languages",
			mcp.Description("Languages the person speaks"),
			mcp.Items(map[string]any{"type": "string"}),
		),
		mcp.WithObject("metadata",
			mcp.Description("Additional information about the person"),
			mcp.Properties(map[string]any{
				"location": map[string]any{
					"type":        "string",
					"description": "Current location",
				},
				"timezone": map[string]any{
					"type":        "string",
					"description": "Timezone",
				},
			}),
		),
	)
	// Add tool handler using the typed handler
	s.AddTool(greetingTool, mcp.NewTypedToolHandler(typedGreetingToolHandler))

	httpTool := mcp.NewTool("http_request",
		mcp.WithDescription("Make HTTP requests to external APIs"),
		mcp.WithString("method",
			mcp.Required(),
			mcp.Description("HTTP method to use"),
			mcp.Enum("GET", "POST", "PUT", "DELETE"),
		),
		mcp.WithString("url",
			mcp.Required(),
			mcp.Description("URL to send the request to"),
			mcp.Pattern("^https?://.*"),
		),
		mcp.WithString("body",
			mcp.Description("Request body (for POST/PUT)"),
		),
	)
	s.AddTool(httpTool, httpToolHandler)

	var transport string
	flag.StringVar(&transport, "t", "stdio", "Transport type (stdio or http)")
	flag.StringVar(&transport, "transport", "stdio", "Transport type (stdio or http)")
	flag.Parse()

	// Only check for "http" since stdio is the default
	if transport == "http" {
		httpServer := server.NewStreamableHTTPServer(s)
		log.Printf("HTTP server listening on :8080/mcp")
		if err := httpServer.Start(":8080"); err != nil {
			log.Fatalf("Server error: %v", err)
		}
	} else {
		if err := server.ServeStdio(s); err != nil {
			log.Fatalf("Server error: %v", err)
		}
	}
}

func staticResourceHandler(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	content, err := os.ReadFile("README.md")
	if err != nil {
		return nil, err
	}

	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      "docs://readme",
			MIMEType: "text/markdown",
			Text:     string(content),
		},
	}, nil
}

func resourceTemplateHandler(
	ctx context.Context,
	request mcp.ReadResourceRequest,
) ([]mcp.ResourceContents, error) {
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     "This is a sample resource",
		},
	}, nil
}

func handleSimplePrompt(
	ctx context.Context,
	request mcp.GetPromptRequest,
) (*mcp.GetPromptResult, error) {
	return &mcp.GetPromptResult{
		Description: "A simple prompt without arguments",
		Messages: []mcp.PromptMessage{
			{
				Role: mcp.RoleUser,
				Content: mcp.TextContent{
					Type: "text",
					Text: "This is a simple prompt without arguments.",
				},
			},
		},
	}, nil
}

func codeReviewPrompteHandler(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prNumber := request.Params.Arguments["pr_number"]
	if prNumber == "" {
		return nil, fmt.Errorf("pr_number is required")
	}

	return mcp.NewGetPromptResult(
		"Code review assistance",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(
				mcp.RoleUser,
				mcp.NewTextContent("Review the changes and provide constructive feedback."),
			),
			mcp.NewPromptMessage(
				mcp.RoleAssistant,
				mcp.NewEmbeddedResource(mcp.TextResourceContents{
					URI:      fmt.Sprintf("git://pulls/%s/diff", prNumber),
					MIMEType: "text/x-diff",
				}),
			),
		},
	), nil
}

func queryBuilderPrompteHandler(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	tableName := request.Params.Arguments["table"]
	if tableName == "" {
		return nil, fmt.Errorf("table name is required")
	}

	return mcp.NewGetPromptResult(
		"SQL query builder assistance",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(
				mcp.RoleUser,
				mcp.NewTextContent("Help construct efficient and safe queries for the provided schema."),
			),
			mcp.NewPromptMessage(
				mcp.RoleUser,
				mcp.NewEmbeddedResource(mcp.TextResourceContents{
					URI:      fmt.Sprintf("db://schema/%s", tableName),
					MIMEType: "application/json",
				}),
			),
		},
	), nil
}

func helloHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	name, err := request.RequireString("name")
	if err != nil {
		return mcp.NewToolResultError(err.Error()), nil
	}

	return mcp.NewToolResultText(fmt.Sprintf("Hello, %s!", name)), nil
}

func calculatorHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	// Using helper functions for type-safe argument access
	op, err := request.RequireString("operation")
	if err != nil {
		return mcp.NewToolResultError(err.Error()), nil
	}

	x, err := request.RequireFloat("x")
	if err != nil {
		return mcp.NewToolResultError(err.Error()), nil
	}

	y, err := request.RequireFloat("y")
	if err != nil {
		return mcp.NewToolResultError(err.Error()), nil
	}

	var result float64
	switch op {
	case "add":
		result = x + y
	case "subtract":
		result = x - y
	case "multiply":
		result = x * y
	case "divide":
		if y == 0 {
			return mcp.NewToolResultError("cannot divide by zero"), nil
		}
		result = x / y
	}

	return mcp.NewToolResultText(fmt.Sprintf("%.2f", result)), nil
}

// Define a struct for our typed arguments
type GreetingArgs struct {
	Name      string   `json:"name"`
	Age       int      `json:"age"`
	IsVIP     bool     `json:"is_vip"`
	Languages []string `json:"languages"`
	Metadata  struct {
		Location string `json:"location"`
		Timezone string `json:"timezone"`
	} `json:"metadata"`
}

// Our typed handler function that receives strongly-typed arguments
func typedGreetingToolHandler(ctx context.Context, request mcp.CallToolRequest, args GreetingArgs) (*mcp.CallToolResult, error) {
	if args.Name == "" {
		return mcp.NewToolResultError("name is required"), nil
	}

	// Build a personalized greeting based on the complex arguments
	greeting := fmt.Sprintf("Hello, %s!", args.Name)

	if args.Age > 0 {
		greeting += fmt.Sprintf(" You are %d years old.", args.Age)
	}

	if args.IsVIP {
		greeting += " Welcome back, valued VIP customer!"
	}

	if len(args.Languages) > 0 {
		greeting += fmt.Sprintf(" You speak %d languages: %v.", len(args.Languages), args.Languages)
	}

	if args.Metadata.Location != "" {
		greeting += fmt.Sprintf(" I see you're from %s.", args.Metadata.Location)

		if args.Metadata.Timezone != "" {
			greeting += fmt.Sprintf(" Your timezone is %s.", args.Metadata.Timezone)
		}
	}

	return mcp.NewToolResultText(greeting), nil
}

func httpToolHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := request.GetArguments()
	method := args["method"].(string)
	url := args["url"].(string)
	body := ""
	if b, ok := args["body"].(string); ok {
		body = b
	}

	// Create and send request
	var req *http.Request
	var err error
	if body != "" {
		req, err = http.NewRequest(method, url, strings.NewReader(body))
	} else {
		req, err = http.NewRequest(method, url, nil)
	}
	if err != nil {
		return mcp.NewToolResultErrorFromErr("unable to create request", err), nil
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return mcp.NewToolResultErrorFromErr("unable to execute request", err), nil
	}
	defer resp.Body.Close()

	// Return response
	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return mcp.NewToolResultErrorFromErr("unable to read request response", err), nil
	}

	return mcp.NewToolResultText(fmt.Sprintf("Status: %d\nBody: %s", resp.StatusCode, string(respBody))), nil
}
