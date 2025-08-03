use bson;

use crate::field_witnesses::{FieldName, HasField};
use crate::path::Path;

/// A builder for creating MongoDB projection documents with compile-time field verification.
///
/// `BasicProjectionBuilder` provides a fluent API for constructing MongoDB projection documents
/// while ensuring that only fields that exist on the target struct `T` can be projected.
/// This eliminates runtime errors caused by typos in field names.
///
/// This builder provides the basic/safe projection features using `includes()` and `excludes()`
/// methods. For additional projection capabilities with custom MongoDB expressions, see the
/// `ProjectionBuilder` trait which provides the `project()` method (requires manual field path
/// strings and is less type-safe).
///
/// # Type Parameters
///
/// * `T` - The target struct type that implements the necessary field witness traits
///
/// # Examples
///
/// ```rust
/// use nessus::{FieldWitnesses, projection::empty};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(FieldWitnesses, Serialize, Deserialize)]
/// struct User {
///     pub id: String,
///     pub name: String,
///     pub email: String,
///     pub age: i32,
/// }
///
/// let projection_doc = empty::<User>()
///     .includes::<user_fields::Name>()
///     .includes::<user_fields::Age>()
///     .excludes::<user_fields::Email>()
///     .build();
/// // Results in: { "name": 1, "age": 1, "email": 0 }
/// ```
///
/// # Field Path Construction
///
/// The builder automatically constructs proper MongoDB field paths by combining
/// any prefix (for nested projections) with the field name. For example:
/// - Without prefix: `"name"`
/// - With prefix `["user"]`: `"user.name"`  
/// - With nested prefix `["data", "profile"]`: `"data.profile.name"`
pub struct BasicProjectionBuilder<T> {
    prefix: Vec<String>,
    clauses: Vec<(String, bson::Bson)>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for BasicProjectionBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BasicProjectionBuilder<T> {
    /// Creates a new `BasicProjectionBuilder` instance.
    ///
    /// The builder starts with an empty prefix and no projection clauses.
    ///
    /// # Returns
    ///
    /// A new `BasicProjectionBuilder` instance ready for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::projection::BasicProjectionBuilder;
    ///
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    /// }
    ///
    /// let builder = BasicProjectionBuilder::<User>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            prefix: Vec::new(),
            clauses: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a fully qualified field path for the given field name marker type.
    /// Returns a fully qualified field path for the given field name marker type.
    ///
    /// This method constructs the complete dot-notation path for a field by combining
    /// any existing prefix (for nested document projections) with the field name.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Returns
    ///
    /// A `String` containing the fully qualified field path.
    ///
    /// # Examples
    ///
    /// - Without prefix: `"field_name"`
    /// - With prefix `["parent"]`: `"parent.field_name"`
    /// - With nested prefix `["root", "child"]`: `"root.child.field_name"`
    fn field_path<F: FieldName>(&self) -> String {
        if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        }
    }

    /// Projects a field with the specified inclusion/exclusion flag.
    ///
    /// This is a helper method that combines field path generation with clause addition.
    /// It's used internally by both `includes()` and `excludes()` methods.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Parameters
    ///
    /// * `includes` - `true` to include the field (value 1), `false` to exclude (value 0)
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    fn project_field<F: FieldName>(&mut self, includes: bool) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();
        let flag = if includes { 1 } else { 0 };

        self.clauses.push((path, flag.into()));

