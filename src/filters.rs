use mongodb::bson;

use crate::field_witnesses::{FieldName, HasField, NonEmptyStruct};
use crate::mongo_comparable::MongoComparable;

/// A builder for constructing MongoDB filters with type safety.
///
/// This builder provides a fluent interface for creating complex MongoDB queries
/// while ensuring at compile time that fields exist and have compatible types.
///
/// The builder can be used to chain multiple filter conditions together,
/// and provides methods to combine them using MongoDB's `$and` semantics.
///
/// # Type Parameters
///
/// * `T` - The struct type that this filter builder operates on (e.g., `User`, `Product`)
pub struct FilterBuilder<T> {
    prefix: Vec<String>,
    clauses: Vec<bson::Document>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> FilterBuilder<T> {
    /// Creates a new empty FilterBuilder instance.
    ///
    /// Notes: Prefer using the `new` method directly instead of the `empty` function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::FilterBuilder;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User { Name: String }
    ///
    /// let builder = FilterBuilder::<User>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            prefix: Vec::new(),
            clauses: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a fully qualified field path for the given field name marker type.
    fn field_path<F: FieldName>(&self) -> String {
        if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        }
    }

    /// Creates a type-safe equality filter (`$eq`) that checks at compile time if the field exists
    /// and has the correct type or a compatible type.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::Name`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use nessus::filters::empty;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { Name: String, Age: i32 }
    ///
    /// // Using builder pattern with efficient chaining:
    /// empty::<User>().eq::<user_fields::Name, _>("John".to_string());
    /// ```
    pub fn eq<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses.push(bson::doc! { path: value.into() });

