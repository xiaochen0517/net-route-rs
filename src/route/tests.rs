use crate::route::WinRoute;
use test_case::test_case;

#[test_case(true ; "测试创建路由类")]
fn win_route_new_success(expected: bool) {
    let result = WinRoute::new();
    assert_eq!(result.is_ok(), expected);
}

#[test_case(true ; "测试获取路由信息")]
fn win_route_get_routes_success(expected: bool) {
    let win_route = WinRoute::new();
    assert_eq!(win_route.is_ok(), expected);
    let win_route = win_route.unwrap();
    let result = win_route.get_routes();
    assert_eq!(result.is_ok(), expected);
}
