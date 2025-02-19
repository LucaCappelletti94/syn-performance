//! Test submodule for the `using-syn-partial` crate.

use shared::{Attribute, Struct};
use using_syn_partial::implements_struct_and_foo;

#[test]
/// Tests the `implements_struct_and_foo` function.
pub fn test_using_syn_partial() {
	let s = Struct {
		name: "MyStruct".to_string(),
		attributes: vec![
			Attribute {
				name: "field1".to_string(),
				r#type: "String".to_string(),
				optional: false,
			},
			Attribute {
				name: "field2".to_string(),
				r#type: "i32".to_string(),
				optional: true,
			},
		],
	};

	let generated = implements_struct_and_foo(&s);
	println!("{:?}", generated);
}