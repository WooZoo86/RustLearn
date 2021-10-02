fn test_integer_types() {
    //signed integer
    let var_i8 = 1i8;
    let var_i16 = 2i16;
    let var_i32 = 3i32;
    let var_i64 = 4i64;
    let var_i128 = 5i128;
    let var_isize = 0isize;//pointer size, likely to be 32bit or 64bit depends on platform os
    println!("signe integer->{},{},{},{},{},{}", var_i8, var_i16, var_i32, var_i64, var_i128, var_isize);

    //unsigned integer
    let var_u8 = 6u8;
    let var_u16 = 7u16;
    let var_u32 = 8u32;
    let var_u64 = 9u64;
    let var_u128 = 110u128;
    let var_usize = 14usize;//pointer size
    println!("unsigned integer->{},{},{},{},{},{}", var_u8, var_u16, var_u32, var_u64, var_u128, var_usize);

    let var_default = 1;//default integer type is i32
    let var_specific: i16 = 2;//we can specific it type
    println!("default integer->{},specific integer->{}", var_default, var_specific);

    //use const value,but must specific const type
    const AGE: i32 = 32;
    let var_age = AGE;
    println!("const integer->{},const initialed integer->{}", AGE, var_age);

    //use literal value
    let var_hex = 0x10;//hex
    let var_bin = 0b1100_1100;//binary also can be 0b11001100
    let var_oct = 0o123;//octal
    let var_decimal = 123_456;//decimal also can be 123456
    let var_byte = b'a';//byte for u8 only
    println!("literal initialed integer->{},{},{},{},{}", var_hex, var_bin, var_oct, var_decimal, var_byte);

    //if you want change variable value,define mut variable
    let mut age = AGE;//caution:we define a new variable to override the preview variable
    age = 18;//we can chang it value
    println!("mut integer->{}", age);
}

fn test_float_types() {
    //float
    let var_float = 15.0f32;
    let var_double = 16.0123f64;
    println!("float->{},{}", var_float, var_double);

    //default
    let var_default = 3.1415;
    println!("default float->{}", var_default);

    //cast type
    println!("f32->u8:{}", 100.02_f32 as u8);
    println!("f32->i8:{}", 259.02_f32 as i8);

    //float range
    println!("float->max:{},min:{},nan:{},infinity:{},neg_infinity:{}", f32::MAX, f32::MIN, f32::NAN, f32::INFINITY, f32::NEG_INFINITY);
}

fn test_char_types() {
    //char 4byts each
    let var_char = 'α';
    println!("{}", var_char);

    //ascii
    let var_ascii = 'b';
    //unicode
    let var_unicode = '\u{CA0}';
    //escape character
    let var_escape = '\n';
    let var_escape_hex = '\x67';
    println!("ascii->{},unicode->{},escape->{},{}", var_ascii, var_unicode, var_escape, var_escape_hex);

    //type cast
    println!("char->i32:{:x}", '\x67' as i32);
}

fn test_bool_types() {
    //bool either true or false
    let var_bool = true;
    println!("{}", var_bool);

    //type cast
    println!("bool->i32:{},{}", true as i32, false as i32);

    //bool express
    let var_ret = !(true & false | true);
    println!("bool expression->{}", var_ret);
}

fn test_scalar_types() {
    /************** scalar types *******************/
    test_integer_types();
    test_float_types();
    test_char_types();
    test_bool_types();
}

fn test_compound_types() {
    /************* compound types *******************/
    //array:fixed length,same type,can not changed value
    let var_array = [0, 1, 2, 3, 4];//five i32 element
    let var_array_initial = [1; 10];//10 element of 1
    println!("var_array->{:?},length->{:?}", var_array, var_array.len());
    println!("var_array_initial->{:?},length->{:?}", var_array_initial, var_array_initial.len());
    //mut array can change element value but type and length
    let mut var_target = [2; 5];
    var_target[0] = 3;//do not access element out of bound
    println!("var_target->{:?},length->{:?}", var_target, var_target.len());
    //rust don not have VLA yet,so we use vec as dynamic array
    let mut var_dynamic = vec![1, 2, 3, 4];
    var_dynamic.push(5);
    println!("dynamic array->{:?}", var_dynamic);

    //tuple
    let var_tuple = (1, 2u32, 3.0f32, 4.0123f64, 'b', true, ());//different element type, but limited elements
    println!("{:?}", var_tuple);
    println!("tuple.0={}", var_tuple.0);//access member by indicate it's position
    //destruct tuple
    let (x, y, z, i, j, k, m) = var_tuple;
    println!("x->{},j->{}", x, j);
    //be careful on empty tuple,if tuple get one element,you should add , followed it
    let var_empty = ();
    let var_one_element = (1, );
    println!("empty tuple->{:?},one element tuple->{:?}", var_empty, var_one_element);

    //range
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });//not include 5
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));//include 5
    println!("sum->{:?}", (1..10).sum::<i32>());
    assert_eq!(1 + 2 + 3 + 4, (1..5).sum());
    //range iterator
    for i in 1..5 {
        print!("{},", i);
    }
    println!();
    for i in 1..=5 {
        print!("{},", i);
    }
    println!();

    //slice
    let mut var_target = [1, 2, 3, 4, 5];
    println!("{:?}", &var_target[..]);//all
    println!("{:?}", &var_target[1..]);//except index 0
    println!("{:?}", &var_target[..4]);//except index 4
    println!("{:?}", &var_target[1..4]);//except index 0 and 4
    println!("{:?}", &var_target[1..=4]);//except index 0
    let var_string = ["1", "2", "3", "4"];
    println!("{:?}", &var_string[1..=3]);//except index 0
    //mut slice can change value
    let var_slice = &mut var_target;
    var_slice.fill(0);
    var_slice[1] = 2;
    println!("slice->{:?}", var_slice);
}

