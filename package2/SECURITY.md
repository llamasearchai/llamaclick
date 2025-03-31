# Security Policy

## Supported Versions

We are committed to providing security updates for the following versions of LlamaClick:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| 0.0.x   | :x:                |

## Reporting a Vulnerability

We take the security of LlamaClick seriously. If you believe you've found a security vulnerability, please follow these steps:

1. **Do not disclose the vulnerability publicly**
2. **Email us** at security@llamasearch.ai with:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggestions for remediation
3. **Allow time for response** - We aim to acknowledge reports within 48 hours and provide an estimated timeline for a fix
4. **Coordinate disclosure** - Once the vulnerability is fixed, we will work with you on the public disclosure timing

## Security Design Principles

LlamaClick is built with the following security principles in mind:

### 1. Defense in Depth

- Multiple layers of security controls
- Failing securely by default
- Principle of least privilege for all operations

### 2. Data Protection

- Sensitive data (API keys, credentials) are encrypted at rest
- All data in transit is encrypted using TLS
- No sensitive data is logged or stored unnecessarily
- Session data is isolated and ephemeral

### 3. Third-Party Dependencies

- Regular scanning of dependencies for vulnerabilities
- Pinned versions to prevent supply chain attacks
- Minimized dependency footprint

### 4. Browser Isolation

- Browser sessions run in isolated contexts
- Content Security Policy implementation
- Cross-Origin Resource Sharing (CORS) protection

### 5. Secure Defaults

- No default credentials
- Secure default configurations
- Explicit opt-in for sensitive features

## Security Features

LlamaClick includes the following security features:

### Credential Management

- Secure storage of API keys and tokens
- Support for environment variables and external secret managers
- Automatic credential rotation capabilities

### Access Control

- Role-based access to automation features
- Audit logging of all automation actions
- Session management with timeout and revocation

### Network Security

- Control over which domains can be accessed
- Proxy support for network isolation
- User-Agent customization and fingerprint management

### Data Handling

- Configurable data retention policies
- Export and deletion capabilities for compliance
- Input validation and output encoding

## Security Development Lifecycle

Our security development process includes:

1. **Planning**: Threat modeling and security requirements
2. **Development**: Secure coding practices and peer reviews
3. **Testing**: SAST, DAST, and manual security reviews
4. **Release**: Final security verification
5. **Maintenance**: Ongoing vulnerability management

## Responsible Disclosure Timeline

When a vulnerability is reported:

1. **0 days**: Acknowledgement of report
2. **7 days**: Validation and severity assessment 
3. **30 days**: Target timeline for patch development
4. **60 days**: Public disclosure (may be extended for complex issues)

## Acknowledgments

We would like to thank the following security researchers for their responsible disclosures:

- No acknowledgments yet - your name could be here!

## Contact

For any security concerns, please contact us at security@llamasearch.ai. 