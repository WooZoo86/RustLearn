fn test_primitives() {
    /************** scalar types *******************/
    //signed integer
    let var_i8 = 1i8;
    let var_i16 = 2i16;
    let var_i32 = 3i32;
    let var_i64 = 4i64;
    let var_i128 = 5i128;
    let var_isize = 0isize;//pointer size, likely to be 32bit or 64bit depends on platform os
    println!("{},{},{},{},{},{}", var_i8, var_i16, var_i32, var_i64, var_i128, var_isize);

    //unsigned integer
    let var_u8 = 6u8;
    let var_u16 = 7u16;
    let var_u32 = 8u32;
    let var_u64 = 9u64;
    let var_u128 = 110u128;
    let var_usize = 14usize;//pointer size
    println!("{},{},{},{},{},{}", var_u8, var_u16, var_u32, var_u64, var_u128, var_usize);

    //float
    let var_float = 15.0f32;
    let var_double = 16.0123f64;
    println!("{},{}", var_float, var_double);

    //char 4byts each
    let var_char = 'α';
    println!("{}", var_char);

    //bool either true or false
    let var_bool = true;
    println!("{}", var_bool);

    //unit type, empty tuple
    let var_unit_type = ();
    println!("{:?}", var_unit_type);

    /************* compound types *******************/
    let var_array = [0, 1, 2, 3, 4];//the same type
    let var_tuple = (1, 2u32, 3.0f32, 4.0123f64, 'b', true, ());//different element type
    println!("{:?}", var_array);
    println!("{:?}", var_tuple);
}

fn main() {
    test_primitives();
    println!("Hello, world!");
}