fn test_special_types() {
    //never type this need nightly version rust
    /*#![feature(never_type)]
    let x:! = {
        return 123
    };*/

    //unit type, empty tuple
    let var_unit_type = ();
    println!("{:?}", var_unit_type);
}

fn test_primitive_types() {
    test_scalar_types();
    test_compound_types();
    test_special_types();
}

//no parameter enum
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

//unit struct
#[derive(Debug)]
struct Favorite;

//tuple struct
#[derive(Debug)]
struct Class(String, u32, u32);

//c style struct or named struct
#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    sex: Gender,
    weight: f32,
    height: f32,
    class: Class,
    favorite: Favorite,
}

fn test_struct_types() {
    //normal variable cannot be modified
    let xiaohua: Student = Student { name: String::from("xiaohua"), age: 14, sex: Gender::Female, weight: 1.55, height: 43.2, class: Class(String::from("yiban"), 1, 50), favorite: Favorite };//initial list must indicate the member name of the element.
    println!("xiaohua->{:?}", xiaohua);

    //define const must special it type
    const AGE: u32 = 15;
    //define some variables
    let name = String::from("name");
    let age = AGE;
    let sex = Gender::Female;
    let weight = 50.0;
    let height = 1.70;
    let class = Class(String::from("yiban"), 1, 50);
    let favorite = Favorite;

    //we don't want indicate the name of element, so we use above variable to initial a student
    //let lileifull=Student{name:name,age:age,sex:sex,weight:weight,height:height};//stupid to use  this statement
    let lilei = Student { name, age, sex, weight, height, class, favorite };
    println!("lilei->{:?}", lilei);

    //mut variable can be modified, and we use the rest element of xiaohua to initial xiaoming
    let mut xiaoming: Student = Student { name: String::from("xiaoming"), ..xiaohua };
    xiaoming.name = String::from("xiaoming");
    xiaoming.age = 18;
    xiaoming.sex = Gender::Male;
    xiaoming.height = 1.75;
    xiaoming.weight = 50.2;
    xiaoming.class.0 = String::from("erban");
    xiaoming.class.1 = 2;
    xiaoming.class.2 = 46;
    println!("xiaoming->{:?}", xiaoming);

    //we can destruct and get some element value
    let Student { name, age, sex, weight, height, class, favorite } = xiaoming;
    //let Student { name:name, age:age, sex:sex, weight:weight, height:height, class:class, favorite:favorite } = xiaoming;//stupid to use  this statement
    println!("destruct xiaoming->{:?},{:?},{:?},{:?},{:?},{:?},{:?}", name, age, sex, weight, height, class, favorite);
}

//no parameter enum
enum Color {
    Red,
    Blue,
    Yellow,
}

//c style enum
#[derive(Debug)]
enum WeekDay {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

//with parameter enum
enum Operations {
    Add { x: i32, y: i32 },
    Multiply { x: i32, y: i32 },
    Subtract(i32, i32),
}

impl Operations {
    fn act(&self) -> i32 {
        match self {
            Operations::Add { x, y } => x + y,
            Operations::Multiply { x, y } => x * y,
            Operations::Subtract(x, y) => x - y,
        }
    }
}

fn test_enum_types() {
    //no parameter enum
    let option = Color::Red;
    match option {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Yellow => println!("Yellow"),
    }

    //c style enum
    println!("weekday->{}", WeekDay::Thursday as i32);//we can cast type cos it has a integer value

    //with parameter enum
    let add = Operations::Add { x: 3, y: 2 };
    println!("Add->{}", add.act());
}

fn test_custom_types() {
    test_struct_types();
    test_enum_types();
}

fn test_data_types() {
    test_primitive_types();
    test_custom_types();
}

fn test_variable_bindings() {
    //mutable
    let mut mutable_variable = 1;
    let immutable_variable = 2;
    //we can change mut variables after initialized,but immutable variable
    mutable_variable = 3;
    //immutable_variable=4;//error

    //scope and shadows
    let scoped_variable = 1;
    {
        let scoped_variable = '2';
        println!("shadowed: {:?}", scoped_variable);
    }
    println!("scoped: {:?}", scoped_variable);

    //declare first
    let declared_variable = 1;
    println!("declared: {:?}", declared_variable);
    //let unknown_variable;//to be safe,you must initialize this variable
    //println!("unknown_variable: {:?}", unknown_variable);//compile error,even we give it a type

    //freezing variable
    let mut freezing_variable = 1;
    {
        //after this statement,the variable is frozen in this scope
        let freezing_variable = 2;
        //freezing_variable=3;//we cannot assigned a value to it
    }
    //out the scope,assignment is ok.
    freezing_variable = 3;
    println!("{:?}", freezing_variable);
}

fn main() {
    test_data_types();
    test_variable_bindings();

    println!("Hello, world!");
}
