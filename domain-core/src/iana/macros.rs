//! Macros making implementing IANA types easier.

/// Creates a standard IANA type wrapping an integer.
///
/// This adds impls for `From`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and
/// `Hash`.
/// 
/// For `FromStr` and `Display`, see one of the other macros in this module.
macro_rules! int_enum {
    ( $(#[$attr:meta])* =>
      $ianatype:ident, $inttype:path;
      $( $(#[$variant_attr:meta])* ( $variant:ident =>
                                        $value:expr, $mnemonic:expr) )* ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug)]
        pub enum $ianatype {
            $( $(#[$variant_attr])* $variant ),*,
            
            /// A raw value given through its integer.
            Int($inttype)
        }

        impl $ianatype {
            /// Returns a value from its raw integer value.
            pub fn from_int(value: $inttype) -> Self {
                match value {
                    $( $value => $ianatype::$variant ),*,
                    _ => $ianatype::Int(value)
                }
            }

            /// Returns the raw integer value for a value.
            pub fn to_int(self) -> $inttype {
                match self {
                    $( $ianatype::$variant => $value ),*,
                    $ianatype::Int(value) => value
                }
            }

            /// Returns a value from a well-defined mnemonic.
            pub fn from_mnemonic(m: &[u8]) -> Option<Self> {
                $(
                    if m.eq_ignore_ascii_case($mnemonic) {
                        return Some($ianatype::$variant)
                    }
                )*
                None
            }

            /// Returns the mnemonic for this value if there is one.
            ///
            /// This will also return a mnemonic if a well-defined variant
            /// is hidden in a `Int` variant.
            pub fn to_mnemonic(self) -> Option<&'static [u8]> {
                match self {
                    $( $ianatype::$variant => Some($mnemonic) ),*,
                    $ianatype::Int(value) => {
                        match $ianatype::from_int(value) {
                            $ianatype::Int(_) => None,
                            value => value.to_mnemonic()
                        }
                    }
                }
            }
        }


        //--- From

        impl From<$inttype> for $ianatype {
            fn from(value: $inttype) -> Self {
                $ianatype::from_int(value)
            }
        }

        impl From<$ianatype> for $inttype {
            fn from(value: $ianatype) -> Self {
                value.to_int()
            }
        }

        impl<'a> From<&'a $ianatype> for $inttype {
            fn from(value: &'a $ianatype) -> Self {
                value.to_int()
            }
        }


        //--- Parse and Compose

        impl $crate::bits::parse::Parse for $ianatype {
            type Err = $crate::bits::parse::ShortBuf;

            fn parse(parser: &mut $crate::bits::parse::Parser)
                     -> Result<Self, Self::Err> {
                <$inttype as $crate::bits::parse::Parse>::parse(parser)
                    .map(Self::from_int)
            }

            fn skip(parser: &mut $crate::bits::parse::Parser)
                    -> Result<(), Self::Err> {
                <$inttype as $crate::bits::parse::Parse>::skip(parser)
            }
        }

        impl $crate::bits::compose::Compose for $ianatype {
            fn compose_len(&self) -> usize {
                <$inttype as $crate::bits::compose::Compose>
                    ::compose_len(&self.to_int())
            }

            fn compose<B: ::bytes::BufMut>(&self, buf: &mut B) {
                $crate::bits::compose::Compose::compose(&self.to_int(), buf)
            }
        }


        //--- PartialEq and Eq

        impl PartialEq for $ianatype {
            fn eq(&self, other: &Self) -> bool {
                self.to_int() == other.to_int()
            }
        }

        impl PartialEq<$inttype> for $ianatype {
            fn eq(&self, other: &$inttype) -> bool {
                self.to_int() == *other
            }
        }

        impl PartialEq<$ianatype> for $inttype {
            fn eq(&self, other: &$ianatype) -> bool {
                *self == other.to_int()
            }
        }

        impl Eq for $ianatype { }


        //--- PartialOrd and Ord

        impl PartialOrd for $ianatype {
            fn partial_cmp(&self, other: &Self)
                           -> Option<::std::cmp::Ordering> {
                self.to_int().partial_cmp(&other.to_int())
            }
        }

        impl PartialOrd<$inttype> for $ianatype {
            fn partial_cmp(&self, other: &$inttype)
                           -> Option<::std::cmp::Ordering> {
                self.to_int().partial_cmp(other)
            }
        }

        impl PartialOrd<$ianatype> for $inttype {
            fn partial_cmp(&self, other: &$ianatype)
                           -> Option<::std::cmp::Ordering> {
                self.partial_cmp(&other.to_int())
            }
        }

        impl Ord for $ianatype {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.to_int().cmp(&other.to_int())
            }
        }


        //--- Hash

        impl ::std::hash::Hash for $ianatype {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.to_int().hash(state)
            }
        }
    }
}

