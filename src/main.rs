use std::str::FromStr;

use maplit::hashmap;

use common::data::payload::Value;
use common::data::schema::{Property, PropertyBuilder, Type};
use common::data::serialization::Jsonable;
use common::isolang::Language;

fn main() {
    println!("Begin");
    // let vendor = VendorBuilder::default()
    //     .name("ravenfire".try_into().unwrap())
    //     .build()
    //     .unwrap();
    //
    // let json = vendor.to_json().unwrap();
    //
    // let again: Vendor = Vendor::from_json(&json).unwrap();

    // TODO: Start Here
    let original = PropertyBuilder::default()
        .key("test".try_into().unwrap())
        .data_type(Type::Integer)
        .titles(hashmap! {Language::from_str("en").unwrap() => "Test".to_owned()})
        .descriptions(hashmap! {Language::from_str("en").unwrap() => "Descriptions".to_owned()})
        .optional(false)
        .enumerations(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(4),
        ])
        .build()
        .unwrap();

    let json = original.to_json().unwrap();

    let again = Property::from_json(&json).unwrap();

    println!("End");
}
