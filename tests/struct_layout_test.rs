use c_struct_layout::CStructLayout;

#[derive(CStructLayout)]
#[repr(C)]
struct MyData {
    x: u32,
    y: f64,
}

#[test]
fn test_check_layout() {
    MyData::check_layout();
}
