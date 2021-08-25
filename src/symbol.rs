use std::fmt;
use std::fmt::Formatter;

#[allow(unused_attributes, non_upper_case_globals)]
macro_rules! symbols {
    ( $( $ndex: expr => $sym: ident ),* ) => {
        #[allow(non_upper_case_globals, dead_code)]
        const Unknown: &'static str = "";
        $( const $sym: &'static str = stringify!($sym); )*

        #[derive(Clone, Copy, Hash, Eq, PartialEq)]
        pub enum Symbol {
           $( $sym, )*
            Unknown
        }

        impl<'s> From<&'s str> for Symbol {
            fn from(value: &'_ str) -> Self {
                match value {
                    $( $sym => Self::$sym, )*
                    _ => Self::Unknown,
                }
            }
        }

        impl<'s> From<&Symbol> for &'s str {
            fn from(s: &Symbol) -> &'s str {
                match *s {
                    $( Symbol::$sym => $sym, )*
                    Symbol::Unknown => "",
                }
            }
        }

        impl<'s> From<Symbol> for &'s str {
            fn from(s: Symbol) -> &'s str {
                match s {
                    $( Symbol::$sym => $sym, )*
                    Symbol::Unknown => "",
                }
            }
        }

        impl Into<u8> for Symbol {
            fn into(self) -> u8 {
                match self {
                    $( Symbol::$sym => $ndex, )*
                    Symbol::Unknown => 0x7F,
                }
            }
        }

        impl fmt::Display for Symbol {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.into())
            }
        }

        impl fmt::Debug for Symbol {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str(self.into())
            }
        }
    }
}

symbols! {
    0x01 => BTC,
    0x02 => ETH,
    0x03 => KCS,
    0x04 => USDT,
    0x05 => UNO,
    0x06 => APL,
    0x07 => TRTL,
    0x08 => MATIC,
    0x09 => ENJ,
    0xa0 => STORJ,
    0xa1 => FIL,
    0xa2 => POL,
    0xa3 => ELON,
    0xa4 => ETHO,
    0xa5 => BTT,
    0xa6 => LSS,
    0xa7 => THETA,
    0xa8 => VEED
}