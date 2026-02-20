// use normalization::Sample;
// use normalization::normalization::{NormalizationError};

use normalization::NormalizationError;
use normalization_macros::{Deserializable, Serializable};

#[derive(Serializable, Deserializable)]
pub struct Sample {
    pub number: i32,
    pub flag: bool,
    pub text: String,
    pub nested: Nested,
}

// TODO:  Try to remove the need to have the Debug attribute, by calling serialize().
// TODO:  Debug why serialization and deserialization are incorrect.
#[derive(Serializable, Deserializable, Debug)]
pub struct Nested {
    pub number: i32,
    pub string: String,
}

impl PartialEq for Nested {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.string == other.string
    }
}

#[test]
fn test_serialization() {
    let sample = Sample {
        number: 5,
        flag: true,
        text: "Hello".to_string(),
        nested: Nested {number: 8, string: "Hi".to_string()},
    };
    let serialized = sample.serialize();
    assert_eq!(serialized, "{number: 5,flag: true,text: \"Hello\",nested: {number: 8,string: \"Hi\"}}");
}

#[test]
fn test_deserialization() {
    let serialized = "{number: 5,flag: true,text: \"Hello\\:\\,\",nested: {number: 8,string: \"Hi\"}}";

    let deserialized: Result<Sample, NormalizationError> =
        Sample::deserialize(serialized);

    match deserialized {
        Ok(sample) => {
            assert_eq!(sample.number, 5);
            assert_eq!(sample.flag, true);
            assert_eq!(sample.text, "Hello:,".to_string());
            assert_eq!(sample.nested.number, 8);
            assert_eq!(sample.nested.string, "Hi".to_string());
        }
        Err(e) => panic!("Deserialization failed with error: {:?}", e),
    }
}

#[derive(Serializable, Deserializable)]
struct TestStruct {
    numbers: Vec<i32>,
    strings: Vec<String>,
    structs: Vec<Nested>,
    sample: Sample,
    samples: Vec<Sample>,
}

#[test]
fn test_vector_serialization_deserialization() {
    let test_struct = TestStruct {
        numbers: vec![1, 2, 3],
        strings: vec!["a".to_string(), "b".to_string()],
        structs: vec![Nested {number: 1, string: "c".to_string()}, Nested {number: 2, string: "d".to_string()}],
        sample: Sample {
            number: 5,
            flag: true,
            text: "Hello".to_string(),
            nested: Nested {number: 8, string: "Hi".to_string()},
        },
        samples: vec![
            Sample {
                number: 6,
                flag: true,
                text: "A".to_string(),
                nested: Nested {number: 7, string: "B".to_string()},
            },
            Sample {
                number: 9,
                flag: false,
                text: "C".to_string(),
                nested: Nested {number: 10, string: "D".to_string()},
            },
        ],
    };
    let serialized = test_struct.serialize();
    println!("Serialized: {}", serialized);

    let deserialized: Result<TestStruct, NormalizationError> = TestStruct::deserialize(&serialized);
    match deserialized {
        Ok(test_struct) => {
            // println!("Deserialized: {:?}", test_struct);
            assert_eq!(test_struct.numbers, vec![1, 2, 3]);
            assert_eq!(test_struct.strings, vec!["a", "b"]);
            assert_eq!(test_struct.structs[0], Nested {number: 1, string: "c".to_string()});
            assert_eq!(test_struct.structs[1], Nested {number: 2, string: "d".to_string()});
            assert_eq!(test_struct.sample.number, 5);
            assert_eq!(test_struct.sample.flag, true);
            assert_eq!(test_struct.sample.text, "Hello".to_string());
            assert_eq!(test_struct.sample.nested.number, 8);
            assert_eq!(test_struct.sample.nested.string, "Hi".to_string());
        }
        Err(e) => panic!("Deserialization failed with error: {:?}", e),
    }
}

// #[derive(Serializable, Deserializable)]
// pub struct Sample {
//     pub number: i32,
//     pub flag: bool,
//     pub text: String,
// }

// #[test]
// fn test_serialization() {
//     let sample = Sample {
//         number: 5,
//         flag: true,
//         text: "Hello".to_string(),
//     };
//     let serialized = sample.serialize();
//     assert_eq!(serialized, "{number: 5,flag: true,text: \"Hello\"}");
// }

// #[test]
// fn test_deserialization() {
//     let serialized = "{number: 5,flag: true,text: \"Hello\\:\\,\"}";

//     let deserialized: Result<Sample, NormalizationError> =
//         Sample::deserialize(serialized);

//     match deserialized {
//         Ok(sample) => {
//             assert_eq!(sample.number, 5);
//             assert_eq!(sample.flag, true);
//             assert_eq!(sample.text, "Hello:,".to_string());
//         }
//         Err(e) => panic!("Deserialization failed with error: {:?}", e),
//     }
// }
