# 🌍 Global MCP Server Installation Guide

## ✅ Installation Complete!

MCP Server Everything sudah diinstall secara **GLOBAL** untuk semua workspace!

---

## 📦 Installed Packages

**Package:** `@modelcontextprotocol/server-everything`  
**Location:** Global npm packages  
**Command:** `mcp-server-everything`

---

## 🔧 Global Configuration

### **For Qwen Code (Global)**

**Location:** `~/.qwen-code/config.json` or `C:\Users\YourName\.qwen-code\config.json`

**Config:**
```json
{
  "mcpServers": {
    "everything": {
      "command": "mcp-server-everything",
      "args": []
    }
  }
}
```

**Steps:**
1. Open `~/.qwen-code/config.json`
2. Add the config above
3. Restart Qwen Code

---

### **For Gemini CLI (Global)**

**Location:** `~/.gemini-cli/config.json` or `C:\Users\YourName\.gemini-cli\config.json`

**Config:**
```json
{
  "mcp": {
    "servers": {
      "everything": {
        "command": "mcp-server-everything",
        "args": []
      }
    }
  }
}
```

**Steps:**
1. Open `~/.gemini-cli/config.json`
2. Add the config above
3. Restart Gemini CLI

---

### **For Cursor (Global)**

**Location:** Cursor Settings → MCP Servers

**Config:**
```json
{
  "mcpServers": [
    {
      "name": "everything",
      "command": "mcp-server-everything",
      "args": []
    }
  ]
}
```

**Steps:**
1. Open Cursor
2. Go to Settings → MCP Servers
3. Click "Add Server"
4. Select "Command"
5. Enter:
   - **Name:** `everything`
   - **Command:** `mcp-server-everything`
   - **Args:** `[]`
6. Save and restart Cursor

---

## 🧪 Test Installation

### **In Qwen Code:**
```
List available MCP resources
```

### **In Gemini CLI:**
```
Show MCP tools
```

### **In Cursor:**
```
What MCP servers are available?
```

---

## 📂 Workspace-Specific vs Global

### **Global Installation (This)**
✅ Available in ALL workspaces  
✅ One-time setup  
✅ System-wide availability  

### **Workspace-Specific (Previous)**
❌ Only available in StrataForge workspace  
❌ Need to setup for each project  
✅ Project-specific configuration  

---

## 🎯 Available Features

Once installed globally, you can use MCP features in ANY workspace:

**Resources:**
- ✅ Test resources
- ✅ Dynamic resources
- ✅ Slow resources (for testing timeouts)

**Prompts:**
- ✅ Simple prompts
- ✅ Complex prompts
- ✅ Resource-based prompts

**Tools:**
- ✅ Echo tool
- ✅ Add tool
- ✅ Long running operations
- ✅ Sample LLM
- ✅ Get tiny image

---

## 🛠️ Verification

### **Check Global Installation:**

**Windows:**
```powershell
npm list -g @modelcontextprotocol/server-everything
```

**Linux/Mac:**
```bash
npm list -g @modelcontextprotocol/server-everything
```

### **Test Server:**

```bash
mcp-server-everything --help
```

---

## 📝 Notes

1. **Global Command:** `mcp-server-everything` is now available in PATH
2. **No Workspace Needed:** Works in any directory
3. **Multiple AI Assistants:** Can be used by Qwen Code, Gemini CLI, Cursor simultaneously

---

## 🚀 Quick Start Summary

**Already Done:**
1. ✅ Installed globally via npm
2. ✅ Available system-wide
3. ✅ Ready for all AI assistants

**Next Steps:**
1. Add global config to Qwen Code
2. Add global config to Gemini CLI
3. Add global config to Cursor
4. Test in each assistant

---

**Status: MCP SERVER EVERYTHING INSTALLED GLOBALLY! 🌍🚀**