        self
    }

    /// Returns the current filter clauses as a vector of BSON documents.
    pub fn clauses(&self) -> &Vec<bson::Document> {
        &self.clauses
    }

    /// Creates a type-safe version of MongoDB's greater than (`$gt`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { Price: f64 }
    ///
    /// // Filter for products with price > 500
    /// empty::<Product>().gt::<product_fields::Price, _>(500.0);
    /// // Resulting BSON: { "Price": { "$gt": 500.0 } }
    /// ```
    pub fn gt<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$gt": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's less (`$lt`) than filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Stock`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { Stock: i32 }
    ///
    /// // Filter for products with stock < 10
    /// empty::<Product>().lt::<product_fields::Stock, _>(10);
    /// // Resulting BSON: { "Stock": { "$lt": 10 } }
    /// ```
    pub fn lt<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$lt": value.into() } });

        self
    }

    /// Type-safe version of "in" operator filter
    ///
    /// Creates a MongoDB filter that matches any of the values in the provided array.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::Age`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { Age: i32 }
    ///
    /// // Filter for users with age in [20, 30, 40]
    /// empty::<User>().r#in::<user_fields::Age, _>(vec![20, 30, 40]);
    /// // Resulting BSON: { "Age": { "$in": [20, 30, 40] } }
    /// ```
    pub fn r#in<F, V>(&mut self, values: Vec<V>) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.clauses
            .push(bson::doc! { path: { "$in": bson_values } });

        self
    }

    /// Creates a type-safe version of MongoDB's "not equal" (`$ne`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `order_fields::Status`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Order { Status: String }
    ///
    /// // Filter for orders with status not equal to "Delivered"
    /// empty::<Order>().ne::<order_fields::Status, _>("Delivered".to_string());
    /// // Resulting BSON: { "Status": { "$ne": "Delivered" } }
    /// ```
    pub fn ne<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$ne": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's "greater than or equal" (`$gte`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Rating`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { Rating: f64 }
    ///
    /// // Filter for products with rating >= 4.5
    /// empty::<Product>().gte::<product_fields::Rating, _>(4.5);
    /// // Resulting BSON: { "Rating": { "$gte": 4.5 } }
    /// ```
    pub fn gte<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$gte": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's "less than or equal" (`$lte`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { Price: f64 }
    ///
    /// // Filter for products with price <= 100.0
    /// empty::<Product>().lte::<product_fields::Price, _>(100.0);
    /// // Resulting BSON: { "Price": { "$lte": 100.0 } }
    /// ```
    pub fn lte<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$lte": value.into() } });

        self
    }

    /// Creates a type-safe version of "exists" filter, that checks if a field exists in the document.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::OptionalField`)
    ///   (must implement `HasField<F>` to ensure compile-time field existence)
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     Name: String,
    ///     PhoneNumber: Option<String>
    /// }
    ///
    /// // Filter for users that have a phone number
    /// empty::<User>().exists::<user_fields::PhoneNumber>(true);
    /// // Resulting BSON: { "PhoneNumber": { "$exists": true } }
    ///
    /// // Filter for users without a phone number
    /// empty::<User>().exists::<user_fields::PhoneNumber>(false);
    /// // Resulting BSON: { "PhoneNumber": { "$exists": false } }
    /// ```
    pub fn exists<F>(&mut self, exists: bool) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$exists": exists } });

        self
    }

    /// Create a type-safe version of MongoDB's "not in" (`$nin`) operator filter,
    /// that matches values NOT in the provided array.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Category`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { Category: String }
    ///
    /// // Filter for products NOT in the categories "Clothing", "Shoes", or "Accessories"
    /// empty::<Product>().nin::<product_fields::Category, _>(vec![
    ///     "Clothing".to_string(),
    ///     "Shoes".to_string(),
    ///     "Accessories".to_string()
    /// ]);
    /// // Resulting BSON: { "Category": { "$nin": ["Clothing", "Shoes", "Accessories"] } }
    /// ```
    pub fn nin<F, V>(&mut self, values: Vec<V>) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.clauses
            .push(bson::doc! { path: { "$nin": bson_values } });

        self
    }

    /// Creates filters for nested fields within documents by allowing you to build
    /// filters on a nested structure. This method handles the proper field path
    /// construction for nested MongoDB queries.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type for the nested field (e.g., `user_fields::Address`)
    /// * `V` - The type of the nested field value (must be a struct type)
    /// * `N` - The closure that builds filters on the nested FilterBuilder
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Address {
    ///     Street: String,
    ///     City: String,
    ///     ZipCode: String,
    /// }
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     Name: String,
    ///     HomeAddress: Address,
    /// }
    ///
    /// // Filter for users with home address in "New York" city
    /// let mut builder = empty::<User>();
    ///
    /// builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
    ///     nested.eq::<address_fields::City, _>("New York".to_string())
    /// });
    /// // Resulting BSON: { "HomeAddress.City": "New York" }
    ///
    /// // Chaining multiple nested filters
    /// builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
    ///     nested.eq::<address_fields::City, _>("San Francisco".to_string())
    ///           .eq::<address_fields::ZipCode, _>("94102".to_string())
    /// });
    /// // Adds: { "HomeAddress.City": "San Francisco" }, { "HomeAddress.ZipCode": "94102" }
    /// ```
    pub fn with_nested<F, V, N>(&mut self, f: N) -> &mut Self
    where
        F: FieldName,
        T: HasField<F, Value = V>,
        V: NonEmptyStruct,
        N: FnOnce(&mut FilterBuilder<V>) -> &mut FilterBuilder<V>,
    {
        // Prepare prefix for the nested builder,
        // by copying the current prefix and appending the field name
        let mut nested_prefix = self.prefix.clone();

        nested_prefix.push(F::field_name().to_string());

        // Create a new FilterBuilder for the nested field
        let mut nested_builder = FilterBuilder::<V> {
            prefix: nested_prefix,
            clauses: vec![],
            _marker: std::marker::PhantomData,
        };

        f(&mut nested_builder);

        // Add the nested clauses individually to the main builder
        self.clauses.extend(nested_builder.clauses);

        self
    }

    /// Create a type-safe version of MongoDB's "$or" operator,
    /// where each clause is generated by applying a closure to each item in the input iterable.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Category`)
    /// * `V` - An iterable type containing values to process
    /// * `N` - A closure that takes a FilterBuilder and an item from V, and returns the FilterBuilder
    ///
    /// # Arguments
    ///
    /// * `input` - An iterable collection of values to process
    /// * `f` - A closure that builds filter conditions for each value in the input
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     name: String,
    ///     category: String,
    ///     price: f64,
    /// }
    ///
    /// // Filter for products that match any of the given names
    /// let mut builder = empty::<Product>();
    /// let names = vec!["Laptop", "Smartphone", "Tablet"];
    ///
    /// builder.or::<product_fields::Name, _, _>(names, |filter, name| {
    ///     filter.eq::<product_fields::Name, _>(name.to_string())
    /// });
    ///
    /// // Resulting BSON:
    /// // { "$or": [
    /// //     { "name": "Laptop" },
    /// //     { "name": "Smartphone" },
    /// //     { "name": "Tablet" }
    /// // ]}
    /// ```
    ///
    /// # Complex Example with Multiple Conditions
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     name: String,
    ///     category: String,
    ///     price: f64,
    /// }
    ///
    /// // Filter for products in specific price ranges
    /// let mut builder = empty::<Product>();
    /// let price_ranges = vec![(0.0, 100.0), (500.0, 1000.0), (2000.0, 5000.0)];
    ///
    /// builder.or::<product_fields::Price, _, _>(price_ranges, |filter, (min, max)| {
    ///     filter.gte::<product_fields::Price, _>(min)
    ///           .lte::<product_fields::Price, _>(max)
    /// });
    ///
    /// // Resulting BSON (note: multiple clauses from each iteration are flattened):
    /// // { "$or": [
    /// //     { "price": { "$gte": 0.0 } },
    /// //     { "price": { "$lte": 100.0 } },
    /// //     { "price": { "$gte": 500.0 } },
    /// //     { "price": { "$lte": 1000.0 } },
    /// //     { "price": { "$gte": 2000.0 } },
    /// //     { "price": { "$lte": 5000.0 } }
    /// // ]}
    /// ```
    pub fn or<F, V: IntoIterator, N>(&mut self, input: V, f: N) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
        N: Fn(&mut FilterBuilder<T>, V::Item) -> &mut FilterBuilder<T>,
    {
        let mut nested = empty::<T>();
        let mut or_clauses: Vec<bson::Document> = vec![];

        for value in input {
            f(&mut nested, value);

            match nested.clauses.len() {
                0 => continue, // Skip empty nested clauses
                1 => or_clauses.push(nested.clauses.clone().into_iter().next().unwrap()),
                _ => or_clauses.extend(nested.clauses.clone()),
            }

            nested.clauses.clear(); // Clear for next iteration
        }

        self.clauses.push(bson::doc! { "$or": or_clauses });

        self
    }

    /// Create a type-safe version of MongoDB's "$not" operator.
    ///
    /// Such MongoDB filter negates operations on a specific field.
    /// This method uses an `OperationBuilder` to construct the operations to be negated.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `B` - A closure that takes an `OperationBuilder` and returns it with configured operations
    ///
    /// # Arguments
    /// * `f` - A closure that builds the operations to be negated using the `OperationBuilder`
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     name: String,
    ///     price: f64,
    ///     category: String,
    /// }
    ///
    /// // Filter for products where the name is NOT "Smartphone"
    /// let mut builder = empty::<Product>();
    ///
    /// builder.not::<product_fields::Name, _>(|op| {
    ///     op.eq("Smartphone".to_string())
    /// });
    /// // Resulting BSON: { "name": { "$not": { "name": "Smartphone" } } }
    ///
    /// // Filter for products where the price is NOT equal to 500.0
    /// let mut builder = empty::<Product>();
    ///
    /// builder.not::<product_fields::Price, _>(|op| {
    ///     op.eq(500.0)
    /// });
    /// // Resulting BSON: { "price": { "$not": { "price": 500.0 } } }
    /// ```
    ///
    /// # MongoDB Behavior
    ///
    /// The `$not` operator in MongoDB performs logical NOT operation on the specified expression.
    /// It can be used to negate the result of any MongoDB expression, including:
    ///
    /// - Equality checks
    /// - Range queries  
    /// - Pattern matching
    /// - Other conditional expressions
    ///
    /// Note that `$not` affects the semantics of the query and can behave differently than
    /// using `$ne` (not equal) for simple equality checks, especially with missing fields.
    pub fn not<F, B>(&mut self, f: B) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
        B: FnOnce(OperationBuilder<F, T>) -> OperationBuilder<F, T>,
    {
        let op_builder = OperationBuilder::new();
        let prepared_ops = f(op_builder).build();
        let bson_path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { bson_path: bson::doc! { "$not": prepared_ops } });

        self
    }

    /// Combines all clauses into a single BSON document,
    /// according to the MongoDB `$and` semantics.
    ///
    /// - If no clauses exist, returns an empty document `{}`
    /// - If only one clause exists, returns that clause directly
    /// - If multiple clauses exist, wraps them in a `$and` array
    ///
    /// This method borrows the builder, allowing you to continue using it afterwards.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::empty;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     name: String,
    ///     age: i32,
    ///     email: String,
    ///     is_active: bool,
    ///     score: f64,
    /// }
    ///
    /// // Use the generated field witnesses from the user_fields module
    /// use user_fields::{Name, Age, Email};
    ///
    /// let mut builder = empty::<User>();
    /// builder.eq::<Name, _>("John Doe".to_string())
    ///        .gt::<Age, _>(18)
    ///        .exists::<Email>(true);
    /// let filter = builder.and();
    /// // builder can still be used here
    /// ```
    ///
    /// Resulting BSON:
    /// ```text
    /// { "$and": [{ "name": "John Doe" }, { "age": { "$gt": 18 } }, { "email": { "$exists": true } }] }
    /// ```
    pub fn and(&self) -> bson::Document {
        if self.clauses.is_empty() {
            bson::doc! {}
        } else if self.clauses.len() == 1 {
            self.clauses[0].clone()
        } else {
            bson::doc! { "$and": self.clauses.clone() }
        }
    }
}

