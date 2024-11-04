use ios_binding_generator::impl_ios_binding;
use ios_utilities::list::MantleList;
use ios_utilities::result::MantleResult;
use mantle_utilities::error::MantleResultError;
use mantle_utilities::string::{MantleString, MantleStringPointer};
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, c_double, c_float, c_long, c_schar, c_short, c_uchar, c_uint};

#[test]
fn handles_primitives() {
    fn test(char: i8, double: f64, schar: i8, short: i16, uchar: u8, uint: u32, long: i64) -> f32 {
        assert_eq!(char, 1);
        assert_eq!(double, 2.0);
        assert_eq!(schar, 3);
        assert_eq!(short, 4);
        assert_eq!(uchar, 5);
        assert_eq!(uint, 6);
        assert_eq!(long, 7);

        7.0
    }

    impl_ios_binding!(
        test_binding(c_char, c_double, c_schar, c_short, c_uchar, c_uint, c_long) -> c_float,
        test
    );

    let res = unsafe { test_binding(1, 2.0f64, 3, 4, 5, 6, 7) };
    assert_eq!(res, 7.0);
}

#[test]
fn handles_pointers() {
    fn test(
        str: String,
        optional_str: Option<String>,
        int: Option<u32>,
        bytes: Vec<u8>,
        optional_list: Option<Vec<u32>>,
        empty_list: Vec<f32>,
    ) -> String {
        assert_eq!(str, "test".to_string());
        assert_eq!(optional_str, None);
        assert_eq!(int, Some(42));
        assert_eq!(bytes, vec![1, 2, 3]);
        assert_eq!(optional_list, Some(vec![44]));
        assert_eq!(empty_list, Vec::<f32>::new());

        "return test".to_string()
    }

    impl_ios_binding!(
        test_binding_2(*const c_char, *const c_char = OptionalString, *const c_uint, *const MantleList<u8>, *const MantleList<u32> = OptionalPrimitiveList, *const MantleList<f32>) -> *const c_char,
        test
    );

    let test_str = MantleString("test".to_string()).to_ptr();
    let opt_test_str = std::ptr::null();
    let opt_number = Box::into_raw(Box::new(42));
    let bytes = Box::into_raw(Box::new(MantleList::from(vec![1u8, 2, 3])));
    let opt_list = Box::into_raw(Box::new(MantleList::from(vec![44u32])));
    let empty_list = Box::into_raw(Box::new(MantleList::from(vec![])));
    let res = unsafe {
        test_binding_2(
            test_str,
            opt_test_str,
            opt_number,
            bytes,
            opt_list,
            empty_list,
        )
    };
    assert_eq!("return test".to_string(), unsafe {
        MantleStringPointer(res).to_string()
    });
}

#[test]
fn handles_serialization() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        first: String,
        second: u32,
    }
    let passed = TestStruct {
        first: "1".to_string(),
        second: 1,
    };
    let serialized = serde_json::to_vec(&passed).unwrap();
    let serialized_list = serde_json::to_vec(&vec![passed]).unwrap();
    fn test(
        arg: TestStruct,
        arg2: Option<TestStruct>,
        arg3: Vec<TestStruct>,
        arg4: Option<Vec<TestStruct>>,
    ) -> Result<TestStruct, Box<dyn MantleResultError>> {
        let expected = TestStruct {
            first: "1".to_string(),
            second: 1,
        };

        assert_eq!(arg, expected);
        assert_eq!(arg2.as_ref(), Some(&expected));
        assert_eq!(arg3, vec![expected]);
        assert!(arg4.is_none());

        Ok(TestStruct {
            first: "2".to_string(),
            second: 2,
        })
    }

    impl_ios_binding!(
        test_binding_3(*const MantleList<u8> = Serialized(TestStruct), *const MantleList<u8> = OptionalSerialized(TestStruct), *const MantleList<u8> = Serialized(Vec<TestStruct>), *const MantleList<u8> = OptionalSerialized(Vec<TestStruct>)) -> MantleResult<MantleList<u8>> = FallibleSerialized(TestStruct),
        test
    );

    let serialized_arg = Box::into_raw(Box::new(MantleList::from(serialized)));
    let serialized_list = Box::into_raw(Box::new(MantleList::from(serialized_list)));
    let res = unsafe {
        test_binding_3(
            serialized_arg,
            serialized_arg,
            serialized_list,
            std::ptr::null(),
        )
    };
    let got: TestStruct = serde_json::from_slice(&match res {
        MantleResult::Success(ptr) => unsafe { (*ptr).copy_to_vec() },
        MantleResult::Fail(_) => panic!(),
    })
    .unwrap();
    assert_eq!(
        got,
        TestStruct {
            first: "2".to_string(),
            second: 2
        }
    );
}

