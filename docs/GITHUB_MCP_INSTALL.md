# GitHub MCP Server Installation Guide

## 🎯 Overview

GitHub MCP Server allows AI assistants to interact with GitHub directly - create issues, manage PRs, search code, and more!

---

## 🔑 Prerequisites

### 1. Create GitHub Personal Access Token (PAT)

1. Go to: https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select scopes:
   - ✅ `repo` - Full control of private repositories
   - ✅ `read:org` - Read organization membership
   - ✅ `read:packages` - Read packages
4. Click "Generate token"
5. **Copy and save the token** (you won't see it again!)

---

## 🛠️ Installation Methods

### **Method 1: Remote Server (Recommended - Easiest)**

Uses GitHub's hosted MCP server - no local installation needed!

#### **For Qwen Code:**

Add to your Qwen Code configuration:

```json
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer YOUR_GITHUB_TOKEN"
      }
    }
  }
}
```

#### **For Gemini CLI:**

Add to `~/.gemini-cli/config.json`:

```json
{
  "mcp": {
    "servers": {
      "github": {
        "type": "http",
        "url": "https://api.githubcopilot.com/mcp/",
        "headers": {
          "Authorization": "Bearer YOUR_GITHUB_TOKEN"
        }
      }
    }
  }
}
```

#### **For Cursor:**

1. Open Cursor Settings
2. Go to "MCP Servers"
3. Click "Add Server"
4. Select "HTTP"
5. Enter:
   - **Name:** `github`
   - **URL:** `https://api.githubcopilot.com/mcp/`
   - **Headers:** `Authorization: Bearer YOUR_GITHUB_TOKEN`

---

### **Method 2: Local Docker Server**

Run GitHub MCP Server locally using Docker.

#### **Step 1: Install Docker**

Download from: https://www.docker.com/products/docker-desktop

#### **Step 2: Set Environment Variable**

**Windows (PowerShell):**
```powershell
$env:GITHUB_PERSONAL_ACCESS_TOKEN="your_token_here"
```

**Linux/Mac:**
```bash
export GITHUB_PERSONAL_ACCESS_TOKEN="your_token_here"
```

#### **Step 3: Configure for Qwen Code**

Add to Qwen Code MCP config:

```json
{
  "mcpServers": {
    "github": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e",
        "GITHUB_PERSONAL_ACCESS_TOKEN",
        "ghcr.io/github/github-mcp-server"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${input:github_token}"
      }
    }
  },
  "inputs": [
    {
      "type": "promptString",
      "id": "github_token",
      "description": "GitHub Personal Access Token",
      "password": true
    }
  ]
}
```

#### **Step 4: Configure for Gemini CLI**

Add to `~/.gemini-cli/config.json`:

```json
{
  "mcp": {
    "servers": {
      "github": {
        "command": "docker",
        "args": [
          "run",
          "-i",
          "--rm",
          "-e",
          "GITHUB_PERSONAL_ACCESS_TOKEN",
          "ghcr.io/github/github-mcp-server"
        ],
        "env": {
          "GITHUB_PERSONAL_ACCESS_TOKEN": "${env:GITHUB_PERSONAL_ACCESS_TOKEN}"
        }
      }
    }
  }
}
```

#### **Step 5: Configure for Cursor**

1. Open Cursor Settings → MCP Servers
2. Click "Add Server"
3. Select "Command"
4. Enter:
   - **Name:** `github`
   - **Command:** `docker`
   - **Args:** `["run", "-i", "--rm", "-e", "GITHUB_PERSONAL_ACCESS_TOKEN", "ghcr.io/github/github-mcp-server"]`
   - **Env:** `GITHUB_PERSONAL_ACCESS_TOKEN=your_token_here`

---

## ✅ Verification

### **Test GitHub MCP Server**

After installation, test with these commands:

**In Qwen Code:**
```
List my GitHub repositories
```

**In Gemini CLI:**
```
Show my GitHub issues
```

**In Cursor:**
```
Search GitHub for StrataForge
```

---

## 🔧 Available Features

Once installed, you can:

### **Repository Management**
- ✅ List repositories
- ✅ Create repositories
- ✅ Manage branches
- ✅ Search code

### **Issues**
- ✅ Create issues
- ✅ List issues
- ✅ Update issues
- ✅ Add comments

### **Pull Requests**
- ✅ Create PRs
- ✅ List PRs
- ✅ Review PRs
- ✅ Merge PRs

### **Actions**
- ✅ List workflows
- ✅ Trigger workflows
- ✅ Check run status

---

## 🛡️ Security Best Practices

### **1. Store PAT Securely**

**Windows:**
```powershell
# Use Windows Credential Manager
[System.Security.SecureString]::new()
```

**Linux/Mac:**
```bash
# Use keyring
chmod 600 ~/.github_token
```

### **2. Use Minimum Scopes**

Only request the scopes you actually need:
- For read-only: `public_repo`
- For private repos: `repo`
- For organizations: `read:org`

### **3. Rotate Tokens**

Regenerate your PAT every 90 days for security.

### **4. Never Commit Tokens**

Add to `.gitignore`:
```
.env
*.token
*token*
```

---

## 📚 Configuration Examples

### **Full Qwen Code Config**

Location: `~/.qwen-code/config.json`

```json
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer ghp_your_token_here"
      }
    }
  }
}
```

### **Full Gemini CLI Config**

Location: `~/.gemini-cli/config.json`

```json
{
  "mcp": {
    "servers": {
      "github": {
        "type": "http",
        "url": "https://api.githubcopilot.com/mcp/",
        "headers": {
          "Authorization": "Bearer ghp_your_token_here"
        }
      }
    }
  }
}
```

### **Full Cursor Config**

Location: Cursor Settings → MCP Servers

```json
{
  "mcpServers": [
    {
      "name": "github",
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer ghp_your_token_here"
      }
    }
  ]
}
```

---

## 🐛 Troubleshooting

### **Error: "Authentication failed"**
- ✅ Check your PAT is valid
- ✅ Verify token hasn't expired
- ✅ Ensure correct scopes are selected

### **Error: "Server not found"**
- ✅ Check internet connection
- ✅ Verify URL is correct
- ✅ Try remote server instead of local

### **Error: "Docker not found"**
- ✅ Install Docker Desktop
- ✅ Ensure Docker is running
- ✅ Check Docker daemon status

### **Error: "No repositories found"**
- ✅ Verify PAT has `repo` scope
- ✅ Check you're using correct GitHub account
- ✅ Ensure repositories exist

---

## 📖 Additional Resources

- **GitHub MCP Server Repo:** https://github.com/github/github-mcp-server
- **MCP Specification:** https://modelcontextprotocol.io
- **GitHub API Docs:** https://docs.github.com/en/rest

---

## 🎯 Quick Start Summary

**5-Minute Setup:**

1. **Create PAT** (2 min)
   - Go to https://github.com/settings/tokens
   - Generate token with `repo` scope

2. **Install for Qwen Code** (1 min)
   - Add config to `~/.qwen-code/config.json`
   - Add your token

3. **Install for Gemini CLI** (1 min)
   - Add config to `~/.gemini-cli/config.json`
   - Add your token

4. **Install for Cursor** (1 min)
   - Open Settings → MCP Servers
   - Add GitHub server with token

5. **Test** (1 min)
   - Ask AI: "List my GitHub repositories"
   - Should show your repos!

---

**Status: READY TO INSTALL! 🚀**
