// This test verifies that MongoComparable fails with incompatible types

use tnuctipun::{FieldWitnesses, MongoComparable, HasField, FieldName};
use tnuctipun::mongo_comparable::MongoComparable as MongoComparableTrait;
use tnuctipun::HasField;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Name: String,
    pub Price: f64,
}

// Custom type that doesn't implement Into<mongodb::bson::Bson>
struct CustomType;

fn main() {
    // This should fail because CustomType doesn't implement Into<mongodb::bson::Bson>
    fn assert_implements_mongo_comparable<T, A, B>()
    where
        T: MongoComparableTrait<A, B>
    {}
    
    // Attempt to compare String field with CustomType
    assert_implements_mongo_comparable::<Product, 
        <Product as tnuctipun::field_witnesses::HasField<product_fields::Name>>::Value, 
        CustomType>();
}