#[test]
fn supports_fallible_functions() {
    fn test_void() -> Result<(), Box<dyn MantleResultError>> {
        Ok(())
    }

    fn test_primitive() -> Result<u32, Box<dyn MantleResultError>> {
        Ok(1)
    }

    fn test_bytes() -> Result<Vec<u8>, Box<dyn MantleResultError>> {
        Ok(vec![1, 2, 3])
    }

    fn test_string() -> Result<String, Box<dyn MantleResultError>> {
        Ok("test".to_string())
    }

    impl_ios_binding!(
        test_binding_4() -> MantleResult<()>,
        test_void
    );
    impl_ios_binding!(
        test_binding_5() -> MantleResult<c_uint>,
        test_primitive
    );
    impl_ios_binding!(
        test_binding_6() -> MantleResult<MantleList<u8>>,
        test_bytes
    );
    impl_ios_binding!(
        test_binding_7() -> MantleResult<*const c_char>,
        test_string
    );

    let res_void = unsafe { test_binding_4() };
    let res_primitive = unsafe { test_binding_5() };
    let res_bytes = unsafe { test_binding_6() };
    let res_string = unsafe { test_binding_7() };
    assert!(matches!(res_void, MantleResult::Success(_)));
    assert!(matches!(res_primitive, MantleResult::Success(ptr) if unsafe { *ptr } == 1));
    assert!(
        matches!(res_bytes, MantleResult::Success(ptr) if unsafe { (*ptr).copy_to_vec() } == vec![1, 2, 3])
    );
    assert!(
        matches!(res_string, MantleResult::Success(ptr) if unsafe { MantleStringPointer(*ptr).to_string() } == "test")
    );
}

#[test]
fn supports_infallible_functions() {
    fn test_void() {}

    fn test_primitive() -> u32 {
        1
    }

    fn test_string() -> String {
        "test".to_string()
    }

    fn test_list() -> Vec<u32> {
        vec![1]
    }

    impl_ios_binding!(test_binding_8(), test_void);
    impl_ios_binding!(
        test_binding_9() -> c_uint,
        test_primitive
    );
    impl_ios_binding!(
        test_binding_10() -> *const c_char,
        test_string
    );
    impl_ios_binding!(
        test_binding_11() -> *const MantleList<u32>,
        test_list
    );

    unsafe { test_binding_8() };
    assert_eq!(unsafe { test_binding_9() }, 1);
    assert_eq!(
        unsafe { MantleStringPointer(test_binding_10()).to_string() },
        "test"
    );
    assert_eq!(unsafe { (*test_binding_11()).copy_to_vec() }, vec![1]);
}

#[test]
fn supports_string_vec() {
    fn test(list: Vec<String>) {
        assert_eq!(list, vec!["test"]);
    }

    impl_ios_binding!(test_binding_12(*const MantleList<*const c_char>), test);

    let ptr = MantleString("test".to_string()).to_ptr();
    let list = MantleList::from(vec![ptr]);
    let list_ptr = Box::into_raw(Box::new(list));
    unsafe { test_binding_12(list_ptr) }
}