/*
/// Adds impls for `FromStr` and `Display` to the type given as first argument.
///
/// The `FromStr` impl matches only well known mnemonics ignoring case,
/// otherwise it returns an error of the second argument.
///
/// For `Display`, it will display a decimal number for values without
/// mnemonic.
macro_rules! int_enum_str_mnemonics_only {
    ($ianatype:ident, $error:expr) => {
        impl ::std::str::FromStr for $ianatype {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // We assume all mnemonics are always ASCII, so using
                // the bytes representation of `s` is safe.
                $ianatype::from_mnemonic(s.as_bytes()).ok_or(FromStrError)
            }
        }

        impl ::std::fmt::Display for $ianatype {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
                   -> ::std::fmt::Result {
                use ::std::fmt::Write;

                match self.to_mnemonic() {
                    Some(m) => {
                        for ch in m {
                            try!(f.write_char(*ch as char))
                        }
                        Ok(())
                    }
                    None => {
                        write!(f, "{}", self.to_int())
                    }
                }
            }
        }

        from_str_error!($error);
    }
}
*/

/// Adds impls for `FromStr` and `Display` to the type given as first argument.
///
/// For `FromStr`, recognizes only the decimal values. For `Display`, it will
/// only print the decimal values.
macro_rules! int_enum_str_decimal {
    ($ianatype:ident, $inttype:ident) => {
        
        impl $ianatype {
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                ::std::str::from_utf8(bytes).ok().and_then(|r| {
                    $inttype::from_str_radix(r, 10).ok()
                        .map($ianatype::from_int)
                })
            }
        }

        impl ::std::str::FromStr for $ianatype {
            type Err = ::std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $inttype::from_str_radix(s, 10).map($ianatype::from_int)
            }
        }

        impl ::master::scan::Scan for $ianatype {
            fn scan<C: ::master::scan::CharSource>(
                scanner: &mut ::master::scan::Scanner<C>
            ) -> Result<Self, ::master::scan::ScanError> {
                scanner.scan_string_word(|word| {
                    use ::std::str::FromStr;

                    Self::from_str(&word)
                         .map_err(::master::scan::SyntaxError::content)
                })
            }
        }

        impl ::std::fmt::Display for $ianatype {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter
            ) -> ::std::fmt::Result {
                write!(f, "{}", self.to_int())
            }
        }
    }
}


/// Adds impls for `FromStr` and `Display` to the type given as first argument.
///
/// For `FromStr`, recognizes all mnemonics case-insensitively as well as a
/// decimal number representing any value.
///
/// For `Display`, it will display a decimal number for values without
/// mnemonic.
macro_rules! int_enum_str_with_decimal {
    ($ianatype:ident, $inttype:ident, $error:expr) => {
        impl $ianatype {
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                $ianatype::from_mnemonic(bytes).or_else(|| {
                    ::std::str::from_utf8(bytes).ok().and_then(|r| {
                        $inttype::from_str_radix(r, 10).ok()
                            .map($ianatype::from_int)
                    })
                })
            }
        }
                                        
        impl ::std::str::FromStr for $ianatype {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // We assume all mnemonics are always ASCII, so using
                // the bytes representation of `s` is safe.
                match $ianatype::from_mnemonic(s.as_bytes()) {
                    Some(res) => Ok(res),
                    None => {
                        if let Ok(res) = $inttype::from_str_radix(s, 10) {
                            Ok($ianatype::Int(res))
                        }
                        else {
                            Err(FromStrError)
                        }
                    }
                }
            }
        }

        impl ::std::fmt::Display for $ianatype {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
                   -> ::std::fmt::Result {
                use ::std::fmt::Write;

                match self.to_mnemonic() {
                    Some(m) => {
                        for ch in m {
                            try!(f.write_char(*ch as char))
                        }
                        Ok(())
                    }
                    None => {
                        write!(f, "{}", self.to_int())
                    }
                }
            }
        }

        impl ::master::scan::Scan for $ianatype {
            fn scan<C: ::master::scan::CharSource>(
                scanner: &mut ::master::scan::Scanner<C>
            ) -> Result<Self, ::master::scan::ScanError> {
                scanner.scan_string_word(|word| {
                    use ::std::str::FromStr;

                    Self::from_str(&word)
                         .map_err(|_| {
                             ::master::scan::SyntaxError::UnknownMnemonic
                         })
                })
            }
        }

        from_str_error!($error);
    }
}

