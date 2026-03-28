# .gitignore Setup Guide

## ✅ Status: Complete

Comprehensive `.gitignore` sudah di-configure untuk StrataForge project.

---

## 📊 What's Ignored

### **Rust Build Artifacts**
```
/target/
*.dll, *.exe, *.lib
*.so, *.dylib
**/debug/, **/release/
```

### **Database Files**
```
*.sqlite, *.sqlite3, *.db
data/*.sqlite, data/*.db
```

### **IDE & Editor Files**
```
.vscode/, .idea/, .vs/
*.iml, *.ipr, *.suo
*.swp, *.swo, *~
```

### **OS Files**
```
.DS_Store (macOS)
Thumbs.db (Windows)
Desktop.ini (Windows)
```

### **Logs & Temp**
```
*.log
logs/, temp/, tmp/
*.tmp, *.temp
```

### **Test & Coverage**
```
coverage/
*.profraw, *.profdata
*.gcda, *.gcno
tarpaulin-report.html
```

### **Secrets & Credentials**
```
.env, .env.local
*.key, *.pem, *.crt
.aws/credentials
.github/secrets
```

### **Project-Specific**
```
*.sf/ (StrataForge project folders)
*.sfp (StrataForge project files)
data/blobstore/
data/projects/
```

### **Misc**
```
.qwen/ (Qwen Code state)
*.bak, *.backup, *.old
*.zip, *.tar.gz
```

---

## 🔍 Verification

### **Check What's Ignored:**
```bash
# Test if file would be ignored
git check-ignore -v path/to/file

# Examples:
git check-ignore -v target/debug/sf-app.exe
# Output: .gitignore:5:*.exe

git check-ignore -v .vscode/settings.json
# Output: .gitignore:72:.vscode/
```

### **Check What's Tracked:**
```bash
# See all tracked files
git ls-files

# See ignored files that are already tracked
git ls-files --others --ignored --exclude-standard
```

---

## 📋 Key Decisions

### **Ignored:**
- ✅ `Cargo.lock` - Allow different environments to have different dependency versions
- ✅ `target/` - Build artifacts should never be committed
- ✅ `*.sqlite` - Database files are environment-specific
- ✅ `.env` - Secrets should never be committed
- ✅ `.vscode/`, `.idea/` - IDE settings are personal

### **NOT Ignored (Tracked):**
- ✅ `Cargo.toml` - Project configuration
- ✅ `README.md` - Documentation
- ✅ `.github/workflows/` - CI/CD configuration
- ✅ `schemas/` - Database schemas
- ✅ `examples/` - Example files

---

## 🛠️ Maintenance

### **Add New Ignore Pattern:**

1. **Edit `.gitignore`:**
   ```bash
   code .gitignore
   ```

2. **Add pattern:**
   ```gitignore
   # New section
   *.newpattern
   ```

3. **Test:**
   ```bash
   git check-ignore -v file.newpattern
   ```

4. **Commit:**
   ```bash
   git add .gitignore
   git commit -m "chore: Add *.newpattern to gitignore"
   ```

### **Remove File from Git (Keep Local):**

```bash
# Stop tracking but keep file
git rm --cached path/to/file

# Commit change
git commit -m "chore: Stop tracking path/to/file"

# Add to .gitignore
echo "path/to/file" >> .gitignore
```

### **Remove File from Git (Delete Too):**

```bash
# Delete and stop tracking
git rm path/to/file
git commit -m "chore: Remove path/to/file"
```

---

## 🎯 Best Practices

### **DO:**
- ✅ Commit `.gitignore` early
- ✅ Use specific patterns
- ✅ Add comments for clarity
- ✅ Group related patterns
- ✅ Test with `git check-ignore`

### **DON'T:**
- ❌ Ignore `Cargo.toml` or `README.md`
- ❌ Use `*` without good reason
- ❌ Ignore entire directories unnecessarily
- ❌ Forget to test ignore patterns

---

## 📖 Resources

**Git Documentation:**
- [gitignore](https://git-scm.com/docs/gitignore)
- [git check-ignore](https://git-scm.com/docs/git-check-ignore)

**GitHub Guides:**
- [Ignoring files](https://docs.github.com/en/get-started/getting-started-with-git/ignoring-files)
- [.gitignore templates](https://github.com/github/gitignore)

**Rust-Specific:**
- [Rust .gitignore template](https://github.com/github/gitignore/blob/main/Rust.gitignore)

---

## 🔍 Common Issues

### **Issue: File still shows in `git status`**

**Cause:** File was already tracked before adding to `.gitignore`

**Solution:**
```bash
# Stop tracking
git rm --cached path/to/file

# Commit
git commit -m "chore: Stop tracking path/to/file"
```

### **Issue: Pattern not working**

**Cause:** Pattern syntax error or wrong path

**Solution:**
```bash
# Test pattern
git check-ignore -v path/to/file

# Check .gitignore syntax
# Patterns are relative to .gitignore location
# Use **/ for recursive matching
```

### **Issue: Want to ignore in specific directory only**

**Solution:**
```gitignore
# Ignore only in /data directory
data/*.sqlite

# Ignore everywhere
**/*.sqlite
```

---

## ✅ Verification Checklist

```
[✅] .gitignore committed to repository
[✅] Rust build artifacts ignored
[✅] IDE files ignored
[✅] OS files ignored
[✅] Secrets/credentials ignored
[✅] Database files ignored
[✅] Test artifacts ignored
[✅] Project-specific files ignored
[✅] Patterns tested with git check-ignore
```

---

**Status: .gitignore complete and pushed to GitHub!** 🎉
