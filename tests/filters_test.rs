use nessus_derive::MongoComparable;

use mongodb::bson;
use nessus::FieldWitnesses;
use nessus::filters::empty;
use serde::{Deserialize, Serialize};

// Define test structs
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    id: String,
    name: String,
    price: f64,
    stock: i32,
    categories: Vec<String>,
}

// Define nested structs for testing with_nested function
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct ContactInfo {
    email: String,
    phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    id: String,
    name: String,
    age: i32,
    home_address: Address,
    work_address: Address,
    contact: ContactInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Company {
    name: String,
    address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Employee {
    id: String,
    name: String,
    company: Company,
}

#[test]
fn test_equality_filter() {
    let mut builder = empty::<Product>();

    builder.eq::<product_fields::Name, _>("Smartphone".to_string());

    let expected_name = bson::doc! { "name": "Smartphone" };

    assert_eq!(builder.clauses(), &vec![expected_name.clone()]);

    builder.eq::<product_fields::Categories, _>("Electronics".to_string());

    let expected_categories = bson::doc! { "categories": "Electronics" };

    assert_eq!(builder.clauses(), &vec![expected_name, expected_categories]);
}

#[test]
fn test_greater_than_filter() {
    let mut builder = empty::<Product>();

    builder.gt::<product_fields::Price, _>(500.0);

    let expected = bson::doc! { "price": { "$gt": 500.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);

    let mut builder2 = empty::<Product>();

    builder2.gt::<product_fields::Categories, _>(vec!["Electronics".to_string()]);

    let expected_categories = bson::doc! { "categories": { "$gt": ["Electronics"] } };

    assert_eq!(builder2.clauses(), &vec![expected_categories.clone()]);

    // Even if Price field is f64, we can use an integer
    // This should work because f64 can be compared with i32
    let mut builder3 = empty::<Product>();

    builder3.gt::<product_fields::Price, _>(1000);

    let expected_int = bson::doc! { "price": { "$gt": 1000 } };

    assert_eq!(builder3.clauses(), &vec![expected_int.clone()]);
}

#[test]
fn test_less_than_filter() {
    let mut builder = empty::<Product>();

    builder.lt::<product_fields::Stock, _>(20);

    let expected = bson::doc! { "stock": { "$lt": 20 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_in_filter() {
    let mut builder = empty::<Product>();

    builder.r#in::<product_fields::Id, _>(vec!["prod-123".to_string(), "prod-456".to_string()]);

    let expected = bson::doc! { "id": { "$in": ["prod-123", "prod-456"] } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_ne_filter() {
    let mut builder = empty::<Product>();

    builder.ne::<product_fields::Name, _>("Laptop".to_string());

    let expected = bson::doc! { "name": { "$ne": "Laptop" } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_greater_than_equal_filter() {
    let mut builder = empty::<Product>();

    builder.gte::<product_fields::Price, _>(1000.0);

    let expected = bson::doc! { "price": { "$gte": 1000.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_less_than_equal_filter() {
    let mut builder = empty::<Product>();

    builder.lte::<product_fields::Price, _>(1000.0);

    let expected = bson::doc! { "price": { "$lte": 1000.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_exists_filter() {
    let mut builder = empty::<Product>();

    builder.exists::<product_fields::Categories>(true);

    let expected = bson::doc! { "categories": { "$exists": true } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_nin_filter() {
    // For a field that's a Vec<String>, we need to create a Vec<Vec<String>>
    let category1 = vec!["Electronics".to_string()];
    let category2 = vec!["Books".to_string()];
    let categories = vec![category1, category2];

    let mut builder = empty::<Product>();

    builder.nin::<product_fields::Categories, _>(categories);

    let expected = bson::doc! { "categories": { "$nin": [["Electronics"], ["Books"]] } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

// Test compile-time safety
#[test]
fn test_compile_time_safety() {
    // Note: Compile-time type safety tests are handled by trybuild tests in compile_fail/ directory:
    // - filters_wrong_type.rs: Tests that using the wrong type causes a compile error
    // - filters_nonexistent_field.rs: Tests that using a nonexistent field causes a compile error

    // Verify correct usage works
    let mut builder = empty::<Product>();

    builder.eq::<product_fields::Price, _>(599.99);

    let expected_price_filter = bson::doc! { "price": 599.99 };

    assert_eq!(builder.clauses(), &vec![expected_price_filter.clone()]);
}

#[test]
fn test_and_function_empty_builder() {
    // Test with empty builder - should return empty document
    let builder = empty::<Product>();
    let result = builder.and();
    let expected = bson::doc! {};

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_single_clause() {
    // Test with single clause - should return the clause directly (no $and wrapper)
    let mut builder = empty::<Product>();

    builder.eq::<product_fields::Name, _>("Smartphone".to_string());

    let result = builder.and();
    let expected = bson::doc! { "name": "Smartphone" };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_multiple_clauses() {
    // Test with multiple clauses - should wrap in $and
    let mut builder = empty::<Product>();

    builder
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .eq::<product_fields::Price, _>(599.99);

    let result = builder.and();
    let expected = bson::doc! {
        "$and": [
            { "name": "Smartphone" },
            { "price": 599.99 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_complex_builder_chain() {
    // Test with a complex chain of different filter types
    let mut builder = empty::<Product>();

    builder
        .eq::<product_fields::Name, _>("Gaming Laptop".to_string())
        .eq::<product_fields::Stock, _>(10);

    let result = builder.and();
    let expected = bson::doc! {
        "$and": [
            { "name": "Gaming Laptop" },
            { "stock": 10 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_three_clauses() {
    // Test with three clauses to ensure proper array construction
    let mut builder = empty::<Product>();
    builder
        .eq::<product_fields::Name, _>("Tablet".to_string())
        .eq::<product_fields::Price, _>(299.99)
        .eq::<product_fields::Stock, _>(5);

    let result = builder.and();
    let expected = bson::doc! {
        "$and": [
            { "name": "Tablet" },
            { "price": 299.99 },
            { "stock": 5 }
        ]
    };

    assert_eq!(result, expected);
}

// Tests for with_nested function
#[test]
fn test_with_nested_single_field() {
    // Test filtering on a single nested field
    let mut builder = empty::<User>();

    builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
        nested.eq::<address_fields::City, _>("New York".to_string())
    });

    let expected = bson::doc! { "home_address.city": "New York" };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_with_nested_multiple_fields() {
    // Test filtering on multiple nested fields within the same nested object
    let mut builder = empty::<User>();

    builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
        nested
            .eq::<address_fields::City, _>("San Francisco".to_string())
            .eq::<address_fields::ZipCode, _>("94102".to_string())
    });

    let expected_city = bson::doc! { "home_address.city": "San Francisco" };
    let expected_zip = bson::doc! { "home_address.zip_code": "94102" };

    assert_eq!(builder.clauses(), &vec![expected_city, expected_zip]);
}

#[test]
fn test_with_nested_different_operators() {
    // Test using different MongoDB operators within nested fields
    let mut builder = empty::<User>();

    builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
        nested
            .eq::<address_fields::Country, _>("USA".to_string())
            .ne::<address_fields::City, _>("Los Angeles".to_string())
            .exists::<address_fields::ZipCode>(true)
    });

    let expected_country = bson::doc! { "home_address.country": "USA" };
    let expected_city = bson::doc! { "home_address.city": { "$ne": "Los Angeles" } };
    let expected_zip_exists = bson::doc! { "home_address.zip_code": { "$exists": true } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_country, expected_city, expected_zip_exists]
    );
}

#[test]
fn test_with_nested_multiple_nested_objects() {
    // Test filtering on different nested objects within the same parent
    let mut builder = empty::<User>();

    builder
        .with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
            nested.eq::<address_fields::City, _>("Boston".to_string())
        })
        .with_nested::<user_fields::WorkAddress, Address, _>(|nested| {
            nested.eq::<address_fields::City, _>("Cambridge".to_string())
        });

    let expected_home = bson::doc! { "home_address.city": "Boston" };
    let expected_work = bson::doc! { "work_address.city": "Cambridge" };

    assert_eq!(builder.clauses(), &vec![expected_home, expected_work]);
}

#[test]
fn test_with_nested_mixed_with_regular_filters() {
    // Test combining nested filters with regular field filters
    let mut builder = empty::<User>();

    builder
        .eq::<user_fields::Name, _>("John Doe".to_string())
        .with_nested::<user_fields::Contact, ContactInfo, _>(|nested| {
            nested.eq::<contactinfo_fields::Email, _>("john@example.com".to_string())
        })
        .gt::<user_fields::Age, _>(25);

    let expected_name = bson::doc! { "name": "John Doe" };
    let expected_email = bson::doc! { "contact.email": "john@example.com" };
    let expected_age = bson::doc! { "age": { "$gt": 25 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_name, expected_email, expected_age]
    );
}

#[test]
fn test_with_nested_deep_nesting() {
    // Test deeply nested structures (Company -> Address)
    let mut builder = empty::<Employee>();

    builder.with_nested::<employee_fields::Company, Company, _>(|nested| {
        nested
            .eq::<company_fields::Name, _>("Tech Corp".to_string())
            .with_nested::<company_fields::Address, Address, _>(|deeply_nested| {
                deeply_nested
                    .eq::<address_fields::City, _>("Seattle".to_string())
                    .eq::<address_fields::Country, _>("USA".to_string())
            })
    });

    let expected_company_name = bson::doc! { "company.name": "Tech Corp" };
    let expected_company_city = bson::doc! { "company.address.city": "Seattle" };
    let expected_company_country = bson::doc! { "company.address.country": "USA" };

    assert_eq!(
        builder.clauses(),
        &vec![
            expected_company_name,
            expected_company_city,
            expected_company_country
        ]
    );
}

#[test]
fn test_with_nested_and_function() {
    // Test that nested fields work correctly with the and() function
    let mut builder = empty::<User>();

    builder
        .eq::<user_fields::Name, _>("Alice Smith".to_string())
        .with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
            nested
                .eq::<address_fields::City, _>("Portland".to_string())
                .eq::<address_fields::ZipCode, _>("97201".to_string())
        });

    let result = builder.and();
    let expected = bson::doc! {
        "$and": [
            { "name": "Alice Smith" },
            { "home_address.city": "Portland" },
            { "home_address.zip_code": "97201" }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_nested_single_clause_and_function() {
    // Test that a single nested clause works correctly with and() function
    let mut builder = empty::<User>();

    builder.with_nested::<user_fields::Contact, ContactInfo, _>(|nested| {
        nested.eq::<contactinfo_fields::Phone, _>("+1-555-0123".to_string())
    });

    let result = builder.and();
    let expected = bson::doc! { "contact.phone": "+1-555-0123" };

    assert_eq!(result, expected);
}

#[test]
fn test_with_nested_complex_operators() {
    // Test nested fields with complex MongoDB operators like $in and $nin
    let mut builder = empty::<User>();

    builder.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
        nested
            .r#in::<address_fields::City, _>(vec![
                "New York".to_string(),
                "Boston".to_string(),
                "Chicago".to_string(),
            ])
            .nin::<address_fields::Country, _>(vec!["Canada".to_string(), "Mexico".to_string()])
            .gte::<address_fields::ZipCode, _>("10000".to_string())
            .lte::<address_fields::ZipCode, _>("99999".to_string())
    });

    let expected_city_in =
        bson::doc! { "home_address.city": { "$in": ["New York", "Boston", "Chicago"] } };
    let expected_country_nin =
        bson::doc! { "home_address.country": { "$nin": ["Canada", "Mexico"] } };
    let expected_zip_gte = bson::doc! { "home_address.zip_code": { "$gte": "10000" } };
    let expected_zip_lte = bson::doc! { "home_address.zip_code": { "$lte": "99999" } };

    assert_eq!(
        builder.clauses(),
        &vec![
            expected_city_in,
            expected_country_nin,
            expected_zip_gte,
            expected_zip_lte
        ]
    );
}

#[test]
fn test_or_function_simple_equality() {
    // Test OR with simple equality conditions
    let mut builder = empty::<Product>();
    let names = vec!["Laptop", "Smartphone", "Tablet"];

    builder.or::<product_fields::Name, _, _>(names, |filter, name| {
        filter.eq::<product_fields::Name, _>(name.to_string())
    });

    let expected = bson::doc! {
        "$or": [
            { "name": "Laptop" },
            { "name": "Smartphone" },
            { "name": "Tablet" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_price_ranges() {
    // Test OR with complex conditions (price ranges)
    let mut builder = empty::<Product>();
    let price_ranges = vec![(0.0, 100.0), (500.0, 1000.0), (2000.0, 5000.0)];

    builder.or::<product_fields::Price, _, _>(price_ranges, |filter, (min, max)| {
        filter
            .gte::<product_fields::Price, _>(min)
            .lte::<product_fields::Price, _>(max)
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "price": { "$gte": 0.0 } },
            { "price": { "$lte": 100.0 } },
            { "price": { "$gte": 500.0 } },
            { "price": { "$lte": 1000.0 } },
            { "price": { "$gte": 2000.0 } },
            { "price": { "$lte": 5000.0 } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_different_operators() {
    // Test OR with different MongoDB operators
    let mut builder = empty::<Product>();
    let search_criteria = vec!["Electronics", "Books", "Clothing"];

    builder.or::<product_fields::Categories, _, _>(search_criteria, |filter, category| {
        filter
            .eq::<product_fields::Categories, _>(vec![category.to_string()])
            .exists::<product_fields::Name>(true)
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "name": { "$exists": true } },
            { "categories": ["Books"] },
            { "name": { "$exists": true } },
            { "categories": ["Clothing"] },
            { "name": { "$exists": true } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_empty_input() {
    // Test OR with empty input - should add empty $or array
    let mut builder = empty::<Product>();
    let empty_vec: Vec<String> = vec![];

    builder.or::<product_fields::Name, _, _>(empty_vec, |filter, name| {
        filter.eq::<product_fields::Name, _>(name)
    });

    let expected = bson::doc! { "$or": [] };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_single_item() {
    // Test OR with single item
    let mut builder = empty::<Product>();
    let single_name = vec!["Laptop"];

    builder.or::<product_fields::Name, _, _>(single_name, |filter, name| {
        filter.eq::<product_fields::Name, _>(name.to_string())
    });

    let expected = bson::doc! {
        "$or": [
            { "name": "Laptop" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_chained_with_other_conditions() {
    // Test OR chained with other filter conditions
    let mut builder = empty::<Product>();

    // First add a regular condition
    builder.gt::<product_fields::Stock, _>(0);

    // Then add OR condition
    let categories = vec!["Electronics", "Books"];
    builder.or::<product_fields::Categories, _, _>(categories, |filter, category| {
        filter.eq::<product_fields::Categories, _>(vec![category.to_string()])
    });

    // Add another regular condition
    builder.lte::<product_fields::Price, _>(1000.0);

    let expected_stock = bson::doc! { "stock": { "$gt": 0 } };
    let expected_or = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "categories": ["Books"] }
        ]
    };
    let expected_price = bson::doc! { "price": { "$lte": 1000.0 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_stock, expected_or, expected_price]
    );
}

#[test]
fn test_or_function_with_nested_fields() {
    // Test OR with nested field access
    let mut builder = empty::<User>();
    let cities = vec!["New York", "Boston", "Chicago"];

    builder.or::<user_fields::HomeAddress, _, _>(cities, |filter, city| {
        filter.with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
            nested.eq::<address_fields::City, _>(city.to_string())
        })
    });

    let expected = bson::doc! {
        "$or": [
            { "home_address.city": "New York" },
            { "home_address.city": "Boston" },
            { "home_address.city": "Chicago" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_complex_nested_conditions() {
    // Test OR with complex nested conditions
    let mut builder = empty::<User>();
    let age_ranges = vec![(18, 25), (30, 40), (50, 65)];

    builder.or::<user_fields::Age, _, _>(age_ranges, |filter, (min_age, max_age)| {
        filter
            .gte::<user_fields::Age, _>(min_age)
            .lte::<user_fields::Age, _>(max_age)
            .with_nested::<user_fields::Contact, ContactInfo, _>(|nested| {
                nested.exists::<contactinfo_fields::Email>(true)
            })
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "age": { "$gte": 18 } },
            { "age": { "$lte": 25 } },
            { "contact.email": { "$exists": true } },
            { "age": { "$gte": 30 } },
            { "age": { "$lte": 40 } },
            { "contact.email": { "$exists": true } },
            { "age": { "$gte": 50 } },
            { "age": { "$lte": 65 } },
            { "contact.email": { "$exists": true } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_and_final_result() {
    // Test OR with and() function to get final MongoDB query
    let mut builder = empty::<Product>();
    let names = vec!["Laptop", "Smartphone"];

    builder
        .gt::<product_fields::Price, _>(100.0)
        .or::<product_fields::Name, _, _>(names, |filter, name| {
            filter.eq::<product_fields::Name, _>(name.to_string())
        });

    let result = builder.and();
    let expected = bson::doc! {
        "$and": [
            { "price": { "$gt": 100.0 } },
            { "$or": [
                { "name": "Laptop" },
                { "name": "Smartphone" }
            ]}
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_or_function_realistic_usage() {
    // Test OR with realistic usage where each closure generates a single condition
    let mut builder = empty::<Product>();
    let category_filters = vec!["Electronics", "Books", "Clothing"];

    builder.or::<product_fields::Categories, _, _>(category_filters, |filter, category| {
        // Each iteration should ideally generate a single meaningful condition
        filter.eq::<product_fields::Categories, _>(vec![category.to_string()])
    });

    let expected = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "categories": ["Books"] },
            { "categories": ["Clothing"] }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_mixed_single_and_multiple_clauses() {
    // Test that demonstrates mixing single and multiple clause generations
    let mut builder = empty::<Product>();
    let mixed_conditions = vec![("simple", 100.0), ("range", 500.0)];

    builder.or::<product_fields::Price, _, _>(
        mixed_conditions,
        |filter, (condition_type, value)| match condition_type {
            "simple" => filter.eq::<product_fields::Price, _>(value),
            "range" => filter
                .gte::<product_fields::Price, _>(value)
                .lte::<product_fields::Price, _>(value * 2.0),
            _ => filter,
        },
    );

    // Shows how multiple clauses from "range" condition get flattened
    let expected = bson::doc! {
        "$or": [
            { "price": 100.0 },
            { "price": { "$gte": 500.0 } },
            { "price": { "$lte": 1000.0 } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_single_equality() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()));

    let expected = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_numeric_equality() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.eq(500.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 500.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_integer_field() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Stock, _>(|op| op.eq(10));

    let expected = bson::doc! {
        "stock": {
            "$not": {
                "stock": { "$eq": 10 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_chained_with_other_filters() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .gt::<product_fields::Price, _>(100.0);

    let expected_not = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };
    let expected_gt = bson::doc! { "price": { "$gt": 100.0 } };

    assert_eq!(builder.clauses(), &vec![expected_not, expected_gt]);
}

#[test]
fn test_not_filter_multiple_not_operations() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .not::<product_fields::Price, _>(|op| op.eq(500.0));

    let expected_not_name = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };
    let expected_not_price = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 500.0 }
            }
        }
    };

    assert_eq!(
        builder.clauses(),
        &vec![expected_not_name, expected_not_price]
    );
}

#[test]
fn test_not_filter_with_and_combination() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .eq::<product_fields::Categories, _>("Electronics".to_string());

    let filter = builder.and();

    let expected = bson::doc! {
        "$and": [
            {
                "name": {
                    "$not": {
                        "name": { "$eq": "Smartphone" }
                    }
                }
            },
            { "categories": "Electronics" }
        ]
    };

    assert_eq!(filter, expected);
}

#[test]
fn test_not_filter_type_compatibility() {
    let mut builder = empty::<Product>();

    // Test that we can use i32 for f64 price field due to MongoComparable
    builder.not::<product_fields::Price, _>(|op| {
        op.eq(100) // i32 instead of f64
    });

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 100 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

// Tests for OperationBuilder functions
#[test]
fn test_operation_builder_gt() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gt(100.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gt": 100.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_gte() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gte(50.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gte": 50.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_lt() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Stock, _>(|op| op.lt(5));

    let expected = bson::doc! {
        "stock": {
            "$not": {
                "stock": { "$lt": 5 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_lte() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.lte(200.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$lte": 200.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_in() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Categories, _>(|op| {
        op.r#in(vec!["Electronics".to_string(), "Books".to_string()])
    });

    let expected = bson::doc! {
        "categories": {
            "$not": {
                "categories": { "$in": ["Electronics", "Books"] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_nin() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Categories, _>(|op| {
        op.nin(vec!["Clothing".to_string(), "Shoes".to_string()])
    });

    let expected = bson::doc! {
        "categories": {
            "$not": {
                "categories": { "$nin": ["Clothing", "Shoes"] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_exists() {
    let mut builder = empty::<User>();

    builder.not::<user_fields::Contact, _>(|op| op.exists(true));

    let expected = bson::doc! {
        "contact": {
            "$not": {
                "contact": { "$exists": true }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_exists_false() {
    let mut builder = empty::<User>();

    builder.not::<user_fields::Contact, _>(|op| op.exists(false));

    let expected = bson::doc! {
        "contact": {
            "$not": {
                "contact": { "$exists": false }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_chaining_multiple_operations() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gte(100.0).lte(500.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gte": 100.0, "$lte": 500.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_type_compatibility_gt() {
    let mut builder = empty::<Product>();

    // Test that we can use i32 for f64 price field due to MongoComparable
    builder.not::<product_fields::Price, _>(|op| {
        op.gt(100) // i32 instead of f64
    });

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gt": 100 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_type_compatibility_in() {
    let mut builder = empty::<User>();

    // Test that we can use Vec<i32> for i32 age field
    builder.not::<user_fields::Age, _>(|op| op.r#in(vec![20, 30, 40]));

    let expected = bson::doc! {
        "age": {
            "$not": {
                "age": { "$in": [20, 30, 40] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_complex_chain() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Price, _>(|op| op.gt(50.0).lt(1000.0))
        .eq::<product_fields::Name, _>("Laptop".to_string());

    let expected_not = bson::doc! {
        "price": {
            "$not": {
                "price": {
                    "$gt": 50.0,
                    "$lt": 1000.0
                }
            }
        }
    };
    let expected_eq = bson::doc! { "name": "Laptop" };

    assert_eq!(builder.clauses(), &vec![expected_not, expected_eq]);
}

// Tests for Into<bson::Document> implementation

#[test]
fn test_into_bson_document_empty_builder() {
    let builder = empty::<Product>();
    let doc: bson::Document = builder.into();

    assert_eq!(doc, bson::doc! {});
}

#[test]
fn test_into_bson_document_single_clause() {
    let mut builder = empty::<Product>();
    builder.eq::<product_fields::Name, _>("Laptop".to_string());

    let doc: bson::Document = builder.into();
    let expected = bson::doc! { "name": "Laptop" };

    assert_eq!(doc, expected);
}

#[test]
fn test_into_bson_document_multiple_clauses() {
    let mut builder = empty::<Product>();
    builder
        .eq::<product_fields::Name, _>("Laptop".to_string())
        .gt::<product_fields::Price, _>(500.0)
        .lt::<product_fields::Stock, _>(100);

    let doc: bson::Document = builder.into();
    let expected = bson::doc! {
        "$and": [
            { "name": "Laptop" },
            { "price": { "$gt": 500.0 } },
            { "stock": { "$lt": 100 } }
        ]
    };

    assert_eq!(doc, expected);
}

#[test]
fn test_into_bson_document_complex_filters() {
    let mut builder = empty::<User>();
    builder
        .eq::<user_fields::Name, _>("John Doe".to_string())
        .gte::<user_fields::Age, _>(18)
        .exists::<user_fields::Id>(true);

    let doc: bson::Document = builder.into();
    let expected = bson::doc! {
        "$and": [
            { "name": "John Doe" },
            { "age": { "$gte": 18 } },
            { "id": { "$exists": true } }
        ]
    };

    assert_eq!(doc, expected);
}

#[test]
fn test_into_bson_document_with_function_parameter() {
    // Test that FilterBuilder can be passed to functions expecting Into<bson::Document>
    fn process_filter(filter: impl Into<bson::Document>) -> bson::Document {
        filter.into()
    }

    let mut builder = empty::<Product>();
    builder
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .lte::<product_fields::Price, _>(800.0);

    let result = process_filter(builder);
    let expected = bson::doc! {
        "$and": [
            { "name": "Smartphone" },
            { "price": { "$lte": 800.0 } }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_into_bson_document_consistency_with_and() {
    // Test that Into<bson::Document> produces the same result as and()
    let mut builder1 = empty::<Product>();
    builder1
        .eq::<product_fields::Name, _>("Tablet".to_string())
        .gt::<product_fields::Price, _>(200.0)
        .exists::<product_fields::Id>(true);

    let and_result = builder1.and();

    // Create a second identical builder for the into() test
    let mut builder2 = empty::<Product>();
    builder2
        .eq::<product_fields::Name, _>("Tablet".to_string())
        .gt::<product_fields::Price, _>(200.0)
        .exists::<product_fields::Id>(true);

    let into_result: bson::Document = builder2.into();

    assert_eq!(and_result, into_result);
}

#[test]
fn test_into_bson_document_with_nested_filters() {
    let mut builder = empty::<User>();
    builder
        .eq::<user_fields::Name, _>("Alice".to_string())
        .with_nested::<user_fields::HomeAddress, Address, _>(|nested| {
            nested.eq::<address_fields::City, _>("New York".to_string())
        });

    let doc: bson::Document = builder.into();
    let expected = bson::doc! {
        "$and": [
            { "name": "Alice" },
            { "home_address.city": "New York" }
        ]
    };

    assert_eq!(doc, expected);
}
