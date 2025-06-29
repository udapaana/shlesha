# Security Setup & Token Management

This document explains the security configuration and token management for the Shlesha release system.

## 🔐 Environment-Based Security Model

### GitHub Environments

The project uses **two isolated GitHub environments** for deployment security:

#### **dev Environment** (Release Candidates)
- **Purpose**: RC testing and validation
- **Access**: Development team, automated RC deployments
- **Tokens**: Separate npm token, shared crates.io token
- **Protection**: Minimal (fast iteration)

#### **prd Environment** (Stable Releases)  
- **Purpose**: Production stable releases
- **Access**: Release managers, approved deployments
- **Tokens**: Separate npm token, shared crates.io token
- **Protection**: Full (approvals, reviews, timing)

## 🎯 Token Configuration

### Environment Secrets Matrix

| Environment | Secret | Usage | Scope |
|-------------|--------|-------|-------|
| **dev** | `NPM_TOKEN` | RC npm publishing | `@rc` tag |
| **dev** | `CARGO_REGISTRY_TOKEN` | RC crates.io publishing | Pre-release |
| **prd** | `NPM_TOKEN` | Stable npm publishing | `@latest` tag |
| **prd** | `CARGO_REGISTRY_TOKEN` | Stable crates.io publishing | Stable release |

### PyPI Authentication

Uses **OIDC Trusted Publishing** (no tokens required):

| Environment | Target | Authentication |
|-------------|--------|----------------|
| **dev** | TestPyPI | OIDC Trusted Publisher |
| **prd** | Production PyPI | OIDC Trusted Publisher |

## 🛡️ Security Benefits

### 1. **Token Isolation**
```
RC Compromise ❌ → Stable Releases ✅ (Protected)
Stable Compromise ❌ → RC Releases ✅ (Isolated)
```

### 2. **Blast Radius Limitation**
- **RC token leak**: Cannot affect production npm packages
- **Stable token leak**: Cannot affect RC testing workflow
- **Environment breach**: Other environment remains secure

### 3. **Access Control Granularity**
```yaml
dev environment:
  - Junior developers: Read access
  - Senior developers: Deploy access
  - Automated systems: RC deployment

prd environment:  
  - Release managers: Deploy access
  - Security team: Review access
  - Automated systems: Stable deployment (with approval)
```

### 4. **Audit Trail Separation**
- **Clear deployment history** per environment
- **Separate logs** for RC vs stable releases
- **Independent access reviews** per environment

## 🔧 Token Setup Instructions

### npm Token Creation

Create **two separate npm tokens**:

#### RC Token (dev environment)
```bash
# Login to npm
npm login

# Create automation token for RC
# Name: "shlesha-rc-releases"  
# Type: "Automation"
# Scope: "@rc tag publishing"
```

#### Stable Token (prd environment)
```bash
# Create automation token for stable  
# Name: "shlesha-stable-releases"
# Type: "Automation" 
# Scope: "latest tag publishing"
```

### crates.io Token Setup

Create **one crates.io token** (used in both environments):

```bash
# Visit: https://crates.io/settings/tokens
# Create token with scopes:
# ✅ publish-new
# ✅ publish-update  
# ✅ yank

# Token name: "shlesha-releases"
# Add to both dev AND prd environments
```

### PyPI Trusted Publishing Setup

Configure **OIDC trusted publishers** (no tokens needed):

#### TestPyPI (dev)
```
Publisher: udapaana/shlesha
Workflow: python.yml
Environment: dev
```

#### Production PyPI (prd)  
```
Publisher: udapaana/shlesha
Workflow: python.yml
Environment: prd
```

## 🚨 Security Best Practices

### Token Rotation

#### Quarterly Rotation Schedule
- **Q1**: Rotate npm tokens
- **Q2**: Review access permissions
- **Q3**: Rotate crates.io token
- **Q4**: Security audit

#### Emergency Rotation
```bash
# If token compromise suspected:
1. Immediately revoke compromised token
2. Create new token with same scopes
3. Update environment secret
4. Test deployment pipeline
5. Document incident
```

### Access Management

#### Principle of Least Privilege
- **Developers**: Only dev environment access
- **Release managers**: Both environment access
- **CI/CD systems**: Scoped to specific workflows

#### Regular Access Reviews
- **Monthly**: Review environment access lists
- **Quarterly**: Audit token usage and permissions
- **Annually**: Complete security assessment

### Monitoring & Alerting

#### Deploy Notifications
```yaml
# Slack/email alerts for:
- Successful RC deployments (dev)
- Successful stable deployments (prd)  
- Failed deployments (any environment)
- Unusual deployment patterns
```

#### Security Monitoring
```yaml
# Monitor for:
- Unexpected token usage
- Failed authentication attempts
- Environment access changes
- Token permission modifications
```

## 🔄 Deployment Protection Rules

### dev Environment
```yaml
# Minimal protection for fast iteration
deployment_protection_rules:
  - required_reviewers: 0
  - wait_timer: 0
  - allowed_branches: ["main"]
  - prevent_self_review: false
```

### prd Environment
```yaml
# Full protection for stable releases
deployment_protection_rules:
  - required_reviewers: 1
  - wait_timer: 300  # 5 minute delay
  - allowed_branches: ["main"]
  - prevent_self_review: true
  - restrict_pushes: true
```

## 📋 Security Checklist

### Initial Setup
- [ ] Create separate npm tokens for dev/prd
- [ ] Create crates.io token with minimal scopes
- [ ] Configure PyPI trusted publishers
- [ ] Set up environment protection rules
- [ ] Test deployment pipeline
- [ ] Document token locations and scopes

### Regular Maintenance
- [ ] Monthly access review
- [ ] Quarterly token rotation
- [ ] Annual security audit
- [ ] Monitor deployment logs
- [ ] Review environment configurations
- [ ] Update documentation

### Incident Response
- [ ] Token revocation procedures
- [ ] Emergency contact list
- [ ] Rollback procedures
- [ ] Communication templates
- [ ] Post-incident review process

## 🔗 Related Documentation

- [DEPLOYMENT.md](../DEPLOYMENT.md) - Complete deployment guide
- [CRATES_IO_RC_SUPPORT.md](CRATES_IO_RC_SUPPORT.md) - RC publishing details
- [RELEASE_SYSTEM.md](RELEASE_SYSTEM.md) - Release workflow overview

## 🎯 Summary

The enhanced security model provides:

✅ **Complete token isolation** between RC and stable releases  
✅ **Granular access control** through GitHub environments  
✅ **Zero-token PyPI** publishing through OIDC  
✅ **Audit trails** and deployment monitoring  
✅ **Emergency procedures** for security incidents  

This setup ensures that compromise of any single token or environment cannot affect the entire release pipeline.