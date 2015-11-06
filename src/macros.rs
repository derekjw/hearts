#[macro_use]

macro_rules! string_enum {
    ($name:ident { $($value:ident,)* }) => {
        string_enum! {
            $name {
                $($value => $value,)*
            }
        }
    };

    ($name:ident { $($value:ident => $string:tt,)* }) => {

        #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
        pub enum $name {
            $($value,)*
        }

        impl $name {
            #[allow(dead_code)]
            pub fn all() -> ::std::collections::BTreeSet<$name> {
                use self::$name::*;
                vec!($($value,)*).into_iter().collect()
            }
        }

        impl From<$name> for &'static str {
            fn from(entity: $name) -> &'static str {
                From::from(&entity)
            }
        }

        impl From<$name> for String {
            fn from(entity: $name) -> String {
                From::from(&entity)
            }
        }

        impl <'a> From<&'a $name> for &'static str {
            fn from(entity: &'a $name) -> &'static str {
                use self::$name::*;
                match *entity {
                    $($value => stringify!($string),)*
                }
            }
        }

        impl <'a> From<&'a $name> for String {
            fn from(entity: &'a $name) -> String {
                Into::<&str>::into(entity).to_owned()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::error::Error;

            fn from_str(string: &str) -> ::error::Result<$name> {
                use self::$name::*;
                match string {
                    $(stringify!($string) => Ok($value),)*
                    _ => Err(::error::Error::parsing(stringify!($name), string))
                }
            }
        }

    };
}
