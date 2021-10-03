fn test_integer_types() {
    //signed integer
    let var_i8 = 1i8;
    let var_i16 = 2i16;
    let var_i32 = 3i32;
    let var_i64 = 4i64;
    let var_i128 = 5i128;
    let var_isize = 0isize; //pointer size, likely to be 32bit or 64bit depends on platform os
    println!(
        "signe integer->{},{},{},{},{},{}",
        var_i8, var_i16, var_i32, var_i64, var_i128, var_isize
    );

    //unsigned integer
    let var_u8 = 6u8;
    let var_u16 = 7u16;
    let var_u32 = 8u32;
    let var_u64 = 9u64;
    let var_u128 = 110u128;
    let var_usize = 14usize; //pointer size
    println!(
        "unsigned integer->{},{},{},{},{},{}",
        var_u8, var_u16, var_u32, var_u64, var_u128, var_usize
    );

    let var_default = 1; //default integer type is i32
    let var_specific: i16 = 2; //we can specific it type
    println!(
        "default integer->{},specific integer->{}",
        var_default, var_specific
    );

    //use const value,but must specific const type
    const AGE: i32 = 32;
    let var_age = AGE;
    println!(
        "const integer->{},const initialed integer->{}",
        AGE, var_age
    );

    //use literal value
    let var_hex = 0x10; //hex
    let var_bin = 0b1100_1100; //binary also can be 0b11001100
    let var_oct = 0o123; //octal
    let var_decimal = 123_456; //decimal also can be 123456
    let var_byte = b'a'; //byte for u8 only
    println!(
        "literal initialed integer->{},{},{},{},{}",
        var_hex, var_bin, var_oct, var_decimal, var_byte
    );

    //if you want change variable value,define mut variable
    let mut age = AGE; //caution:we define a new variable to override the preview variable
    age = 18; //we can chang it value
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
    println!(
        "float->max:{},min:{},nan:{},infinity:{},neg_infinity:{}",
        f32::MAX,
        f32::MIN,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY
    );
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
    println!(
        "ascii->{},unicode->{},escape->{},{}",
        var_ascii, var_unicode, var_escape, var_escape_hex
    );

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
    let var_array = [0, 1, 2, 3, 4]; //five i32 element
    let var_array_initial = [1; 10]; //10 element of 1
    println!("var_array->{:?},length->{:?}", var_array, var_array.len());
    println!(
        "var_array_initial->{:?},length->{:?}",
        var_array_initial,
        var_array_initial.len()
    );
    //mut array can change element value but type and length
    let mut var_target = [2; 5];
    var_target[0] = 3; //do not access element out of bound
    println!(
        "var_target->{:?},length->{:?}",
        var_target,
        var_target.len()
    );
    //rust don not have VLA yet,so we use vec as dynamic array
    let mut var_dynamic = vec![1, 2, 3, 4];
    var_dynamic.push(5);
    println!("dynamic array->{:?}", var_dynamic);

    //tuple
    let var_tuple = (1, 2u32, 3.0f32, 4.0123f64, 'b', true, ()); //different element type, but limited elements
    println!("{:?}", var_tuple);
    println!("tuple.0={}", var_tuple.0); //access member by indicate it's position
    //destruct tuple
    let (x, y, z, i, j, k, m) = var_tuple;
    println!("x->{},j->{}", x, j);
    //be careful on empty tuple,if tuple get one element,you should add , followed it
    let var_empty = ();
    let var_one_element = (1, );
    println!(
        "empty tuple->{:?},one element tuple->{:?}",
        var_empty, var_one_element
    );

    //range
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 }); //not include 5
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5)); //include 5
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
    println!("{:?}", &var_target[..]); //all
    println!("{:?}", &var_target[1..]); //except index 0
    println!("{:?}", &var_target[..4]); //except index 4
    println!("{:?}", &var_target[1..4]); //except index 0 and 4
    println!("{:?}", &var_target[1..=4]); //except index 0
    let var_string = ["1", "2", "3", "4"];
    println!("{:?}", &var_string[1..=3]); //except index 0
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
    let xiaohua: Student = Student {
        name: String::from("xiaohua"),
        age: 14,
        sex: Gender::Female,
        weight: 1.55,
        height: 43.2,
        class: Class(String::from("yiban"), 1, 50),
        favorite: Favorite,
    }; //initial list must indicate the member name of the element.
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
    let lilei = Student {
        name,
        age,
        sex,
        weight,
        height,
        class,
        favorite,
    };
    println!("lilei->{:?}", lilei);

    //mut variable can be modified, and we use the rest element of xiaohua to initial xiaoming
    let mut xiaoming: Student = Student {
        name: String::from("xiaoming"),
        ..xiaohua
    };
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
    let Student {
        name,
        age,
        sex,
        weight,
        height,
        class,
        favorite,
    } = xiaoming;
    //let Student { name:name, age:age, sex:sex, weight:weight, height:height, class:class, favorite:favorite } = xiaoming;//stupid to use  this statement
    println!(
        "destruct xiaoming->{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
        name, age, sex, weight, height, class, favorite
    );
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
    println!("weekday->{}", WeekDay::Thursday as i32); //we can cast type cos it has a integer value

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

