use ray_derive::AsAny;
use ray_traits::AsAny;

#[derive(AsAny)]
struct TestStruct {
    value: i32,
}

#[test]
fn test_derive_as_any_basic() {
    let test = TestStruct { value: 42 };
    // Call the generated as_any method
    let any_ref = test.as_any();
    // Verify it returns a reference to the correct type
    assert!(any_ref.is::<TestStruct>());
}

#[test]
fn test_derive_as_any_downcast() {
    let test = TestStruct { value: 42 };
    let any_ref = test.as_any();
    // Downcast back to the concrete type
    let downcasted = any_ref.downcast_ref::<TestStruct>().unwrap();
    assert_eq!(downcasted.value, 42);
}

#[derive(AsAny)]
struct UnitStruct;

#[test]
fn test_derive_as_any_unit_struct() {
    let test = UnitStruct;
    let any_ref = test.as_any();
    assert!(any_ref.is::<UnitStruct>());
}
