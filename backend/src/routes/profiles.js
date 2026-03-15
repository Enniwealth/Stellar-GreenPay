/**
 * src/routes/profiles.js
 */
"use strict";
const express = require("express");
const router  = express.Router();
const { profiles } = require("../services/store");

function validateKey(k) {
  if (!k || !/^G[A-Z0-9]{55}$/.test(k)) { const e = new Error("Invalid public key"); e.status = 400; throw e; }
}

router.get("/:publicKey", (req, res, next) => {
  try {
    validateKey(req.params.publicKey);
    const p = profiles.get(req.params.publicKey);
    if (!p) { const e = new Error("Profile not found"); e.status = 404; throw e; }
    res.json({ success: true, data: p });
  } catch (e) { next(e); }
});

router.post("/", (req, res, next) => {
  try {
    const { publicKey, displayName, bio } = req.body;
    validateKey(publicKey);
    const existing = profiles.get(publicKey) || {
      publicKey, totalDonatedXLM: "0", projectsSupported: 0,
      badges: [], createdAt: new Date().toISOString(),
    };
    const updated = {
      ...existing,
      displayName: displayName?.trim().slice(0, 30) || existing.displayName || null,
      bio:         bio?.trim().slice(0, 300)         || existing.bio         || null,
      updatedAt:   new Date().toISOString(),
    };
    profiles.set(publicKey, updated);
    res.json({ success: true, data: updated });
  } catch (e) { next(e); }
});

module.exports = router;
