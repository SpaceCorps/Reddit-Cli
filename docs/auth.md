---
title: "Authentication Guide"
description: "Authentication methods, OS keystore credential storage, and error handling for developers and AI agents using Reddit CLI."
author: "SpaceCorps"
date: "2026-09-24"
---

# Authentication Guide for Reddit CLI

This document outlines authentication methods, credential storage, and error handling for developers and AI agents using the Reddit CLI (`reddit`).

## Overview
Reddit CLI communicates with the Apify REST API (`v2`) to execute Reddit scraping workflows. An Apify API token is required to authenticate requests. Tokens can be provided via native OS keystores, environment variables, or per-command flags.

## Prerequisites
- An Apify account ([apify.com](https://www.apify.com))
- A personal API token from [Apify Console Integrations](https://console.apify.com/account/integrations)
- Reddit CLI installed (`cargo install --git https://github.com/SpaceCorps/Reddit-Cli --locked`)

## Authentication Methods

### 1. Interactive Login (`reddit login`)
The recommended flow for local development machines:
```bash
reddit login [account_name]
```
1. The CLI launches your default web browser directly to the Apify API integrations page.
2. Copy your personal API token.
3. Paste the token into the masked CLI prompt.
4. The CLI validates the token against `GET /v2/users/me`.
5. Upon verification, the token is encrypted and stored in your operating system's native keystore (macOS Keychain, Windows DPAPI, or Linux Secret Service).

### 2. Non-Interactive Login (CI/CD, Docker, Agents)
Pass the token through stdin:
```bash
echo "$APIFY_TOKEN" | reddit login [account_name] --api-key-stdin
```
Or pass it directly:
```bash
reddit login [account_name] --api-key "$APIFY_TOKEN"
```

### 3. Environment Variables
For automated containers or ad-hoc sessions, the CLI automatically detects:
- `APIFY_TOKEN`: Primary token environment variable.
- `REDDIT_API_KEY`: Alias token environment variable.

### 4. Direct Flag
Every command accepts `--api-key <KEY>`:
```bash
reddit search "rust 2024" --api-key "your_token"
```

## Multi-Account Management
Switch or verify accounts using:
```bash
# List configured accounts
reddit accounts list --check

# Test an account's token
reddit accounts test work

# Remove an account
reddit accounts remove old-account --yes
```

## Error Handling
When authentication fails, commands exit with non-zero exit codes and output standardized error payloads:
- `auth_required` (Exit code 3): Token missing, invalid, or expired.
- `no_account` (Exit code 7): Specified account name does not exist.
- `rate_limited` (Exit code 5): Apify rate limit or quota exceeded.
- `invalid_input` (Exit code 6): Argument validation error.
- `network` (Exit code 2): Connection timeout or network failure.
