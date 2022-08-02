// Guard against https://github.com/illicitonion/num_enum/issues/27
mod alloc {}
mod core {}
mod num_enum {}
mod std {}

#[test]
fn default() {
    #[derive(Debug, Eq, PartialEq, ::num_enum::Default)]
    #[repr(u8)]
    enum Enum {
        #[allow(unused)]
        Zero = 0,
        #[num_enum(default)]
        NonZero = 1,
    }

    assert_eq!(Enum::NonZero, <Enum as ::core::default::Default>::default());
}

#[test]
fn default_standard_default_attribute() {
    #[derive(Debug, Eq, PartialEq, ::num_enum::Default)]
    #[repr(u8)]
    enum Enum {
        #[allow(unused)]
        Zero = 0,
        #[default]
        NonZero = 1,
    }

    assert_eq!(Enum::NonZero, <Enum as ::core::default::Default>::default());
}

#[test]
fn default_catch_all() {
    #[derive(::num_enum::FromPrimitive, ::num_enum::IntoPrimitive, PartialEq, Debug)]
    #[num_enum(u8)]
    enum Foo {
        #[num_enum(num = 1)]
        One,
        #[num_enum(num = 3)]
        Three,
        #[num_enum(num = 4)]
        Four,
        #[num_enum(catch_all)]
        Unknown(u8),
    }
    assert_eq!(u8::from(Foo::One), 1_u8);
    assert_eq!(u8::from(Foo::Three), 3_u8);
    assert_eq!(Foo::from(3_u8), Foo::Three);
    assert_eq!(Foo::from(5), Foo::Unknown(5));
    assert_eq!(Foo::from(7), Foo::Unknown(7));
    assert_eq!(u8::from(Foo::Unknown(4_u8)), 4);
}
