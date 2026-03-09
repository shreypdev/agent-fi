# AgentFi Public Agent

Public demo agent for AgentFi — echo skill, REST bridge, and OpenAPI. Runs on port **3010**.

## Run locally

From repo root:

```bash
npm run public:agent
```

Or from this directory:

```bash
npm install && npm run start
```

## Endpoints

| Endpoint | Description |
|----------|-------------|
| `/.well-known/agent-card.json` | A2A Agent Card (discovery) |
| `/openapi.json` | OpenAPI 3.0 spec for ChatGPT Action editor |
| `GET/POST /api/chat?message=<text>` | REST bridge — returns echo response |

## Exposing via ngrok

To get a stable HTTPS URL for ChatGPT, Claude, or Cursor:

1. Install [ngrok](https://ngrok.com/download).
2. Start the public agent: `npm run playground` (from root) or `npm run start` (from this folder).
3. In another terminal: `ngrok http 3010`.
4. Copy the HTTPS URL (e.g. `https://abc123.ngrok.io`) and set it as the base for the Agent Card:
   ```bash
   PUBLIC_AGENT_URL=https://abc123.ngrok.io npm run start
   ```
   Or use a reserved ngrok domain and set `PUBLIC_AGENT_URL` in your environment.

**Done when:** `curl https://<url>/api/chat?message=Hello` returns the echo response, and `curl https://<url>/.well-known/agent-card.json` returns the agent card.