impl<T> Default for FilterBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts a FilterBuilder into a BSON document using MongoDB's `$and` semantics.
///
/// This implementation allows FilterBuilder to be automatically converted to a BSON document
/// in contexts where `Into<bson::Document>` is expected, providing a convenient way to use
/// the builder directly with MongoDB operations.
///
/// The conversion follows the same logic as the `and()` method:
/// - If no clauses exist, returns an empty document `{}`
/// - If only one clause exists, returns that clause directly
/// - If multiple clauses exist, wraps them in a `$and` array
///
/// # Example
///
/// ```rust
/// use nessus::filters::empty;
/// use nessus::{FieldWitnesses, MongoComparable};
/// use serde::{Serialize, Deserialize};
/// use mongodb::bson;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
/// struct User {
///     name: String,
///     age: i32,
/// }
///
/// use user_fields::{Name, Age};
///
/// // The builder can be automatically converted to bson::Document
/// let mut builder = empty::<User>();
/// builder.eq::<Name, _>("John".to_string())
///        .gt::<Age, _>(18);
///
/// // Automatic conversion when expected type is bson::Document
/// let filter: bson::Document = builder.into();
///
/// // Or use directly in function calls expecting Into<bson::Document>
/// fn process_filter(filter: impl Into<bson::Document>) {
///     let doc = filter.into();
///     // ... use doc for MongoDB query
/// }
///
/// let mut builder2 = empty::<User>();
/// builder2.eq::<Name, _>("Alice".to_string());
/// process_filter(builder2); // Automatic conversion
/// ```
impl<T> From<FilterBuilder<T>> for bson::Document {
    fn from(val: FilterBuilder<T>) -> Self {
        val.and()
    }
}

