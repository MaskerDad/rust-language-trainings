#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

fn test_trait() {
    trait Signed {
        fn is_strictly_negative(self) -> bool;
    }

    //impl self.trait for self.type
    impl Signed for Number {
        fn is_strictly_negative(self) -> bool {
            self.value < 0
        }
    }

    //impl self.trait for outer.type
    impl Signed for i32 {
        fn is_strictly_negative(self) -> bool {
            self < 0
        }
    }

    //impl outer.trait for self.type
    impl std::ops::Neg for Number {
        type Output = Number;
        
        fn neg(self) -> Number {
            Number {
                value: -self.value,
                odd: self.odd,
            }
        }
    }

    let n = Number { odd: false, value: -44 };
    let m: i32 = -44;
    
    println!("Number(-44): {}", n.is_strictly_negative());//true
    println!("i32(-44): {}", m.is_strictly_negative());//true
    
    let b = Number { odd: false, value: 36 };
    let a = -b;
    println!("(-n).value = {}", a.value);//-36
}

fn test_generic() {
    let n = Number { odd: false, value: -44 };
    let _a = n;
    let _b = n;

    use std::fmt::Debug;
    //generic function
    fn compare<T>(left: T, right: T)
    where
        T: Debug + PartialEq,
    {
        println!("{:?} {} {:?}",
                 left,
                 if left == right {"=="}
                 else {"!="},
                 right);
    }
    compare("tea", "coffee");
    compare("tea", "tea");
    //generic strcut: Pair<T>
    struct Pair<T> {
        a: T,
        b: T,
    }
    fn print_type_name<T>(_val: &T) {
        println!("{}", std::any::type_name::<T>());
    }
    let p1 = Pair { a: 1, b: 2 };
    let p2 = Pair { a: true, b: false };
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    v1.push(1);
    v2.push(true);
    print_type_name(&p1);
    print_type_name(&p2);
    print_type_name(&v1);
    print_type_name(&v2);
}

fn test_enum_option_result() {
   /*
      enum Option<T> {
        None,
        Some<T>,
      }

      impl<T> Option<T> {
        fn unwrap(self) -> T {
            match self{
                Self::Some(t) => t,
                Self::None => panic!("..."),
            }
        }
      }

    * */ 
    let o1: Option<i32> = Some(128);
    let o2: Option<i32> = None;
    o1.unwrap();
    //o2.unwrap();
    
    /*
       enum Result<T, E> {
            Ok(T),
            Err(E),
       }
     * */
}

fn test_lifetime() {
   //引用的生命周期不能超过它所借用(引用)的变量绑定的生命周期
}

fn test_closure() {
    fn for_each_planet<F>(f: F)
    where F: Fn(&'static str) {
        f("aaa");
        f("bbb");
        f("ccc");
    }
    for_each_planet(|planet| println!("Hello, {}", planet));

    //a closure that have two params
    fn foobar<F>(x: i32, y: i32, is_greater: F)
        where F: Fn(i32, i32) -> bool
    {
        let (greater, smaller) = if is_greater(x, y) {
            (x, y)
        } else {
            (y, x)
        };
        println!("{} is greater than {}", greater, smaller);
    }

    foobar(32, 64, |x, y| x > y);
    foobar(97, 77, |x, y| x > y);
}

fn main() {
    test_trait();
    test_generic();
    test_enum_option_result();
    test_closure();

}
