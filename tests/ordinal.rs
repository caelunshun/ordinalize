use ordinalizer::Ordinal;

#[test]
fn basic() {
    #[derive(Copy, Clone, Ordinal)]
    enum Test {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

    assert_eq!(Test::A.ordinal(), 0);
    assert_eq!(Test::B.ordinal(), 1);
    assert_eq!(Test::C.ordinal(), 2);
    assert_eq!(Test::D.ordinal(), 3);
    assert_eq!(Test::E.ordinal(), 4);
    assert_eq!(Test::F.ordinal(), 5);
    assert_eq!(Test::G.ordinal(), 6);
}

#[test]
fn fields() {
    #[derive(Ordinal)]
    enum Test {
        A(i32),
        B {
            _named1: String,
            _named2: &'static i32,
        },
        C(i64, i32),
        D,
    }

    assert_eq!(Test::A(43223).ordinal(), 0);
    assert_eq!(
        (Test::B {
            _named1: String::from("test"),
            _named2: &10
        })
        .ordinal(),
        1
    );
    assert_eq!(Test::C(10, 0).ordinal(), 2);
    assert_eq!(Test::D.ordinal(), 3);
}
