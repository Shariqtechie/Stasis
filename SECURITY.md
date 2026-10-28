# Security Policy

Stasis edits Windows Update pause settings in the registry and runs as administrator, so security reports are taken seriously.

## Reporting a vulnerability
Please use GitHub's private **Report a vulnerability** option under the Security tab. Do not open a public issue for security problems. Expect a reply within a few days.

## Verifying a download
Each release includes `Stasis.exe.sha256`. Compare it with:
```powershell
Get-FileHash .\Stasis.exe -Algorithm SHA256
```
