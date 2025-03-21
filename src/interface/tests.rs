use crate::interface::Interface;
use test_case::test_case;

#[test_case(true ; "测试创建接口类")]
fn interface_new_success(expected: bool) {
    let interface = Interface::new();
    assert_eq!(interface.get_interfaces().is_ok(), expected);
}

#[test_case(true ; "测试获取接口信息")]
fn interface_get_interfaces_success(expected: bool) {
    let interface = Interface::new();
    let result = interface.get_interfaces();
    assert_eq!(result.is_ok(), expected);
    let interfaces = result.unwrap();
    assert_eq!(!interfaces.is_empty(), expected);
}
