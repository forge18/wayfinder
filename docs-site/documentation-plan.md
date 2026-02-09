# Wayfinder Documentation Plan

This document outlines the documentation strategy for Wayfinder, following a similar approach to the LuaNext project.

## Documentation Structure

Wayfinder follows a hybrid documentation strategy:

1. **Technical Documentation** (`docs/`): In-depth technical documents for contributors and advanced users
2. **User Documentation** (`docs-site/src/`): User-friendly guides and references for end users

## Technical Documentation (docs/)

The `docs/` directory contains technical documentation for contributors and advanced users:

- Implementation details
- Architecture decisions
- Design documents
- Technical specifications
- Developer guides

These documents are kept in the `docs/` directory to maintain them as the single source of truth for technical information.

## User Documentation (docs-site/)

The `docs-site/` directory contains user-facing documentation built with mdBook:

- Getting started guides
- Tutorials
- Reference materials
- CLI documentation
- Configuration guides

This content is organized in a user-friendly manner suitable for publishing as a documentation website.

## Documentation Categories

### Getting Started
- Installation guides
- Quick start tutorials
- Basic configuration

### Language Reference
- Debugging concepts
- Breakpoints and stepping
- Variable inspection
- Expression evaluation

### Guides
- IDE integration
- TypedLua debugging
- Hot code reload
- Multi-version support

### Reference
- CLI commands
- Configuration options
- Error codes
- DAP protocol details

### Contributing
- Development setup
- Architecture overview
- Coding guidelines

## Building Documentation

To build and preview the documentation locally:

1. Install mdBook:
   ```bash
   cargo install mdbook
   ```

2. Serve the documentation:
   ```bash
   cd docs-site
   mdbook serve --open
   ```

3. Build the documentation:
   ```bash
   cd docs-site
   mdbook build
   ```

## Publishing Documentation

Documentation is automatically published to GitHub Pages via GitHub Actions:

- Pushes to the `main` branch trigger automatic deployment
- Pull requests are validated for broken links
- Release publications create versioned documentation

The documentation site will be available at: `https://forge18.github.io/wayfinder/`

## Maintaining Documentation

### For Technical Documentation
1. Edit files in `docs/` for architecture, implementation details, and design docs
2. These are automatically linked from the user documentation
3. No manual syncing required

### For User-Facing Guides
1. Edit files in `docs-site/src/` for user guides and references
2. CI automatically rebuilds the documentation site
3. Preview available in PR checks
4. Merging to `main` deploys to production

## Documentation Tools

- **mdBook**: Static site generator for documentation
- **GitHub Pages**: Hosting platform (free)
- **GitHub Actions**: CI/CD pipeline for building and deploying
- **mdbook-linkcheck**: Automated link validation
- **mdbook-mermaid**: Diagram support (if needed)

## Success Criteria

- [x] Documentation site structure created
- [x] Basic content framework established
- [x] CI/CD pipeline configured
- [ ] Comprehensive user guides created
- [ ] Technical documentation linked appropriately
- [ ] API documentation integrated
- [ ] Zero broken links
- [ ] Search functionality working
- [ ] Mobile-responsive design
- [ ] Versioned documentation support (future)

## Future Enhancements

- Add mermaid diagram support for architecture visualization
- Implement versioned documentation for releases
- Add dark mode support
- Include interactive examples where appropriate
- Add performance benchmarks and comparisons