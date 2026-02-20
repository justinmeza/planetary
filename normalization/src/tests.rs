use crate::Sample;

#[test]
fn test_serialization() {
    let sample = Sample {
        number: 5,
        // flag: true,
        // text: "Hello".to_string(),
    };
    let serialized = sample.serialize();
    assert_eq!(serialized, "{\"number\": 5,\"flag\": true,\"text\": \"Hello\"}");
}
