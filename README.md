# Evo Kernel Agent: Learning

Discovers potential new skills from external sources such as registries, APIs, and community feeds

## Part of the Evo System

This is a kernel agent in the [Evo self-evolution agent system](https://github.com/ai-evo-agents). It runs using the generic `evo-runner` binary from [evo-agents](https://github.com/ai-evo-agents/evo-agents).

## Quick Start

```sh
# Download the runner binary for your platform
./download-runner.sh

# Set king address (default: http://localhost:3000)
export KING_ADDRESS=http://localhost:3000

# Run the agent
./evo-runner .
```

## Structure

```
evo-kernel-agent-learning/
├── soul.md              # Agent identity and behavior rules
├── skills/              # Skills this agent can use
├── mcp/                 # MCP server definitions
├── download-runner.sh   # Downloads evo-runner binary
└── api-key.config       # API key references (gitignored)
```

## License

MIT
