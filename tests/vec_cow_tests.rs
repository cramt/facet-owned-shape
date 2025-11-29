use facet_owned_shape::vec_cow::VecCow;

#[test]
fn test_vec_cow_from_slice() {
    let data = vec![1, 2, 3];
    let slice = &data[..];
    let cow: VecCow<[i32]> = slice.into();

    match cow {
        VecCow::Borrowed(s) => assert_eq!(s, slice),
        VecCow::Owned(_) => panic!("Expected Borrowed"),
    }

    assert_eq!(cow.as_ref(), slice);
}

#[test]
fn test_vec_cow_from_vec() {
    let data = vec![1, 2, 3];
    let cow: VecCow<[i32]> = data.clone().into();

    match cow {
        VecCow::Borrowed(_) => panic!("Expected Owned"),
        VecCow::Owned(ref v) => assert_eq!(v, &data),
    }

    assert_eq!(cow.as_ref(), &data[..]);
}

#[test]
fn test_vec_cow_clone() {
    let data = vec![1, 2, 3];
    let slice = &data[..];
    let cow: VecCow<[i32]> = slice.into();

    let cloned = cow.clone();
    match cloned {
        VecCow::Borrowed(s) => assert_eq!(s, slice),
        VecCow::Owned(_) => panic!("Expected Borrowed after clone of Borrowed"),
    }

    let cow_owned: VecCow<[i32]> = data.clone().into();
    let cloned_owned = cow_owned.clone();
    match cloned_owned {
        VecCow::Borrowed(_) => panic!("Expected Owned after clone of Owned"),
        VecCow::Owned(ref v) => assert_eq!(v, &data),
    }
}

#[test]
fn test_vec_cow_debug() {
    let data = vec![1, 2, 3];
    let cow: VecCow<[i32]> = (&data[..]).into();
    let debug_str = format!("{:?}", cow);
    assert!(debug_str.contains("Borrowed"));
    assert!(debug_str.contains("[1, 2, 3]"));

    let cow_owned: VecCow<[i32]> = data.into();
    let debug_str_owned = format!("{:?}", cow_owned);
    assert!(debug_str_owned.contains("Owned"));
    assert!(debug_str_owned.contains("[1, 2, 3]"));
}

#[test]
fn test_static_slice_of_strs() {
    let slice: &'static [&'static str] = &["a", "b"];
    let cow: VecCow<[&str]> = slice.into();
    match cow {
        VecCow::Borrowed(s) => assert_eq!(s, slice),
        VecCow::Owned(_) => panic!("Expected Borrowed"),
    }
}
