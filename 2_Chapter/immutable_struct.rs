// Section 2.5.6 Unique vs. shared ownership
struct SomeStruct {
    regular_field: u8,
}

fn main() {
    let my_struct = SomeStruct {regular_field: 0};
    let new_value = 5;
    my_struct.regular_field = new_value;
} // ERROR: `my_struct` is not declared as mutable
