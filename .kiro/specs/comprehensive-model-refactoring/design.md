# Design Document

## Overview

This design outlines a comprehensive refactoring to move all struct and enum definitions to the model layer and enforce strict import rules. The refactoring ensures proper architectural separation and consistent module organization patterns.

## Architecture

### Current Issues
- Struct and enum definitions scattered across different layers (controller, service, utils, exception)
- Inconsistent import patterns with specific module imports in non-mod.rs files
- Violation of architectural separation principles

### Target Architecture
```
app/
├── model/
│   ├── application/
│   │   └── controller/          # Controller-related structures
│   ├── domain/
│   │   ├── auth/               # Authentication structures and enums
│   │   ├── password/           # Password-related structures and enums
│   │   ├── exception/          # Exception enums
│   │   └── error_handler/      # Already moved
│   └── ...
├── controller/                 # Only logic, no struct definitions
├── service/                    # Only logic, no struct definitions
├── utils/                      # Only logic, no struct definitions
└── exception/                  # Only logic, no struct definitions
```

## Components and Interfaces

### Structures to Move

#### From `app/controller/auth/struct.rs`
- `AuthController` → `app/model/application/controller/auth.rs`

#### From `app/service/auth/struct.rs`
- `AuthService` → `app/model/domain/auth/service.rs`
- `AuthError` enum → `app/model/domain/auth/error.rs`

#### From `app/utils/password/struct.rs`
- `PasswordService` → `app/model/domain/password/service.rs`
- `PasswordStrength` → `app/model/domain/password/strength.rs`
- `PasswordStrengthLevel` enum → `app/model/domain/password/strength.rs`
- `PasswordError` enum → `app/model/domain/password/error.rs`

#### From `app/exception/database/enum.rs`
- `DatabaseException` enum → `app/model/domain/exception/database.rs`
- `AuthException` enum → `app/model/domain/exception/auth.rs`

### Import Rules Enforcement

#### Files that can have specific imports:
- `mod.rs` files - Can import and re-export modules
- `lib.rs` files - Can import and re-export modules

#### Files that must only use `use super::*;`:
- All `impl.rs`, `fn.rs`, `struct.rs`, `enum.rs` files
- All other non-mod.rs, non-lib.rs files

## Data Models

### Model Layer Organization

#### `app/model/application/controller/`
- `mod.rs` - Exports all controller structures
- `auth.rs` - AuthController structure

#### `app/model/domain/auth/`
- `mod.rs` - Exports auth-related structures
- `service.rs` - AuthService structure
- `error.rs` - AuthError enum

#### `app/model/domain/password/`
- `mod.rs` - Exports password-related structures
- `service.rs` - PasswordService structure
- `strength.rs` - PasswordStrength and PasswordStrengthLevel
- `error.rs` - PasswordError enum

#### `app/model/domain/exception/`
- `mod.rs` - Exports exception enums
- `database.rs` - DatabaseException enum
- `auth.rs` - AuthException enum

## Error Handling

The refactoring will maintain all existing error handling patterns while moving the error types to the appropriate model locations.

## Testing Strategy

### Validation Steps
1. **Structure Movement**: Move all structs and enums to model layer
2. **Import Cleanup**: Replace specific imports with `use super::*;`
3. **Module Updates**: Update all mod.rs files to properly export moved structures
4. **Compilation Test**: Ensure application compiles after each major change
5. **Functionality Test**: Verify all existing functionality works

### Implementation Approach
- Move structures in logical groups (auth, password, exceptions)
- Update imports incrementally
- Test compilation after each group
- Maintain backward compatibility through proper exports