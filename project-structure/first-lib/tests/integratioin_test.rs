use first_lib::get_process_id;
#[test]
fn test_get_process_id() {
    assert_ne!(get_process_id(),0,"Error in get_process_id()");
}