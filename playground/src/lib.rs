
use proc_playground::*;


bidirectional_enum! {
    pub enum NoWay : u8 {
        MultiRanged <=> 0u8..=8u8 | 9u8..=11u8,
        SingleLiteral <=> 111u8,
        SingleRange <=> 22u8..=99u8,
        MultiValued <=> 100u8 | 102u8..=105u8,
        RangeThenLiteral <=> 106u8..=109u8 | 110u8,
    }
}
pub trait Derp {}

// #[derive(Debug, Derp)]
//         pub enum SomeDerpStructDeriver {
//             #[gribothy_variant_value = "123u8"]
//             First,
//             #[gribothy_variant_value = "8u8..=22u8 | 43u8..=87u8"]
//             Second(u8),
//         }


// macro_rules! bidirectional_enum {
//     (@as_expr $e:expr) => {$e};
//     (@as_pat $($p:pat)|+) => {$($p)|+};

//     ((@variant $var:ident sing $val:expr), $T:ty) => {
//         $var
//     };

//     ((@variant $var:ident mult $($patterns:pat)|+), $T:ty) => {
//         $var($T)
//     };

//     (
//         @collect_variants ($name:ident $T:ty),
//         ($(,)*) -> ($($out:tt)*)
//     ) => {
//         // stringify!(pub $name : $T {
//             // $(
//             //     bidirectional_enum! {
//             //         @variant
//             //         $out,
//             //         $T
//             //     }
//             // )*
//     };


//       (
//         @collect_variants $fixed:tt,
//         ($var:ident <=> $val:expr, $($tail:tt)*) -> ($($variants:tt)*)
//     ) => {
//         bidirectional_enum! {
//             @collect_variants $fixed,
//             ($($tail)*) -> ($($variants)* ($var sing $val),)
//         }
//     };

//     (
//         @collect_variants $fixed:tt,
//         ($var:ident <=> $($patterns:pat)|+, $($tail:tt)*) -> ($($variants:tt)*)
//     ) => {
//         bidirectional_enum! {
//             @collect_variants $fixed,
//             ($($tail)*) -> ($($variants)* ($var mult $($patterns)|+), )
//         }
//     };

  


//     (
//         pub enum $name:ident : $T:ty {$($body:tt)*}
//     ) => {
//         bidirectional_enum! {
//             @collect_variants
//             ($name $T), ($($body)*,) -> ()
//         }
//     };
// }





stupid_expander!();
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_test() {
        #[derive(Debug,Derp)]
        
        pub struct SomeDerpStructDeriver {
            #[gribothy_variant_value = 123]
            pub byte: u8,
            pub unattred: String,
        }
    }
}