/// Creates an empty FilterBuilder instance.
///
/// This is a convenience function that creates a new FilterBuilder.
/// It's equivalent to `FilterBuilder::<T>::new()` but with less typing.
///
/// # Example
///
/// ```rust
/// use nessus::filters::empty;
/// use nessus::{FieldWitnesses, MongoComparable};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
/// struct User { Name: String }
///
/// // Create and use a filter builder in one chain
/// empty::<User>().eq::<user_fields::Name, _>("John".to_string());
/// ```
pub fn empty<T>() -> FilterBuilder<T> {
    FilterBuilder::new()
}

// ---

/// A builder for operation-specific filters.
///
/// # Type Parameters
///
/// * `F` - The field name marker type that this operation builder targets
/// * `T` - The struct type that contains the field `F`
pub struct OperationBuilder<F: FieldName, T: HasField<F>> {
    ops: Vec<(String, bson::Bson)>,
    _marker: std::marker::PhantomData<(F, T)>,
}

impl<F, T> Default for OperationBuilder<F, T>
where
    F: FieldName,
    T: HasField<F>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F, T> OperationBuilder<F, T>
where
    F: FieldName,
    T: HasField<F>,
{
    /// Creates a new OperationBuilder instance.
    ///
    /// # Arguments
    ///
    /// * `build` - A closure that takes a BSON document and returns a FilterBuilder for the target struct
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    // ---

    /// Type-safe equality operation for the OperationBuilder.
    ///
    /// Adds an equality operation to the current operation builder, which can later
    /// be built into a FilterBuilder with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for equality
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::{OperationBuilder, empty};
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { name: String }
    ///
    /// // Create an operation builder and add an equality operation
    /// let op_builder = OperationBuilder::<user_fields::Name, User>::new();
    /// let final_builder = op_builder.eq("John Doe".to_string());
    /// let filter_doc = final_builder.build();
    /// ```
    pub fn eq<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        // For equality, we store it with the $eq operator
        self.ops.push(("$eq".to_string(), value.into()));

        self
    }

    /// Type-safe greater than operation for the OperationBuilder.
    ///
    /// Adds a greater than operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for greater than comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { price: f64 }
    ///
    /// // Create an operation builder and add a greater than operation
    /// let op_builder = OperationBuilder::<product_fields::Price, Product>::new();
    /// let final_builder = op_builder.gt(100.0);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "price": { "$gt": 100.0 } }
    /// ```
    pub fn gt<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$gt".to_string(), value.into()));

        self
    }

    /// Type-safe greater than or equal operation for the OperationBuilder.
    ///
    /// Adds a greater than or equal operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for greater than or equal comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { rating: f64 }
    ///
    /// // Create an operation builder and add a greater than or equal operation
    /// let op_builder = OperationBuilder::<product_fields::Rating, Product>::new();
    /// let final_builder = op_builder.gte(4.5);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "rating": { "$gte": 4.5 } }
    /// ```
    pub fn gte<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$gte".to_string(), value.into()));

        self
    }

    /// Type-safe less than operation for the OperationBuilder.
    ///
    /// Adds a less than operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for less than comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { stock: i32 }
    ///
    /// // Create an operation builder and add a less than operation
    /// let op_builder = OperationBuilder::<product_fields::Stock, Product>::new();
    /// let final_builder = op_builder.lt(10);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "stock": { "$lt": 10 } }
    /// ```
    pub fn lt<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$lt".to_string(), value.into()));

        self
    }

    /// Type-safe less than or equal operation for the OperationBuilder.
    ///
    /// Adds a less than or equal operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for less than or equal comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { price: f64 }
    ///
    /// // Create an operation builder and add a less than or equal operation
    /// let op_builder = OperationBuilder::<product_fields::Price, Product>::new();
    /// let final_builder = op_builder.lte(99.99);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "price": { "$lte": 99.99 } }
    /// ```
    pub fn lte<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$lte".to_string(), value.into()));

        self
    }

    /// Type-safe "in" operation for the OperationBuilder.
    ///
    /// Adds an "in" operation to the current operation builder, which matches any of the
    /// values in the provided array and can later be built into a MongoDB filter.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `values` - A vector of values to match against the field
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { age: i32 }
    ///
    /// // Create an operation builder and add an "in" operation
    /// let op_builder = OperationBuilder::<user_fields::Age, User>::new();
    /// let final_builder = op_builder.r#in(vec![20, 30, 40]);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "age": { "$in": [20, 30, 40] } }
    /// ```
    pub fn r#in<V>(mut self, values: Vec<V>) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.ops.push(("$in".to_string(), bson_values.into()));

        self
    }

    /// Type-safe "not in" operation for the OperationBuilder.
    ///
    /// Adds a "not in" operation to the current operation builder, which matches values
    /// NOT in the provided array and can later be built into a MongoDB filter.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `values` - A vector of values that the field should NOT match
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { category: String }
    ///
    /// // Create an operation builder and add a "not in" operation
    /// let op_builder = OperationBuilder::<product_fields::Category, Product>::new();
    /// let final_builder = op_builder.nin(vec![
    ///     "Clothing".to_string(),
    ///     "Shoes".to_string()
    /// ]);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "category": { "$nin": ["Clothing", "Shoes"] } }
    /// ```
    pub fn nin<V>(mut self, values: Vec<V>) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.ops.push(("$nin".to_string(), bson_values.into()));

        self
    }

    /// Type-safe "exists" operation for the OperationBuilder.
    ///
    /// Adds an "exists" operation to the current operation builder, which checks if a field
    /// exists in the document and can later be built into a MongoDB filter.
    ///
    /// # Arguments
    /// * `exists` - Whether the field should exist (true) or not exist (false)
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::filters::OperationBuilder;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User { phone_number: Option<String> }
    ///
    /// // Create an operation builder and add an "exists" operation
    /// let op_builder = OperationBuilder::<user_fields::PhoneNumber, User>::new();
    /// let final_builder = op_builder.exists(true);
    /// let filter_doc = final_builder.build();
    /// // Resulting BSON: { "phone_number": { "$exists": true } }
    /// ```
    pub fn exists(mut self, exists: bool) -> Self {
        self.ops.push(("$exists".to_string(), exists.into()));

        self
    }

    /// Builds the configured operations into a FilterBuilder.
    ///
    /// This method consumes the OperationBuilder and transforms all accumulated
    /// operations into a FilterBuilder by creating a BSON document from the operations
    /// and passing it to the build closure provided during construction.
    ///
    /// # Returns
    ///
    /// Returns a `FilterBuilder<T>` that contains the configured operations,
    /// ready to be used for further filter building or converted to a final BSON document.
    pub fn build(self) -> bson::Document {
        let field_name = F::field_name().to_string();

        if self.ops.is_empty() {
            return bson::doc! {};
        }

        // Special handling for equality: MongoDB allows both { field: value } and { field: { $eq: value } }
        // For simplicity with other operations, we'll use the explicit $eq form
        let mut operations = bson::Document::new();

        for (op_name, value) in self.ops {
            operations.insert(op_name, value);
        }

        // Return document with operations
        bson::doc! { field_name: operations }
    }
}

// Testing internal/private functions

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_witnesses::FieldName;

    // Test field marker types
    struct Name;
    impl FieldName for Name {
        fn field_name() -> &'static str {
            "Name"
        }
    }

    struct Category;
    impl FieldName for Category {
        fn field_name() -> &'static str {
            "Category"
        }
    }

    struct TestStruct;

    #[test]
    fn test_field_path_empty_prefix() {
        let builder = FilterBuilder::<TestStruct>::new();
        let path = builder.field_path::<Name>();

        assert_eq!(path, "Name");
    }

    #[test]
    fn test_field_path_single_prefix() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec!["user".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "user.Name");
    }

    #[test]
    fn test_field_path_multiple_prefix() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec!["profile".to_string(), "details".to_string()];

        let path = builder.field_path::<Category>();

        assert_eq!(path, "profile.details.Category");
    }

    #[test]
    fn test_field_path_nested_deep() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec![
            "root".to_string(),
            "level1".to_string(),
            "level2".to_string(),
            "level3".to_string(),
        ];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "root.level1.level2.level3.Name");
    }
}
