
use proc_playground::*;


// macro_rules! herp_macro {
//     () => {
//         struct Poopy {
//             ballsacks: String
//         }
//     };
// }

// macro_caller!(herp_macro);


#[derive(Debug, Derp)]
        pub enum SomeDerpStructDeriver {
            #[gribothy_variant_value = "123u8"]
            First,
            #[gribothy_variant_value = "8u8..=22u8 | 43u8..=87u8"]
            Second(u8),
        }

// #[add_fields(df)]
// pub struct Dingus {
//     pub initial_field: bool
// }

            #[add_fields(Append)]
            pub struct Dingus {
                pub initial_field: bool,
            }

            // no_way_expand!();
            // pub struct Debugable {
            //     pub derpily: String
            // }

pub enum ENumerStest {
    Stupid(u8),
    Dumber(i32)
}







fn main() {

    // let hi = ENumerStest::Stupid(3);
    // let nope = hi.1;




    // let raw = "
    //     #[add_fields(df)]
    //         pub struct Dingus {
    //             pub initial_field: bool
    //         }
    //     ";


    //  let raw = "
    //     #[derive(Debug,Derp)]
    //     pub struct SomeDerpStructDeriver {
    //         #[derpy = 123]
    //         pub byte: u8,
    //         pub unattred: String,
    //     }
    //     ";
    //     let input : DeriveInput = syn::parse_str(raw).unwrap();
    //     backend_derive_derp(&input);
}
