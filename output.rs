#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use itertools::Itertools;
use std::collections::HashMap;

#[allow(unused_imports)]
use std::{env, fs};

// mod day_1;
// mod day_10;
// mod day_11;
// mod day_12;
mod day_13 {

    // mod day_2;
    // mod day_3;
    // mod day_4;
    // mod day_6;
    // mod day_9;

    // #[allow(dead_code)]
    // fn do_day_one() {
    //     let contents = fs::read_to_string("./src/1.in").expect("File is not there or unable to read");

    //     let contents: Vec<&str> = contents.split("\n").collect();

    //     let day_one_solution = day_1::solution(contents);

    //     println!(
    //         "Elf with max calories: [{}] \nTop three elfs total: [{}]\n",
    //         day_one_solution.max_calories, day_one_solution.top_three_elfs
    //     );
    // }

    // #[allow(dead_code)]
    // fn do_day_two() {
    //     let contents = fs::read_to_string("./src/2.in").expect("File is not there or unable to read");
    //     let contents: Vec<&str> = contents.split("\n").collect();

    //     let (day_two_solution_one, day_two_solution_two) = day_2::solution(contents);

    //     println!(
    //         "Play score first strat: [{}]. \nPlayer score second strat: [{}]\n",
    //         day_two_solution_one, day_two_solution_two
    //     )
    // }

    // #[allow(dead_code)]
    // fn do_day_three() {
    //     let contents = fs::read_to_string("./src/3.in").expect("File is not there or unable to read");
    //     let contents: Vec<&str> = contents.split("\n").collect();

    //     let (ans_one, ans_two) = day_3::solution(contents);

    //     println!(
    //         "Sum of prios: [{}]. \nPrios of elf groups of 3: [{}]\n",
    //         ans_one, ans_two
    //     )
    // }

    // #[allow(dead_code)]
    // fn do_day_four() {
    //     let contents = fs::read_to_string("./src/4.in").expect("File is not there or unable to read");
    //     let contents: Vec<&str> = contents.split("\n").collect();

    //     let (ans_one, ans_two) = day_4::solution(contents);

    //     println!(
    //         "Fully in range: [{}]. \nFully or partially in range: [{}]\n",
    //         ans_one, ans_two
    //     )
    // }

    // #[allow(dead_code)]
    // fn do_day_six() {
    //     let contents = fs::read_to_string("./src/6.in").expect("File is not there or unable to read");

    //     let (ans_one, ans_two) = day_6::solution(contents);

    //     println!(
    //         "Message packet size [ 4]: [{}]. \nMessage packet size [14]: [{}]",
    //         ans_one, ans_two
    //     );
    // }

    // #[allow(dead_code)]
    // fn do_day_nine() {
    //     let contents = fs::read_to_string("./src/9.in").expect("File is not there or unable to read");

    //     let ans = day_9::solution(contents);

    //     println!("Tail visited: [{ans}] unique positions",);
    // }

    // #[allow(dead_code)]
    // fn do_day_ten() {
    //     let contents = fs::read_to_string("./src/10.in").expect("File is not there or unable to read");

    //     let ans = day_10::solution(contents);

    //     println!("\nSum of singal strenghts: [{ans}]",);
    // }

    // fn do_day_eleven() {
    //     let contents = fs::read_to_string("./src/11.in").expect("File is not there or unable to read");

    //     let ans = day_11::solution(contents);

    //     println!("\n Monkey: [{ans}]",);
    // }

    // fn do_day_twelve() {
    //     let contents = fs::read_to_string("./src/12.in").expect("File is not there or unable to read");

    //     let ans = day_12::solution(contents);

    //     println!("\n Shortest path: [{ans}]",);
    // }




    // hm.insert(1, do_day_one);
    // hm.insert(2, do_day_two);
    // hm.insert(3, do_day_three);
    // hm.insert(4, do_day_four);
    // hm.insert(6, do_day_six);
    // hm.insert(9, do_day_nine);
    // hm.insert(10, do_day_ten);
    // hm.insert(11, do_day_eleven);
    // hm.insert(12, do_day_twelve);





