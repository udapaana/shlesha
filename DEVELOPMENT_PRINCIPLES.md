# Development Principles for Shlesha Transliterator

## Core Modular Architecture

### 1. Complete Modularity
- Each module defines a clear interface and maintains its own todo list
- Modules are self-contained with well-defined boundaries
- Interface contracts must be respected at all times

### 2. Single Module Mutability
- Only one module is mutable at any given time
- All other modules are marked as immutable during active development
- This prevents conflicting changes and maintains code stability

### 3. Cross-Module Communication Protocol
- Calling functions cannot directly modify other modules
- Changes to external modules must be requested via their todo lists
- Module switching is required to implement cross-module changes

### 4. Interface-Only Access
- Modules can only access other modules through their defined interfaces
- Internal implementation details remain hidden
- This enforces loose coupling and high cohesion

### 5. Change Logging
- All changes are logged after each commit
- Detailed change logs enable quick rollback to previous versions
- Commit messages should clearly describe the module and changes made

### 6. Frequent Small Commits
- Make small, focused commits frequently
- Each commit should represent a single logical change
- This provides granular version control and easier debugging

### 7. Test-Driven Development
- Every module must have comprehensive tests before being marked complete
- Write tests immediately after implementing core functionality
- Tests should cover all public interface methods and error conditions
- Use tests to drive development and verify module contracts
- Modules are not considered "done" until tests pass

## Implementation Guidelines

- Before starting work on a module, clearly define its interface
- Create todo lists for each module to track pending work
- Write comprehensive tests for each module immediately after implementation
- Use git commits strategically to checkpoint progress
- Document interface changes that affect other modules
- Maintain change logs for architectural decisions
- Run tests before switching module focus