#[derive(Debug)]
struct Numbers {
    max: i32,
    min: i32,
}

impl From<i32> for Numbers {
    fn from(value: i32) -> Self {
        Numbers {
            max: value,
            min: value,
        }
    }
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EventNumber(i32);

impl TryFrom<i32> for EventNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (value % 2) == 0 {
            Ok(EventNumber(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Circle {
    radius: usize,
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

use std::str::FromStr;

impl FromStr for Circle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circle {
            radius: s.parse::<usize>().unwrap(),
        })
    }
}

use std::vec;

#[allow(overflowing_literals)]
fn test_type_misc() {
    //casting
    let a_float = 3.1415;
    //let a_integer: i8 = a_float;//we cannot directly cast to different types,no implicit conversion
    let a_integer = a_float as i8; //this is ok,we explicit the type
    //let a_character = a_float as char;//invalid cast,cannot directly cast float/double to char.
    //when casting any value to unsigned type,make fits into the new type
    println!("1234 as u8->{}", 1234 as u8); //like 1234%256 in c language
    //but when significant is 1,it get truncated
    println!("-2 as u8->{}", (-2i8) as u8);
    //when casting to a signed type,if the significant bit is 1,then value is negative
    println!("128 as i8->{}", 128 as i8);
    //saturating casting.when casting from float to int,if the float exceeds the upper bound or lower bound,then value is the bound value
    println!("300.12 as u8->{}", 300.12_f32 as u8); //255
    println!("-100.22 as u8->{}", -100.22_f32 as u8); //0;
    //use unsafe methods to make a real casting,but the value might overflow or return "unsound value",be careful to use.
    unsafe {
        println!("300.12 as u8->{}", 300.12_f32.to_int_unchecked::<u8>());
        println!("-100.22 as u8->{}", (-100.22_f32).to_int_unchecked::<u8>());
    }

    //literals
    let i = 0;
    let x = 1u8;
    let y = 2u32;
    let z = 3_f32;
    let j = 4.0;
    println!("sizeof i: {:?}", std::mem::size_of_val(&i));
    println!("sizeof x: {:?}", std::mem::size_of_val(&x));
    println!("sizeof y: {:?}", std::mem::size_of_val(&y));
    println!("sizeof z: {:?}", std::mem::size_of_val(&z));
    println!("sizeof j: {:?}", std::mem::size_of_val(&j));

    //inference
    let element = 1u8;
    let mut vec = Vec::new(); //no type declared
    vec.push(element); //here vec can inferred the type
    println!("vec: {:?}", vec);

    //aliasing type
    type Integer = i32;
    let alia_int: Integer = 1; //we can use the aliasing type

    //custom type casting,which implement std::convert::From trait
    let org_str = "string";
    let org_string = String::from(org_str);

    let num_from = Numbers::from(10);
    println!("num_to: {:?}", num_from);
    let num = 3;
    //into function is the reciprocal function of from
    let num_into: Numbers = num.into(); //must specific the type
    println!("num_into:{:?}", num_into);

    //try_from
    assert_eq!(EventNumber::try_from(6), Ok(EventNumber(6)));
    assert_eq!(EventNumber::try_from(3), Err(()));
    let result: Result<EventNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EventNumber(8)));
    let result: Result<EventNumber, ()> = 3i32.try_into();
    assert_eq!(result, Err(()));

    //convert to string,implement fmt trait
    let circle = Circle { radius: 1 };
    println!("{}", circle.to_string());
    //parse a String
    let num: i32 = "2".parse().unwrap();
    let num_spec = "3".parse::<i32>().unwrap(); //specific type
    let circle: Circle = "4".parse::<Circle>().unwrap();
    println!("circle: {:?}", circle);
}

fn test_expressions_and_statements() {
    //there are so many statements in rust
    //statements are made up of expressions
    //when CPU run into function,then goes into block and scope statements,
    //CPU will execute expressions and check next character whether it is a semicolon,
    //if it is a semicolon,CPU continues executing until it get a expression value without semicolon,
    //the value is the statement or function return value
    let x = 2u32;//statement
    x;//still a statement
    x + 1;
    12;
    let y = {//assigned by a block which return a value of the expression "x+square"
        let x = 3;
        let square = x * x;
        x + square
    };
    println!("y ={}", y)
}

