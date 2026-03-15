# API Reference — Stellar GreenPay

Base URL: `http://localhost:4000`

All responses: `{ "success": true, "data": {...} }` or `{ "error": "..." }`

---

## Health
`GET /health` — Server status check.

---

## Projects

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/projects` | List projects (`?category=&status=active&limit=50`) |
| GET | `/api/projects/:id` | Get single project |

### Project object
```json
{
  "id": "uuid",
  "name": "Amazon Reforestation Initiative",
  "description": "...",
  "category": "Reforestation",
  "location": "Brazil, South America",
  "walletAddress": "GABC...XYZ",
  "goalXLM": "50000.0000000",
  "raisedXLM": "18420.0000000",
  "donorCount": 147,
  "co2OffsetKg": 245000,
  "status": "active",
  "verified": true,
  "tags": ["reforestation", "amazon"],
  "createdAt": "2025-01-01T00:00:00.000Z"
}
```

---

## Donations

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/donations` | Record a donation after on-chain tx |
| GET | `/api/donations/project/:id` | Donations for a project (`?limit=20`) |
| GET | `/api/donations/donor/:publicKey` | A donor's full history |

### POST /api/donations
```json
{
  "projectId": "uuid",
  "donorAddress": "GABC...XYZ",
  "amountXLM": "25.0000000",
  "message": "For the Amazon 🌳",
  "transactionHash": "abc123...64hexchars"
}
```

Donations are **deduplicated by transactionHash** — safe to retry.

---

## Profiles

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/profiles/:publicKey` | Get donor profile + badges |
| POST | `/api/profiles` | Create or update profile |

---

## Leaderboard

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/leaderboard` | Top donors by total XLM (`?limit=20`) |

### Leaderboard entry
```json
{
  "rank": 1,
  "publicKey": "GABC...XYZ",
  "displayName": "Alice",
  "totalDonatedXLM": "2500.0000000",
  "projectsSupported": 4,
  "topBadge": "earth"
}
```

---

## Project Updates

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/updates/:projectId` | Updates posted by a project |

---

## Badge Tiers

| Tier | Threshold | Emoji |
|------|-----------|-------|
| `seedling` | ≥ 10 XLM | 🌱 |
| `tree` | ≥ 100 XLM | 🌳 |
| `forest` | ≥ 500 XLM | 🌲 |
| `earth` | ≥ 2,000 XLM | 🌍 |