/// Adds impls for `FromStr` and `Display` to the type given as first argument.
///
/// For `FromStr` recognizes all defined mnemonics ignoring case. Additionally
/// recognizes a value starting with the prefix given in the second argument
/// (again, ignoring case) directly followed by a decimal number.
///
/// For `Display`, values without mnemonic will be written starting with the
/// prefix directly followed by the decimal representation of the value.
macro_rules! int_enum_str_with_prefix {
    ($ianatype:ident, $str_prefix:expr, $u8_prefix:expr, $inttype:ident,
     $error:expr) => {
        impl $ianatype {
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                $ianatype::from_mnemonic(bytes).or_else(|| {
                    if bytes.len() <= $u8_prefix.len() {
                        return None
                    }
                    let (l, r) = bytes.split_at($u8_prefix.len());
                    if !l.eq_ignore_ascii_case($u8_prefix) {
                        return None
                    }
                    let r = match ::std::str::from_utf8(r) {
                        Ok(r) => r,
                        Err(_) => return None
                    };
                    u16::from_str_radix(r, 10).ok().map($ianatype::from_int)
                })
            }
        }

        impl ::std::str::FromStr for $ianatype {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // We assume all mnemonics are always ASCII, so using
                // the bytes representation of `s` is safe.
                match $ianatype::from_mnemonic(s.as_bytes()) {
                    Some(res) => Ok(res),
                    None => {
                        if let Some((n, _)) = s.char_indices()
                                               .nth($str_prefix.len()) {
                            let (l, r) = s.split_at(n);
                            if l.eq_ignore_ascii_case($str_prefix) {
                                let value = match u16::from_str_radix(r, 10) {
                                    Ok(x) => x,
                                    Err(..) => return Err(FromStrError)
                                };
                                Ok($ianatype::from_int(value))
                            }
                            else {
                                Err(FromStrError)
                            }
                        }
                        else {
                            Err(FromStrError)
                        }
                    }
                }
            }
        }

        impl ::std::fmt::Display for $ianatype {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
                   -> ::std::fmt::Result {
                use ::std::fmt::Write;

                match self.to_mnemonic() {
                    Some(m) => {
                        for ch in m {
                            try!(f.write_char(*ch as char))
                        }
                        Ok(())
                    }
                    None => {
                        write!(f, "{}{}", $str_prefix, self.to_int())
                    }
                }
            }
        }

        impl ::master::scan::Scan for $ianatype {
            fn scan<C: ::master::scan::CharSource>(
                scanner: &mut ::master::scan::Scanner<C>
            ) -> Result<Self, ::master::scan::ScanError> {
                scanner.scan_string_word(|word| {
                    use ::std::str::FromStr;

                    Self::from_str(&word)
                         .map_err(|_| {
                             ::master::scan::SyntaxError::UnknownMnemonic
                         })
                })
            }
        }

        from_str_error!($error);
    }
}

macro_rules! from_str_error {
    ($description:expr) => {
        #[derive(Clone, Debug)]
        pub struct FromStrError;

        impl ::std::error::Error for FromStrError {
            fn description(&self) -> &str {
               $description
            }
        }

        impl ::std::fmt::Display for FromStrError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
                   -> ::std::fmt::Result {
                $description.fmt(f)
            }
        }
    }
}
