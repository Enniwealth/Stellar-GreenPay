# 🤝 Contributing to Stellar GreenPay

Thank you for helping build transparent climate finance! Every contribution — big or small — matters.

---

## 🍴 Fork & Set Up

```bash
git clone https://github.com/YOUR_USERNAME/stellar-greenpay.git
cd stellar-greenpay
git remote add upstream https://github.com/your-org/stellar-greenpay.git
chmod +x scripts/setup-dev.sh && ./scripts/setup-dev.sh
```

---

## 🌿 Branch Naming

```
feature/impact-nft-badges
fix/leaderboard-sort-bug
docs/soroban-contract-guide
contracts/implement-co2-tracking
chore/upgrade-stellar-sdk
```

---

## 💬 Commit Style

```
feat: add donor leaderboard page
fix: correct CO2 offset calculation
docs: update contract deployment guide
contracts: implement impact NFT minting
chore: upgrade soroban-sdk to 21.0
```

---

## 🔃 Pull Request Process

1. Branch from `main`
2. Make your changes and test on Testnet
3. Open a PR against `main`
4. Fill in the PR template and link the issue (`Closes #123`)
5. Wait for review — we respond within 48 hours

---

## 📁 Project Structure

```
stellar-greenpay/
├── frontend/
│   ├── components/     ← Reusable UI components
│   ├── pages/          ← Next.js routes
│   ├── lib/            ← Stellar SDK + wallet + API helpers
│   └── utils/          ← Types, formatting, constants
├── backend/
│   └── src/
│       ├── routes/     ← Express routes
│       ├── services/   ← Business logic
│       └── middleware/ ← Auth, rate limiting
├── contracts/          ← Soroban smart contracts (Rust)
└── docs/               ← Architecture & API docs
```

Look for `good first issue` labels for beginner-friendly tasks!
