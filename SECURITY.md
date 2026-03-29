# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.5.x   | :white_check_mark: |
| 0.4.x   | :white_check_mark: |
| 0.3.x   | :x:                |
| < 0.3   | :x:                |

## Reporting a Vulnerability

We take the security of StrataForge seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### How to Report

**Please DO NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via:
1. **GitHub Private Vulnerability Reporting** (Preferred)
   - Go to: https://github.com/ajamj/StrataForge/security/advisories/new
2. **Email**: security@strataforge.example.com (if available)

### What to Include

Please include the following information in your report:

- **Description**: Clear description of the vulnerability
- **Affected Versions**: Which versions are affected
- **Reproduction Steps**: How to reproduce the issue
- **Impact**: Potential impact of the vulnerability
- **Suggested Fix**: If you have suggestions for fixing it

### Response Time

We will acknowledge your report within **48 hours** and provide a more detailed response within **7 days** including:
- Confirmation of the vulnerability
- Timeline for fix
- Whether your report is in scope

### Security Update Process

1. **Triage** (1-3 days)
   - Confirm vulnerability
   - Assess severity
   - Determine affected versions

2. **Fix Development** (7-30 days depending on severity)
   - Develop fix
   - Test thoroughly
   - Prepare security advisory

3. **Release** (Coordinated disclosure)
   - Publish fix
   - Release new version
   - Public security advisory

4. **Post-Release**
   - Monitor for exploitation
   - Update documentation
   - Learn from incident

## Security Best Practices for Contributors

### Code Security

- **Input Validation**: Always validate user input
- **Authentication**: Never hardcode credentials
- **Dependencies**: Keep dependencies up to date
- **Secrets**: Use environment variables or secret managers

### Dependency Management

We use automated tools to keep dependencies secure:

- **Dependabot**: Weekly security scans
- **cargo-audit**: Regular vulnerability checks
- **GitHub Security Advisories**: Automated monitoring

### Commit Security

- Sign commits with GPG (recommended)
- Use descriptive commit messages
- Include security impact in PR descriptions

## Known Vulnerabilities

This section lists publicly known security vulnerabilities and their status.

| CVE ID | Severity | Status | Fixed Version | Reported Date |
|--------|----------|--------|---------------|---------------|
| - | - | - | - | - |

*No known vulnerabilities at this time*

## Security Tools

We use the following security tools:

- **GitHub Dependabot**: Automated dependency updates
- **cargo-audit**: Rust vulnerability scanner
- **GitHub Code Scanning**: Automated code review
- **Clippy**: Rust linter with security checks

## Contact

For security-related questions or concerns:
- **GitHub Security Advisories**: https://github.com/ajamj/StrataForge/security/advisories
- **Email**: (TBD)

---

**Last Updated**: 2026-03-29
**Policy Version**: 1.0
