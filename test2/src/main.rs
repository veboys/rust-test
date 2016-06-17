fn main() {
    struct Point{ x:i32, y:i32};
    let mut point = Point{ x:10, y:20};
    
    println!("x:{}, y:{}", point.x, point.y);

    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;


    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    
    struct EmptyStruct;
    let empty = EmptyStruct;

    #[derive(Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d::default();
    let point = Point3d { y: 1, .. origin};
    let Point3d { x: x0, y: y0, ..} = point;

    println!("x:{}, y:{}", x0, y0);
    
}
