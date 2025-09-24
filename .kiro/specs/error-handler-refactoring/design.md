# Design Document

## Overview

This design outlines the refactoring of the error_handler module from `app/middleware/error_handler` to `app/model/domain/error_handler`. The refactoring ensures proper architectural separation by placing all data structures and enums in the model layer while maintaining the existing functionality.

## Architecture

The current architecture has the error_handler module incorrectly placed in the middleware layer. The new architecture will follow the proper layered approach:

```
app/
├── middleware/          # Middleware logic only
├── model/
│   └── domain/          # Domain models and core structures
│       └── error_handler/  # Error handling structures and enums
└── ...
```

## Components and Interfaces

### Current Structure (to be moved)
- `app/middleware/error_handler/mod.rs` - Module declaration and exports
- `app/middleware/error_handler/struct.rs` - Data structures (ErrorHandler, ErrorResponse)
- `app/middleware/error_handler/impl.rs` - Implementation logic

### New Structure (target location)
- `app/model/domain/error_handler/mod.rs` - Module declaration and exports
- `app/model/domain/error_handler/struct.rs` - Data structures (ErrorHandler, ErrorResponse)
- `app/model/domain/error_handler/impl.rs` - Implementation logic

### Module Integration
- Update `app/middleware/mod.rs` to remove error_handler export
- Update `app/model/domain/mod.rs` to export error_handler module
- Ensure `app/model/mod.rs` properly exports the domain module

## Data Models

### ErrorHandler Struct
```rust
pub struct ErrorHandler;
```
- Simple unit struct for error handling operations
- Will remain unchanged in functionality

### ErrorResponse Struct
```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub timestamp: String,
}
```
- Serializable response structure for error information
- Contains error message, error code, and timestamp
- Will remain unchanged in functionality

## Error Handling

The error handling logic itself will remain unchanged:
- `ErrorHandler::new()` - Constructor for ErrorHandler
- `ErrorHandler::handle_error()` - Static method to create ErrorResponse with timestamp

## Testing Strategy

### Validation Steps
1. **Compilation Test**: Ensure the application compiles after refactoring
2. **Module Access Test**: Verify that the error_handler module can be imported from its new location
3. **Functionality Test**: Confirm that ErrorHandler and ErrorResponse work as before
4. **Integration Test**: Ensure no broken imports or missing dependencies

### Test Approach
- Move files first, then update imports
- Compile after each step to catch issues early
- Verify that all existing functionality is preserved

## Implementation Notes

### File Operations
1. Create new directory structure in `app/model/domain/error_handler/`
2. Copy existing files to new location
3. Update module declarations
4. Remove old files
5. Update import statements if any exist

### Dependencies
- The module uses `serde::Serialize` and `chrono::Utc`
- These dependencies will remain the same
- No additional dependencies required

### Potential Issues
- Currently no other modules import from error_handler, so minimal risk of breaking changes
- Need to ensure proper module exports through the model layer hierarchy