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
        #[num_enum(num = 8)]
        Eight,
        #[num_enum(catch_all)]
        Unknown(u8),
    }
    assert_eq!(u8::from(Foo::One), 1_u8);
    assert_eq!(u8::from(Foo::Three), 3_u8);
    assert_eq!(Foo::from(3_u8), Foo::Three);
    assert_eq!(Foo::from(4), Foo::Unknown(4));
    assert_eq!(Foo::from(5), Foo::Unknown(5));
    assert_eq!(Foo::from(6), Foo::Unknown(6));
    assert_eq!(Foo::from(7), Foo::Unknown(7));
    assert_eq!(u8::from(Foo::Unknown(4_u8)), 4);
    assert_eq!(u8::from(Foo::Eight), 8);
}

#[test]
fn default_catch_all_into_prim() {
    #[derive(::num_enum::IntoPrimitive, PartialEq, Debug)]
    #[num_enum(u8)]
    enum Foo {
        #[num_enum(num = 1)]
        One(u16),
        #[num_enum(num = 2)]
        Two,
        #[num_enum(num = 3)]
        Three(String, String),
        #[num_enum(num = 8)]
        Eight(Vec<u8>),
        #[num_enum(catch_all)]
        Unknown(u8),
    }
    assert_eq!(u8::from(Foo::One(3)), 1_u8);
    assert_eq!(u8::from(Foo::Two), 2_u8);
    assert_eq!(
        u8::from(Foo::Three("foo".to_string(), "a".to_string())),
        3_u8
    );
    assert_eq!(u8::from(Foo::Unknown(4_u8)), 4);
    assert_eq!(u8::from(Foo::Eight(b"foo".to_vec())), 8);
}
