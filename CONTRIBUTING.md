# Contributing to Solana Mint Address Generator

Thank you for your interest in contributing to the Solana Mint Address Generator! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Git
- Basic knowledge of Solana blockchain concepts

### Development Setup
1. Fork the repository
2. Clone your fork locally
3. Set up your development environment:
   ```bash
   git clone https://github.com/your-username/rustminter.git
   cd rustminter
   cargo build
   ```

## Development Guidelines

### Code Style
- Follow Rust conventions and use `rustfmt` for formatting
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and concise

### Testing
- Test your changes thoroughly before submitting
- Use small batch sizes for testing to avoid database load
- Verify both pump and bonk address generation
- Test error handling scenarios

### Performance Considerations
- Maintain high CPU utilization across all cores
- Optimize database operations for batch processing
- Consider memory usage for large generations
- Profile performance impact of changes

## Pull Request Process

### Before Submitting
1. **Test your changes** with various configurations
2. **Update documentation** if needed
3. **Check for breaking changes** and document them
4. **Ensure all tests pass**

### Pull Request Guidelines
1. **Clear title** describing the change
2. **Detailed description** of what was changed and why
3. **Screenshots or examples** for UI changes
4. **Performance impact** assessment if applicable

### Commit Messages
Use conventional commit format:
```
feat: add new batch processing option
fix: resolve database connection timeout
docs: update installation instructions
perf: optimize CPU utilization algorithm
```

## Areas for Contribution

### High Priority
- **Performance optimizations** for address generation
- **Database efficiency** improvements
- **Error handling** enhancements
- **Security** improvements

### Medium Priority
- **Additional address types** (new suffixes)
- **UI improvements** for batch scripts
- **Monitoring and logging** enhancements
- **Cross-platform** compatibility

### Low Priority
- **Documentation** improvements
- **Code refactoring** for maintainability
- **Testing** coverage expansion
- **Example scripts** and utilities

## Security Guidelines

### Sensitive Information
- **Never commit** `.env` files or API keys
- **Use environment variables** for configuration
- **Sanitize logs** to avoid exposing private keys
- **Follow security best practices** for database access

### Code Review
- All contributions require review
- Security-sensitive changes need additional scrutiny
- Database schema changes require careful consideration

## Community Guidelines

### Communication
- Be respectful and constructive
- Provide clear, actionable feedback
- Help other contributors when possible
- Follow the project's code of conduct

### Issue Reporting
When reporting issues:
1. **Check existing issues** first
2. **Provide detailed information** about the problem
3. **Include system details** (OS, Rust version, etc.)
4. **Share error messages** and logs
5. **Describe steps to reproduce**

## Getting Help

### Resources
- [Rust Documentation](https://doc.rust-lang.org/)
- [Solana Documentation](https://docs.solana.com/)
- [Supabase Documentation](https://supabase.com/docs)

### Questions
- Open an issue for questions
- Use GitHub Discussions if enabled
- Check existing documentation first

## Recognition

Contributors will be recognized in:
- Project README (for significant contributions)
- Release notes
- GitHub contributors list

Thank you for contributing to the Solana Mint Address Generator! 🚀 