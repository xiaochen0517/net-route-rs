use crate::interface::Interface;

#[test]
fn interface_new_success() {
    let interface = Interface::new();
    assert!(
        interface.get_interfaces().is_ok(),
        "Expected Interface creation to succeed"
    );
}

#[test]
fn interface_get_interfaces_success() {
    let interface = Interface::new();
    let result = interface.get_interfaces();
    assert!(result.is_ok(), "Expected get_interfaces to succeed");
    let interfaces = result.unwrap();
    assert!(
        !interfaces.is_empty(),
        "Expected interfaces to not be empty"
    );
}