        self
    }

    /// Includes a field in the projection result.
    ///
    /// This method adds a field to the projection with a value of 1, indicating that
    /// the field should be included in the query results. This is equivalent to
    /// MongoDB's `{ field: 1 }` projection syntax.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Constraints
    ///
    /// * `T` must have the field `F` (enforced by `HasField<F>` trait bound)
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    ///     pub email: String,
    /// }
    ///
    /// let doc = empty::<User>()
    ///     .includes::<user_fields::Name>()
    ///     .includes::<user_fields::Email>()
    ///     .build();
    /// // Results in: { "name": 1, "email": 1 }
    /// ```
    pub fn includes<F: FieldName>(&mut self) -> &mut Self
    where
        T: HasField<F>,
    {
        self.project_field::<F>(true)
    }

    /// Excludes a field from the projection result.
    ///
    /// This method adds a field to the projection with a value of 0, indicating that
    /// the field should be excluded from the query results. This is equivalent to
    /// MongoDB's `{ field: 0 }` projection syntax.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Constraints
    ///
    /// * `T` must have the field `F` (enforced by `HasField<F>` trait bound)
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    ///     pub email: String,
    ///     pub password: String,
    /// }
    ///
    /// let doc = empty::<User>()
    ///     .excludes::<user_fields::Password>()
    ///     .build();
    /// // Results in: { "password": 0 }
    /// ```
    pub fn excludes<F: FieldName>(&mut self) -> &mut Self
    where
        T: HasField<F>,
    {
        self.project_field::<F>(false)
    }

    /// Performs nested field projection using a lookup function.
    ///
    /// This method enables projection on nested object fields by providing a way to
    /// navigate into nested structures while maintaining compile-time field verification.
    /// It's particularly useful for projecting fields within embedded documents or
    /// complex nested structures.
    ///
    /// # Parameters
    ///
    /// * `lookup` - A function that takes a `Path<F, T, T>` and returns a `Path<G, U, T>`,
    ///   defining how to navigate from the current context to the target nested field
    /// * `f` - A function that configures the projection on the nested structure
    ///
    /// # Type Parameters
    ///
    /// * `F` - The field name marker type for the starting field
    /// * `L` - The lookup function type
    /// * `G` - The field name marker type for the target nested field  
    /// * `U` - The type of the nested structure
    /// * `N` - The configuration function type
    ///
    /// # Constraints
    ///
    /// * `T` must have the field `F` (enforced by `HasField<F>` trait bound)
    /// * `U` must have the field `G` (enforced by `HasField<G>` trait bound)
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct Address {
    ///     pub street: String,
    ///     pub city: String,
    /// }
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    ///     pub address: Address,
    /// }
    ///
    /// let doc = empty::<User>()
    ///     .with_lookup::<user_fields::Address, _, address_fields::City, Address, _>(
    ///         |path| path.field::<address_fields::City>(),
    ///         |nested| { nested.includes::<address_fields::City>(); }
    ///     )
    ///     .build();
    /// // Results in: { "address.city": 1 }
    /// ```
    pub fn with_lookup<F: FieldName, L, G: FieldName, U: HasField<G>, N>(
        &mut self,
        lookup: L,
        f: N,
    ) -> &mut Self
    where
        T: HasField<F>,
        L: FnOnce(&Path<F, T, T>) -> Path<G, U, T>,
        N: FnOnce(&mut BasicProjectionBuilder<U>),
    {
        // Create a base field path for the lookup
        let base_field: Path<F, T, T> = Path {
            prefix: self.prefix.clone(),
            _marker: std::marker::PhantomData,
        };

        // Resolve the field path using the provided lookup function
        let resolved_field = lookup(&base_field);

        // Create a new BasicProjectionBuilder for the nested field
        let mut nested_builder = BasicProjectionBuilder::<U> {
            prefix: resolved_field.prefix.clone(),
            clauses: vec![],
            _marker: std::marker::PhantomData,
        };

        f(&mut nested_builder);

        // Add the nested clauses individually to the main builder
        self.clauses.extend(nested_builder.clauses);

        self
    }

    /// Performs projection on a field using an identity lookup (convenience method).
    ///
    /// This method is a convenience wrapper around `with_lookup` that uses an identity function
    /// for the lookup, making it easier to apply projections in the context of a specific field
    /// without needing to specify the lookup logic.
    ///
    /// This is particularly useful when you want to group projection operations logically
    /// by field context, even though the operations might affect multiple fields.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `N` - The configuration function type that defines projections
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes a mutable reference to `BasicProjectionBuilder<T>` and configures projections
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub id: String,
    ///     pub name: String,
    ///     pub email: String,
    /// }
    ///
    /// // Apply multiple projections in the same field context
    /// let projection_doc = empty::<User>()
    ///     .with_field::<user_fields::Name, _>(|nested| {
    ///         nested
    ///             .includes::<user_fields::Name>()
    ///             .excludes::<user_fields::Email>();
    ///     })
    ///     .build();
    /// // Results in: { "name": 1, "email": 0 }
    /// ```
    ///
    /// # Comparison with Direct Method Calls
    ///
    /// The following two approaches are equivalent:
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    /// }
    ///
    /// // Using with_field
    /// let projection_doc1 = empty::<User>()
    ///     .with_field::<user_fields::Name, _>(|nested| {
    ///         nested.includes::<user_fields::Name>();
    ///     })
    ///     .build();
    ///
    /// // Using direct method calls
    /// let projection_doc2 = empty::<User>()
    ///     .includes::<user_fields::Name>()
    ///     .build();
    ///
    /// // Both produce the same result: { "name": 1 }
    /// assert_eq!(projection_doc1, projection_doc2);
    /// ```
    pub fn with_field<F: FieldName, N>(&mut self, f: N) -> &mut Self
    where
        T: HasField<F>,
        N: FnOnce(&mut BasicProjectionBuilder<T>),
    {
        self.with_lookup::<F, _, F, T, _>(
            |path| Path {
                prefix: path.prefix.clone(),
                _marker: std::marker::PhantomData,
            },
            f,
        )
    }

    /// Builds the final MongoDB projection document.
    ///
    /// This method consumes the builder and produces a `bson::Document` that can be
    /// used directly with MongoDB queries. All accumulated projection clauses are
    /// combined into a single document.
    ///
    /// Note that this method takes ownership of the builder (`self`), so it cannot
    /// be called multiple times on the same builder instance.
    ///
    /// # Returns
    ///
    /// A `bson::Document` containing all the projection clauses that were added
    /// to this builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    /// use bson;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    ///     pub email: String,
    ///     pub age: i32,
    /// }
    ///
    /// let projection_doc = empty::<User>()
    ///     .includes::<user_fields::Name>()
    ///     .excludes::<user_fields::Email>()
    ///     .build();
    ///
    /// // Use with MongoDB collection
    /// // collection.find().projection(projection_doc).await?;
    /// ```
    ///
    /// # Behavior with Duplicate Fields
    ///
    /// If the same field is projected multiple times with different values,
    /// the last value will be used in the final document:
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, projection::empty};
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    ///     pub email: String,
    /// }
    ///
    /// let projection_doc = empty::<User>()
    ///     .includes::<user_fields::Name>()  // { "name": 1 }
    ///     .excludes::<user_fields::Name>()  // { "name": 0 } - this wins
    ///     .build();
    /// // Results in: { "name": 0 }
    /// ```
    pub fn build(&mut self) -> bson::Document {
        let mut doc = bson::Document::new();

        for (field, value) in &self.clauses {
            doc.insert(field.clone(), value.clone());
        }

        doc
    }
}

