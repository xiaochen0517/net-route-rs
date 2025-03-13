use crate::route::WinRoute;

#[test]
fn win_route_new_success() {
    let result = WinRoute::new();
    assert!(result.is_ok(), "Expected WinRoute creation to succeed");
}

#[test]
fn win_route_get_routes_success() {
    let win_route = WinRoute::new();
    assert!(win_route.is_ok(), "Expected WinRoute creation to succeed");
    let win_route = win_route.unwrap();
    let result = win_route.get_routes();
    assert!(result.is_ok(), "Expected get_routes to succeed");
}
