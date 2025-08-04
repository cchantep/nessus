---
title: Derive Macros
layout: page
nav_order: 5
parent: User Guide
---

# Derive Macros

This guide provides detailed documentation about Tnuctipun's procedural macros. The derive macros (`FieldWitnesses` and `MongoComparable`) are the foundation that enables type-safe field access and query building.

## Table of Contents

- [Overview](#overview)
- [FieldWitnesses Macro](#fieldwitnesses-macro)
- [MongoComparable Macro](#mongocomparable-macro)
- [Field Naming Strategies](#field-naming-strategies)
- [Field-Level Attributes](#field-level-attributes)
- [Private Field Handling](#private-field-handling)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Overview

Tnuctipun uses procedural macros to generate compile-time field witnesses that enable type-safe MongoDB operations. These macros analyze your struct definitions and generate the necessary code for field validation and query building.

### Core Concepts

- **Field Witnesses**: Zero-cost compile-time types that represent struct fields
- **Type Safety**: Field names and types are validated at compile time
- **Code Generation**: Macros generate helper modules and implementations
- **Zero Runtime Overhead**: All validation happens at compile time

### Required Derives

```rust
use tnuctipun::{FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}
```

**Required for all structs:**

- `FieldWitnesses`: Generates field witness types
- `MongoComparable`: Enables comparison operations for filtering
- `Serialize`/`Deserialize`: Required for MongoDB document conversion

## FieldWitnesses Macro

The `FieldWitnesses` macro is the core of Tnuctipun's type safety system. It generates field witness types and helper modules.

### Basic Usage

```rust
#[derive(FieldWitnesses)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}
```

**Generated Code (conceptual):**

```rust
mod user_fields {
    pub struct Name;
    pub struct Age;
    pub struct Email;
    
    impl FieldName for Name {
        fn field_name() -> &'static str { "name" }
    }
    
    impl FieldName for Age {
        fn field_name() -> &'static str { "age" }
    }
    
    impl FieldName for Email {
        fn field_name() -> &'static str { "email" }
    }
}
```

### Field Witness Usage

Field witnesses are used in query building:

```rust
use tnuctipun::filters::empty;

// Type-safe field access
let mut filter = empty::<User>();
filter.eq::<user_fields::Name, _>("John".to_string());  // ✅ Valid
filter.eq::<user_fields::InvalidField, _>("value");     // ❌ Compile error
```

### Struct Naming and Module Generation

The macro generates module names based on the struct name:

```rust
#[derive(FieldWitnesses)]
struct User { ... }           // → user_fields module

#[derive(FieldWitnesses)]
struct UserProfile { ... }    // → user_profile_fields module

#[derive(FieldWitnesses)]
struct OrderItem { ... }      // → order_item_fields module
```

**Naming Rules:**

- Convert PascalCase struct names to snake_case
- Append `_fields` suffix
- Handle name conflicts automatically with scope isolation

## MongoComparable Macro

The `MongoComparable` macro enables comparison operations for query building.

### Basic Usage

```rust
#[derive(MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}
```

### Generated Capabilities

The macro enables these comparison operations:

```rust
// Equality and inequality
filter.eq::<user_fields::Name, _>("John".to_string());
filter.ne::<user_fields::Age, _>(0);

// Numeric comparisons
filter.gt::<user_fields::Age, _>(18);
filter.gte::<user_fields::Age, _>(21);
filter.lt::<user_fields::Age, _>(65);
filter.lte::<user_fields::Age, _>(64);

// Array operations
filter.in_array::<user_fields::Name, _>(vec!["John".to_string(), "Jane".to_string()]);
filter.nin::<user_fields::Email, _>(vec!["spam@example.com".to_string()]);
```

## Field Naming Strategies

Tnuctipun supports different field naming strategies to match your MongoDB schema conventions.

### Default Naming (snake_case)

By default, field names match the Rust field names:

```rust
#[derive(FieldWitnesses)]
struct User {
    user_name: String,      // → "user_name"
    email_address: String,  // → "email_address"
    created_at: String,     // → "created_at"
}
```

### Built-in Naming Strategies

#### PascalCase Strategy

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "PascalCase")]
struct User {
    user_name: String,      // → "UserName"
    email_address: String,  // → "EmailAddress"
    created_at: String,     // → "CreatedAt"
}
```

#### camelCase Strategy

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct User {
    user_name: String,      // → "userName"
    email_address: String,  // → "emailAddress"
    created_at: String,     // → "createdAt"
}
```

### Naming Strategy Examples

```rust
// Different naming strategies for the same struct
#[derive(FieldWitnesses)]
struct DefaultNaming {
    user_name: String,        // → "user_name"
    email_address: String,    // → "email_address"
    is_admin: bool,          // → "is_admin"
}

#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "PascalCase")]
struct PascalCaseNaming {
    user_name: String,        // → "UserName"
    email_address: String,    // → "EmailAddress"
    is_admin: bool,          // → "IsAdmin"
}

#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct CamelCaseNaming {
    user_name: String,        // → "userName"
    email_address: String,    // → "emailAddress"
    is_admin: bool,          // → "isAdmin"
}
```

## Field-Level Attributes

Override naming and behavior for individual fields using field-level attributes.

### Field Renaming

Use `#[tnuctipun(rename = "custom_name")]` to override field names:

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct User {
    user_name: String,              // → "userName" (camelCase applied)
    
    #[tnuctipun(rename = "email")]
    email_address: String,          // → "email" (override)
    
    #[tnuctipun(rename = "_id")]
    id: String,                     // → "_id" (MongoDB convention)
    
    created_at: String,             // → "createdAt" (camelCase applied)
}
```

### Skipping Fields

Use `#[tnuctipun(skip)]` to exclude fields from witness generation:

```rust
#[derive(FieldWitnesses)]
struct User {
    pub name: String,               // → Included
    pub email: String,              // → Included
    
    #[tnuctipun(skip)]
    internal_id: String,            // → Skipped (no witness generated)
    
    #[tnuctipun(skip)]
    temp_data: Vec<u8>,             // → Skipped
}

// Usage: internal_id and temp_data are not available in queries
let mut filter = empty::<User>();
filter.eq::<user_fields::Name, _>("John".to_string());      // ✅ Available
filter.eq::<user_fields::InternalId, _>("123".to_string()); // ❌ Compile error - skipped field
```

### Combined Attributes

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct ComplexUser {
    user_name: String,                          // → "userName"
    
    #[tnuctipun(rename = "email")]
    email_address: String,                      // → "email"
    
    #[tnuctipun(skip)]
    internal_hash: String,                      // → Skipped
    
    #[tnunctipun(rename = "isActive")]
    is_user_active: bool,                       // → "isActive"
    
    created_timestamp: chrono::DateTime<chrono::Utc>,  // → "createdTimestamp"
}
```

## Private Field Handling

Control whether private fields are included in field witness generation.

### Default Behavior (Public Fields Only)

By default, only `pub` fields generate witnesses:

```rust
#[derive(FieldWitnesses)]
struct User {
    pub name: String,               // ✅ Witness generated
    pub email: String,              // ✅ Witness generated
    internal_id: String,            // ❌ No witness (private)
    private_data: Vec<u8>,          // ❌ No witness (private)
}

// Only public fields are available
let mut filter = empty::<User>();
filter.eq::<user_fields::Name, _>("John".to_string());      // ✅ Works
filter.eq::<user_fields::InternalId, _>("123".to_string()); // ❌ Compile error
```

### Including Private Fields

Use `#[tnuctipun(include_private = true)]` to include private fields:

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(include_private = true)]
struct User {
    pub name: String,               // ✅ Witness generated
    pub email: String,              // ✅ Witness generated
    internal_id: String,            // ✅ Witness generated (private but included)
    private_data: Vec<u8>,          // ✅ Witness generated (private but included)
}

// All fields are available
let mut filter = empty::<User>();
filter.eq::<user_fields::Name, _>("John".to_string());      // ✅ Works
filter.eq::<user_fields::InternalId, _>("123".to_string()); // ✅ Works now
```

### Explicit Private Field Exclusion

Use `#[tnuctipun(include_private = false)]` to explicitly exclude private fields:

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(include_private = false)]  // Explicit (same as default)
struct User {
    pub name: String,               // ✅ Witness generated
    internal_id: String,            // ❌ No witness (explicitly excluded)
}
```

### Mixed Visibility with Field-Level Control

```rust
#[derive(FieldWitnesses)]
#[tnuctipun(include_private = true)]
struct User {
    pub name: String,               // ✅ Included (public)
    pub email: String,              // ✅ Included (public)
    
    internal_id: String,            // ✅ Included (private but include_private=true)
    
    #[tnuctipun(skip)]
    secret_key: String,             // ❌ Skipped (explicitly skipped)
}
```

## Advanced Usage

### Multiple Structs with Field Conflicts

Tnuctipun automatically handles field name conflicts between different structs:

```rust
#[derive(FieldWitnesses)]
struct User {
    pub name: String,               // → user_fields::Name
    pub id: String,                 // → user_fields::Id
}

#[derive(FieldWitnesses)]
struct Product {
    pub name: String,               // → product_fields::Name (no conflict)
    pub id: String,                 // → product_fields::Id (no conflict)
}

// No naming conflicts - each struct gets its own module
let mut user_filter = empty::<User>();
user_filter.eq::<user_fields::Name, _>("John".to_string());

let mut product_filter = empty::<Product>();
product_filter.eq::<product_fields::Name, _>("Widget".to_string());
```

### Generic Structs

Field witnesses work with generic structs:

```rust
#[derive(FieldWitnesses, MongoComparable)]
struct Container<T> {
    pub id: String,
    pub data: T,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Usage with specific types
type UserContainer = Container<User>;
type ProductContainer = Container<Product>;

let mut filter = empty::<UserContainer>();
filter.eq::<container_fields::Id, _>("container123".to_string());
```

### Nested Structs

Each struct needs its own derives, even when nested:

```rust
#[derive(FieldWitnesses, MongoComparable)]
struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
}

#[derive(FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub address: Address,  // Nested struct
}

// Both structs get their own field witnesses
let mut user_filter = empty::<User>();
user_filter.eq::<user_fields::Name, _>("John".to_string());

let mut address_filter = empty::<Address>();
address_filter.eq::<address_fields::City, _>("New York".to_string());
```

## Troubleshooting

### Common Compilation Errors

#### Field Does Not Exist

```rust
// Error: no field `InvalidField` in module `user_fields`
filter.eq::<user_fields::InvalidField, _>("value");
```

**Solution**: Check field name spelling and ensure the field exists in your struct.

#### Private Field Access

```rust
#[derive(FieldWitnesses)]
struct User {
    name: String,  // Private field
}

// Error: field witness not generated for private field
filter.eq::<user_fields::Name, _>("John");
```

**Solution**: Either make the field `pub` or use `#[tnuctipun(include_private = true)]`.

#### Missing Derives

```rust
// Error: trait bound not satisfied
#[derive(FieldWitnesses)]  // Missing MongoComparable
struct User {
    pub name: String,
}

filter.eq::<user_fields::Name, _>("John");  // Fails without MongoComparable
```

**Solution**: Add the `MongoComparable` derive.

#### Type Mismatch

```rust
// Error: expected `String`, found `&str`
filter.eq::<user_fields::Name, _>("John");  // &str instead of String
```

**Solution**: Match the exact field type or use conversion:
```rust
filter.eq::<user_fields::Name, _>("John".to_string());  // Correct
```

### Debugging Generated Code

To see what code the macros generate, use `cargo expand`:

```bash
# Install cargo-expand
cargo install cargo-expand

# View expanded code for a specific module
cargo expand --lib path::to::module
```

### Field Naming Conflicts

If you encounter field naming conflicts, check:

1. **Module isolation**: Each struct gets its own `*_fields` module
2. **Naming strategy**: Ensure consistent naming strategies
3. **Manual renames**: Use `#[tnuctipun(rename = "...")]` for specific cases

## Performance Considerations

- **Zero Runtime Cost**: All validation happens at compile time
- **Code Generation**: Macros generate minimal, optimized code
- **Memory Usage**: Field witnesses are zero-sized types (ZSTs)
- **Compilation Time**: Macro expansion adds minimal compilation overhead

## Best Practices

1. **Consistent Naming**: Use consistent field naming strategies across your project
2. **Public Fields**: Prefer `pub` fields for queryable data
3. **Skip Sensitive**: Use `#[tnuctipun(skip)]` for sensitive or internal fields
4. **Document Schemas**: Document your field naming conventions
5. **Test Compilation**: Ensure all query code compiles with field changes

## Next Steps

- [**Advanced Topics**](06-advanced-topics.md) - Explore complex scenarios and best practices
- [**Getting Started**](02-getting-started.md) - Return to basics if needed
- [**API Documentation**](https://cchantep.github.io/tnuctipun/tnunctipun/) - Complete API reference