/// Creates a new empty `BasicProjectionBuilder` instance.
///
/// This function provides a convenient way to start a fluent chain of projection operations
/// without needing to explicitly call `BasicProjectionBuilder::new()` and assign it to a mutable variable.
///
/// # Type Parameters
///
/// * `T` - The target struct type that implements the necessary field witness traits
///
/// # Returns
///
/// A new `BasicProjectionBuilder<T>` instance ready for method chaining.
///
/// # Examples
///
/// ```rust
/// use nessus::{FieldWitnesses, projection::empty};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(FieldWitnesses, Serialize, Deserialize)]
/// struct User {
///     pub id: String,
///     pub name: String,
///     pub email: String,
///     pub age: i32,
/// }
///
/// // Use method chaining (recommended)
/// let projection_doc = empty::<User>()
///     .includes::<user_fields::Name>()
///     .excludes::<user_fields::Email>()
///     .build();
/// // Results in: { "name": 1, "email": 0 }
/// ```
pub fn empty<T>() -> BasicProjectionBuilder<T> {
    BasicProjectionBuilder::new()
}

/// Extension trait that adds advanced projection capabilities to projection builders.
///
/// This trait extends the basic projection functionality with the ability to specify
/// custom MongoDB projection expressions. Unlike the type-safe `includes()` and `excludes()`
/// methods, the `project()` method requires manual field path strings and allows for
/// complex MongoDB expressions.
///
/// # Type Parameters
///
/// * `T` - The target struct type for the projection
///
/// # Safety Note
///
/// The `project()` method is less type-safe than `includes()`/`excludes()` as it accepts
/// arbitrary field paths as strings. Typos in field names will not be caught at compile time.
///
/// # Examples
///
/// ```rust
/// use nessus::{projection::{empty, ProjectionBuilder}};
/// use bson::doc;
///
/// // Using custom MongoDB expressions
/// let mut builder = empty::<()>();
/// builder.project("name".to_string(), doc! { "$toUpper": "$name" }.into());
/// builder.project("computed".to_string(), doc! { "$add": ["$a", "$b"] }.into());
/// let projection_doc = builder.build();
/// ```
pub trait ProjectionBuilder<T>: Sized {
    /// Projects a field using a custom MongoDB expression.
    ///
    /// This method allows you to specify complex MongoDB projection expressions
    /// for a field, such as computed fields, conditional projections, or array
    /// manipulations. The field path is specified as a string, so be careful
    /// with spelling and ensure the field exists in your MongoDB documents.
    ///
    /// # Parameters
    ///
    /// * `path` - The field path as a string (e.g., "name", "address.city")
    /// * `expr` - A BSON expression defining how to project this field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::projection::{empty, ProjectionBuilder};
    /// use bson::{doc, Bson};
    ///
    /// let mut builder = empty::<()>();
    ///
    /// // Convert field to uppercase
    /// builder.project("name".to_string(), doc! { "$toUpper": "$name" }.into());
    ///
    /// // Slice an array field
    /// builder.project("items".to_string(), doc! { "$slice": 5 }.into());
    ///
    /// // Conditional projection
    /// builder.project(
    ///     "display_name".to_string(),
    ///     doc! {
    ///         "$cond": {
    ///             "if": { "$ne": ["$name", null] },
    ///             "then": "$name",
    ///             "else": "Anonymous"
    ///         }
    ///     }.into()
    /// );
    ///
    /// let projection_doc = builder.build();
    /// ```
    fn project(&mut self, path: String, expr: bson::Bson) -> &mut Self;
}

