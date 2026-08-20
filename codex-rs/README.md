# Codex CLI

[**Codex CLI Documentation**](https://developers.openai.com/codex/cli)

## Kilo AI Gateway

This fork adds a built-in `kilo` model provider that talks to the
[Kilo AI Gateway](https://api.kilo.ai/api/gateway) via the Chat Completions
API. It requires no ChatGPT subscription or login.

### Setup

1. Get an API key from [Kilo](https://app.kilo.ai) and set it in your
   environment:

   ```sh
   export KILO_API_KEY="..."
   ```

   (On Windows: `setx KILO_API_KEY "..."`.)

2. When `KILO_API_KEY` is set and no `model_provider` is configured, Codex
   automatically uses the `kilo` provider, so you can just run:

   ```sh
   codex
   ```

   The default model is selected from Kilo's model catalog. To pin a specific
   model, set it in `~/.codex/config.toml`:

   ```toml
   model_provider = "kilo"
   model = "anthropic/claude-sonnet-4.6"
   ```

3. If you prefer OpenAI, set `model_provider = "openai"` (or unset
   `KILO_API_KEY`) in your config.