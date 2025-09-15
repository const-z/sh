#[macro_export]
macro_rules! sh_test {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() {
            if stringify!($body).replace(" ", "") == "{}" {
                assert!(false, "{} - \x1b[31mПустой тест!\x1b[0m", stringify!($name));
            }
            $body
        }
    };
}

#[cfg(test)]
#[allow(unnameable_test_items)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_with_valid_test() {
        sh_test!(test_macro_with_valid_test, {
            assert_eq!(2 + 2, 4);
            assert!(true);
        });

        test_macro_with_valid_test();
    }

    #[test]
    #[should_panic]
    fn test_with_empty_body() {
        sh_test!(test_macro_with_empty_body, {});

        test_macro_with_empty_body();
    }
}