impl<T> ProjectionBuilder<T> for BasicProjectionBuilder<T> {
    /// Implementation of custom projection for `BasicProjectionBuilder`.
    ///
    /// This adds a custom projection expression to the builder's clause list.
    /// The expression can be any valid MongoDB projection expression.
    ///
    /// # Parameters
    ///
    /// * `path` - The field path for the projection
    /// * `expr` - The BSON projection expression
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    fn project(&mut self, path: String, expr: bson::Bson) -> &mut Self {
        self.clauses.push((path, expr));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_witnesses::FieldName;

    // Test field marker types
    struct Name;
    impl FieldName for Name {
        fn field_name() -> &'static str {
            "name"
        }
    }

    struct Id;
    impl FieldName for Id {
        fn field_name() -> &'static str {
            "id"
        }
    }

    struct Age;
    impl FieldName for Age {
        fn field_name() -> &'static str {
            "age"
        }
    }

    struct Email;
    impl FieldName for Email {
        fn field_name() -> &'static str {
            "email"
        }
    }

    struct TestStruct;

    #[test]
    fn test_field_path_empty_prefix() {
        // Test field_path with empty prefix - should return just the field name
        let builder = empty::<TestStruct>();

        let path = builder.field_path::<Name>();

        assert_eq!(path, "name");

        let path = builder.field_path::<Id>();

        assert_eq!(path, "id");

        let path = builder.field_path::<Age>();

        assert_eq!(path, "age");

        let path = builder.field_path::<Email>();

        assert_eq!(path, "email");
    }

    #[test]
    fn test_field_path_single_prefix() {
        // Test field_path with a single prefix element
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["user".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "user.name");

        let path = builder.field_path::<Id>();

        assert_eq!(path, "user.id");
    }

    #[test]
    fn test_field_path_multiple_prefixes() {
        // Test field_path with multiple prefix elements
        let mut builder = empty::<TestStruct>();
        builder.prefix = vec!["profile".to_string(), "address".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "profile.address.name");

        let path = builder.field_path::<Age>();

        assert_eq!(path, "profile.address.age");
    }

    #[test]
    fn test_field_path_deeply_nested_prefix() {
        // Test field_path with deeply nested prefix
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec![
            "collection".to_string(),
            "documents".to_string(),
            "user_data".to_string(),
            "profile".to_string(),
        ];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "collection.documents.user_data.profile.name");
    }

    #[test]
    fn test_field_path_consistency_across_multiple_calls() {
        // Test that field_path returns consistent results across multiple calls
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["test".to_string()];

        let path1 = builder.field_path::<Name>();
        let path2 = builder.field_path::<Name>();
        let path3 = builder.field_path::<Name>();

        assert_eq!(path1, path2);
        assert_eq!(path2, path3);
        assert_eq!(path1, "test.name");
    }

    #[test]
    fn test_field_path_special_characters_in_prefix() {
        // Test field_path with prefixes containing special characters
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["test-prefix".to_string(), "sub_field".to_string()];

        let path = builder.field_path::<Email>();

        assert_eq!(path, "test-prefix.sub_field.email");
    }

    #[test]
    fn test_field_path_empty_string_prefix() {
        // Test field_path with empty string as prefix element
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, ".name");
    }

    #[test]
    fn test_field_path_mixed_prefix_types() {
        // Test field_path with mixed types of prefix strings
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec![
            "root".to_string(),
            "nested_object".to_string(),
            "array_element".to_string(),
        ];

        let path = builder.field_path::<Id>();

        assert_eq!(path, "root.nested_object.array_element.id");

        let path = builder.field_path::<Email>();

        assert_eq!(path, "root.nested_object.array_element.email");
    }

    #[test]
    fn test_field_path_with_numeric_string_prefix() {
        // Test field_path with numeric strings as prefixes (like array indices)
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["users".to_string(), "0".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "users.0.name");
    }

    #[test]
    fn test_field_path_different_field_types() {
        // Test field_path with different field marker types to ensure consistency
        let mut builder = empty::<TestStruct>();

        builder.prefix = vec!["common".to_string()];

        // Test that different field types produce different paths correctly
        let name_path = builder.field_path::<Name>();
        let id_path = builder.field_path::<Id>();
        let age_path = builder.field_path::<Age>();
        let email_path = builder.field_path::<Email>();

        assert_eq!(name_path, "common.name");
        assert_eq!(id_path, "common.id");
        assert_eq!(age_path, "common.age");
        assert_eq!(email_path, "common.email");

        // Ensure they're all unique
        let paths = [name_path, id_path, age_path, email_path];
        let unique_paths: std::collections::HashSet<_> = paths.iter().collect();

        assert_eq!(unique_paths.len(), 4);
    }
}