    #![allow(dead_code)]
    use core::fmt;
    use std::cmp::Ordering;
    use serde::Deserialize;
    #[serde(untagged)]
    enum PacketValue { Single(usize), List(Vec<PacketValue>), }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            #[allow(unused_extern_crates, clippy :: useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PacketValue {
                fn deserialize<__D>(__deserializer: __D)
                    -> _serde::__private::Result<Self, __D::Error> where
                    __D: _serde::Deserializer<'de> {
                    let __content =
                        <_serde::__private::de::Content as
                                    _serde::Deserialize>::deserialize(__deserializer)?;
                    let __deserializer =
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content);
                    if let _serde::__private::Ok(__ok) =
                                _serde::__private::Result::map(<usize as
                                            _serde::Deserialize>::deserialize(__deserializer),
                                    PacketValue::Single) {
                            return _serde::__private::Ok(__ok);
                        }
                    if let _serde::__private::Ok(__ok) =
                                _serde::__private::Result::map(<Vec<PacketValue> as
                                            _serde::Deserialize>::deserialize(__deserializer),
                                    PacketValue::List) {
                            return _serde::__private::Ok(__ok);
                        }
                    _serde::__private::Err(_serde::de::Error::custom("data did not match any variant of untagged enum PacketValue"))
                }
            }
        };
    #[automatically_derived]
    impl ::core::clone::Clone for PacketValue {
        #[inline]
        fn clone(&self) -> PacketValue {
            match self {
                PacketValue::Single(__self_0) =>
                    PacketValue::Single(::core::clone::Clone::clone(__self_0)),
                PacketValue::List(__self_0) =>
                    PacketValue::List(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PacketValue { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PacketValue {
        #[inline]
        fn eq(&self, other: &PacketValue) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag &&
                match (self, other) {
                    (PacketValue::Single(__self_0),
                        PacketValue::Single(__arg1_0)) => *__self_0 == *__arg1_0,
                    (PacketValue::List(__self_0), PacketValue::List(__arg1_0))
                        => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PacketValue { }
    #[automatically_derived]
    impl ::core::cmp::Eq for PacketValue {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
            let _: ::core::cmp::AssertParamIsEq<Vec<PacketValue>>;
        }
    }
    impl fmt::Debug for PacketValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Single(n) => f.write_fmt(format_args!("{0}", n)),
                Self::List(n) => f.debug_list().entries(n).finish(),
            }
        }
    }
    impl PacketValue {
        fn get_vec(&self) -> Vec<PacketValue> {
            match self {
                Self::List(n) => n.clone(),
                Self::Single(n) =>
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([Self::Single(*n)])),
            }
        }
    }
    impl PartialOrd for PacketValue {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match (self, other) {
                (Self::Single(a), Self::Single(b)) => a.partial_cmp(&b),
                (left, right) => {
                    let l_vec = left.get_vec();
                    let r_vec = right.get_vec();
                    Some(l_vec.iter().zip(r_vec.iter()).map(|(a, b)|
                                        a.cmp(b)).find(|&x|
                                    x !=
                                        Ordering::Equal).unwrap_or_else(||
                                l_vec.len().cmp(&r_vec.len())))
                }
            }
        }
    }
    impl Ord for PacketValue {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    pub fn solution(input: String) -> usize {
        let mut ans = 0usize;
        for (idx, groups) in input.split("\n\n").enumerate() {
            let mut packets =
                groups.lines().map(|line|
                        serde_json::from_str::<PacketValue>(line).unwrap());
            let left = packets.next().unwrap();
            let right = packets.next().unwrap();
            let is_correct = left < right;
            {
                ::std::io::_print(format_args!("Group {0} L={1:?} R={2:?} is in right order {3}\n",
                        idx, left, right, is_correct));
            };
            if is_correct { ans += idx + 1; }
        }
        ans
    }
}
fn do_day_thirteen() {
    let contents =
        fs::read_to_string("./src/13.test").expect("File is not there or unable to read");
    let ans = day_13::solution(contents);
    {
        ::std::io::_print(format_args!("\n Sum of correct packet indexes: [{0}]\n",
                ans));
    };
}
fn main() {
    let mut hm: HashMap<usize, fn() -> ()> = HashMap::new();
    hm.insert(13, do_day_thirteen);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
            {
                ::core::panicking::panic_fmt(format_args!("Not enough args"));
            };
        }
    let what = &args[1];
    if what == "all" {
            for (&day_num, &f) in hm.iter().sorted() {
                {
                    ::std::io::_print(format_args!("===DAY {0}===\n\n",
                            day_num));
                };
                f();
            }
        } else {
           if let Some(elem) = hm.get(&what.parse::<usize>().unwrap()) {
                   {
                       ::std::io::_print(format_args!("===DAY {0}===\n\n", what));
                   };
                   elem();
               } else {
                  {
                      ::core::panicking::panic_fmt(format_args!("not implemented: {0}",
                              format_args!("Nothing for day ${0}", what)));
                  };
              }
       }
}