fn test_flow_control() {
    //if-else
    let cmp = 3;
    //single flow
    if cmp < 4 {
        println!("cmp <4:");
    }
    //with else
    if cmp > 4 {
        println!("cmp>4");
    } else {
        println!("cmp<=4");
    }
    //with multiply else if
    if cmp < 3 {
        println!("cmp<3");
    } else if cmp == 3 {
        println!("cmp=3");
    } else {
        println!("cmp>3");
    }
    //assigned by a if-else statement
    let n = 4;
    let big_num = if n < 3 || n > 3 {
        n + n//expression
    } else {
        n * n
    };
    println!("big_num:{}", big_num);

    //loop:no condition to check,control by "continue" or "break","break" can return a value or goto a label,
    //when return a value,all branches must return the same type value
    //also we can embed loops
    let mut loop_counter = 0;
    'top: loop {
        println!("top loop counter: {:?}", loop_counter);
        loop_counter += 1;
        'second: loop {
            println!("second loop counter: {:?}", loop_counter);
            loop_counter += 1;
            if loop_counter == 4 {
                continue;
            }
            'third: loop {
                println!("third loop counter: {:?}", loop_counter);
                loop_counter += 1;
                break 'top;//goto the label
            }
        }
    };
    loop_counter = 0;
    let loop_result = loop {
        loop_counter += 1;
        if loop_counter == 10 {
            break loop_counter * 2;//return a value
        }
    };
    println!("loop result: {:?}", loop_result);

    //while
    loop_counter = 0;
    while loop_counter < 10 {
        if loop_counter % 2 == 0 {
            println!("odd number find:{}", loop_counter);
        }
        loop_counter += 1;
    }

    //for loop:range based or iterator based
    for n in 1..101 {//not include 101,use 1..=101,like slice
        if n % 2 == 0 {
            println!("odd number find:{}", n);
        }
    }
    let names = vec!["bob", "frank", "mars"];
    for name in names.iter() {//this way we can use elements after.
        match name {
            &"frank" => println!("hello frank"),
            _ => println!("who is {}?", name),
        }
    }
    println!("names:{:?}", names);
    for name in names.into_iter() {//this way elements no longer available.
        match name {
            "bob" => println!("hello bob"),
            _ => println!("who is {}?", name),
        }
    }
    //println!("names:{:?}", names);//this will make a error
    let mut names = vec!["bob", "frank", "mars"];
    for name in names.iter_mut() {//this way we can change element
        *name = match name {
            &mut "mars" => "harris",
            _ => "some one",
        }
    }
    println!("names:{:?}", names);

    //match
    let choice = 10;
    match choice {
        1 => println!("the one"),
        2 | 3 | 5 | 7 => println!("these are primes"),
        8..=9 => println!("numbers i don't understand"),
        _ => println!("all the rests"),
    }
    //assigned by match
    let color_index = 0;
    let color = match color_index {
        1 => "red",
        2 => "black",
        3 => "white",
        _ => "unknown color",
    };
    println!("color: {:?}", color);
    //destruct match
    //can use to destruct tuple\enum\struct\pointer\ref
    let triple = (0, 1, 2);
    match triple {
        (0, x, y) => println!("x:{},y{}", x, y),
        (1, ..) => println!("start with 1"),
        _ => println!("we don't care"),
    }
    //pointer & reference
    let reference = &3;
    match reference {
        &val => println!("reference:{}", val),
    }
    match *reference {
        val => println!("val:{}", val),
    }
    let value = 3;
    let mut mut_value = 5;
    match value {
        ref r => println!("a reference to value{:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("mut value add 10:{}", m);
        }
    }
    //guard math
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("twins"),
        (x, y) if x + y == 0 => println!("antimatter"),
        (x, _) if x % 2 == 1 => println!("odd"),
        _ => println!("no matter"),
    }
    //binding match
    let age = 23;
    match age {
        0 => println!("not bored"),
        child @ 1..=12 => println!("child age: {}", child),
        teen @ 13..=19 => println!("teen age: {}", teen),
        adult @ 20..=100 => println!("adult age: {}", adult),
        _ => println!("monster"),
    }

    //if-let:sometimes we dont want to write complex math cases,use if-let simply
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;
    if let Some(i) = number {
        println!("number: {}", i);
    }
    if let Some(i) = letter {
        println!("letter: {}", i);
    } else {
        println!("it is a letter");
    }
    let is_letter = false;
    if let Some(i) = emotion {
        println!("emotion: {}", i);
    } else if is_letter {
        println!("it is a letter");
    } else {
        println!("it is a emotion");
    }

    //while-let:like if-let,just simplify match cases
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("9 is too big,so we quit");
            optional = None;
        } else {
            println!("continue on");
            optional = Some(i + 1);
        }
    }
}

fn main() {
    test_data_types();
    test_variable_bindings();
    test_type_misc();
    test_expressions_and_statements();
    test_flow_control();

    println!("Hello, world!");
}
