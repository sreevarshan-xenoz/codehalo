# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability within **CodeHalo**, please do **NOT** open a public issue.

Instead, please send an advisory or report directly to the repository maintainers via:
- GitHub Private Vulnerability Reporting (Security tab -> "Report a vulnerability")
- Or via email to the core maintainers.

Please include:
1. Description of the vulnerability.
2. Steps to reproduce or proof-of-concept.
3. Impact assessment.
4. Suggested remediation if available.

We will acknowledge receipt within 48 hours and work on a prompt remediation before disclosure.

---

## Core Security Commitments

1. **Local-First**: CodeHalo never transmits API keys, tokens, or usage metrics to third-party or proprietary telemetry servers.
2. **Native Keyrings**: Credentials and tokens will strictly leverage platform-native secure stores (Windows Credential Manager / DPAPI, Linux Secret Service / FreeDesktop Secret API).
3. **No Unsafe Code**: All platform abstractions prioritize memory safety and strict bounds checking.
