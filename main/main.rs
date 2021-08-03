fn main() {

    // mut
    let mut mutability = 32; // mut : 변수의 변형
    println!("The value of mutability : {}", mutability);
    mutability = 6;
    println!("The value of mutability : {}", mutability);
    println!("");

    // const
    /*
    상수, immutable 변수의 차이점
        상수 : variable shadowing 불가능
        let a = 1;
        let a = a*a;
        const b: u32 = 3;
        const b: u32 = b*b; // errir[E0428]
        상수 : global scope 사용 가능
        let version = "1.0.0"; // error
        const VERSION: &'static str = "1.0.0";
        fn main() {
            ...
        }
        상수 : 타입 명시 필요
        let inch = 23.3
        const E = 2.71; // error
        const PI: f64 = 3.14;
        
        상수 : constant expression으로만 초기화 가능 (함수 반환값, 런타임에 결정되는 값 사용불가)
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        const fn plus_one_const(x:i32) -> i32 {
            x + 1
        }
        fn main() {
            const N1: i32 = plus_one(3); // error
            const N2: i32 = plus_one_const(3);
        }
    */

    // const
    const MAX_POINTS: u32 = 100_000;
    println!("The MAX_PONTS : {}", MAX_POINTS);
    println!("");

    // Shdowing
    let shdowing = 4;
    let shdowing = shdowing + 1;
    let shdowing = shdowing * 2;

    println!("The value of shdowing : {}", shdowing);

    let spaces = "   "; // 문자열 유형
    let spaces = spaces.len(); // 첫 번째 것과 동일한 이름을 가진 새롭게 정의된 숫자 유형의 변수
    println!("The spaces : {}", spaces);
    println!("");

    // Data Types
    let scalar_string = "TutorialsPoint";  // string type
    let scalar_float = 4.5;                 // float type
    let scalar_boolean = true;          // boolean type
    let scalar_char = '♥';                    //unicode character type

    println!("scalar_string : {}",scalar_string);
    println!("scalar_float : {}",scalar_float);
    println!("scalar_boolean : {}",scalar_boolean);
    println!("scalar_char : {}",scalar_char);
    println!("");

    /*
    스칼라 타입
        정수형
        부호 없는 32비트 변수 : u32 (Unsigned)
        부호 있는 32비트 변수 : i32 (Signed)
        ex  :  u8 타입 : 0 에서 2^8 - 1, 0 에서 255 까지의 값을 저장
        정수형 리터럴
        Decimal 
        Hex ex) 0xff
        Octal ex) 0o77
        Binary ex) 0b1111_0000
        Byte (u8 only) ex) b'A'
        부동 소수점
        f32 : 32bit
        f64 : 64bit
        Boolean
        let t = true;
        let t:bool = false;
        문자
        let c = 'z';
        let z = 'Z';
        let black_heart = '🖤';
    */

    // let tuples: (i32, f64, u8) = (500, 5.3, 1);
    // let (tuples_i32, tuples_f64, tuples_u8) = tuples;

    // println!("tuples_f64 : {}", tuples_f64);

    // // destructuring
    // let destructuring_tuples: (i32, f64, u8) = (500, 5.3, 1);

    // let destructuring_tuples_i32 = destructuring_tuples.0;
    // let destructuring_tuples_f64 = destructuring_tuples.1;
    // let destructuring_tuples_u8 = destructuring_tuples.2;
    // println!("destructuring_tuples_i32 : {}", destructuring_tuples_i32);
    // println!("destructuring_tuples_f64 : {}", destructuring_tuples_f64);
    // println!("destructuring_tuples_u8 : {}", destructuring_tuples_u8);
}