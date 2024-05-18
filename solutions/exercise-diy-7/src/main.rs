/* 
Write a program to check the validity of password input by users.
Following are the criteria for checking the password:
1. At least 1 letter between [a-z]
2. At least 1 number between [0-9]
3. At least 1 letter between [A-Z]
4. At least 1 character from [$#@]
5. Minimum length of transaction password: 6
6. Maximum length of transaction password: 12
*/

//Expected output 

/*
    ###### Set New Password ######
    1. At least 1 letter between [a-z]
    2. At least 1 number between [0-9]
    3. At least 1 letter between [A-Z]
    4. At least 1 character from [$#@]
    5. Minimum length of transaction password: 6
    6. Maximum length of transaction password: 12

    Enter New Password: <User enters the password>
    Re-enter New Password: <User re-enters the same password>
    Password is Weak/Strong/Do not match
    Password change successful/unsuccessful 
 */

 //tasks to be completed 
 //1. take user input 
 //2. check the passwd strength..COMPLETED
 //3. match the passwords ...COMPLETED
 //4. output the status

use std::io;

fn main() {
    let (passwd, re_entered_passwd) = promt_passwd_change_message();

    println!("{}",print_status(
        check_password_strength(&passwd), 
        is_password_match(&passwd, &re_entered_passwd)
    ));
}

 fn promt_passwd_change_message() -> (String, String) {
    println!(
        r##"
###### Set New Password ######
1. At least 1 letter between [a-z]
2. At least 1 number between [0-9]
3. At least 1 letter between [A-Z]
4. At least 1 character from [$#@]
5. Minimum length of transaction password: 6
6. Maximum length of transaction password: 12
        "##
    );

    println!(
        "Enter New Password:"
    ); 
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd).expect("Error while reading from stdin");

    let mut re_enter_passwd = String::new();
    println!(
        "Re-enter New Password:"
    );
    io::stdin().read_line(&mut re_enter_passwd).expect("Error while reading from stdin");
 
    (passwd.trim().to_string(), re_enter_passwd.trim().to_string())

 }


 fn is_password_match(passwd_1: &str, passwd_2: &str) -> bool {
    passwd_1 == passwd_2
 }

 fn is_password_contain_lower_case_letters(passwd: &str) -> bool {
    for ch in passwd.chars() {
        if ch.is_ascii_lowercase() {
            return true;
        }
    }

    false
 }

 fn is_password_contain_upper_case_letters(passwd: &str) -> bool {
    for ch in passwd.chars() {
        if ch.is_ascii_uppercase() {
            return true;
        }
    }

    false
 }

 fn is_password_contains_digits(passwd: &str) -> bool {
    for c in passwd.chars() {
        if c.is_ascii_digit() {
            return true;
        }
    }
    
    false
}

fn is_allowed_special_chars(c: char) -> bool{
    c == '$' || c == '@' || c == '#'
}

fn is_password_contains_allowed_special_chars(passwd: &str) -> bool {
    for c in passwd.chars() {
        if is_allowed_special_chars(c) {
            return true;
        }
    }
    
    false
}


fn is_password_contains_illegal_special_chars(passwd: &str) -> bool {
    for c in passwd.chars() {
        if c.is_ascii_alphanumeric() || is_allowed_special_chars(c) {
            continue;
        } else {
            return true;
        }
    }
    
    false
}

fn is_password_length_valid(passwd: &str) -> bool {
    (passwd.len() >= 6) && (passwd.len() <= 12)
}

 fn check_password_strength(passwd: &str) -> i32 {
    //weak = 0, strong = 1
   if is_password_contain_lower_case_letters(passwd) && 
      is_password_contain_upper_case_letters(passwd) &&
      is_password_contains_digits(passwd) &&
      is_password_contains_allowed_special_chars(passwd) &&
      !is_password_contains_illegal_special_chars(passwd) &&
      is_password_length_valid(passwd) {
        1
    } else {
        0
    }

 }

 fn print_status(weak_or_strong: i32, match_status: bool) -> &'static str {
   
    if !match_status {
        "Password do not match\nPassword change was unsuccessful"
    }else if weak_or_strong == 1 {
        "Password is Strong\nPassword change was successful"
    } else {
        "Password is Weak\nPassword change was un-successful"
    }
    
 }

 #[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_when_both_the_passwords_are_same_fn_returns_true() {
        assert_eq!(true, is_password_match("abc", "abc"));
    }

    #[test]
    fn test_when_both_the_passwords_are_not_same_fn_returns_false() {
        assert_eq!(false, is_password_match("abc", "abcd"));
    }

    mod passwd_strength {
        use crate::*;

        #[test]
        fn test_when_password_is_empty_string_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength(""));
        }

        #[test]
        fn test_when_password_contains_only_a_to_z_letters_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("abc"));
        }

        #[test]
        fn test_when_password_contains_only_A_to_Z_letters_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("ABC"));
        }

        #[test]
        fn test_when_password_contains_only_digits_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("123"));
        }

        #[test]
        fn test_when_password_length_is_less_than_6_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("123"));
        }

        #[test]
        fn test_when_password_length_is_greater_than_12_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("abcdEF@123456789"));
        }

        #[test]
        fn test_when_password_contains_lower_case_letters_fn_returns_true() {
            assert_eq!(true, is_password_contain_lower_case_letters("abc"));
        }

        #[test]
        fn test_when_password_does_not_contain_lower_case_letters_fn_returns_false() {
            assert_eq!(false, is_password_contain_lower_case_letters("ABC123"));
        }

        #[test]
        fn test_when_password_contains_any_upper_case_letters_fun_returns_true() {
            assert_eq!(true, is_password_contain_upper_case_letters("12345A"));
        }

        #[test]
        fn test_when_password_contains_no_upper_case_letters_fun_returns_false() {
            assert_eq!(false, is_password_contain_upper_case_letters("12345abc###"));
        }

        #[test]
        fn test_when_password_contains_illegal_special_chars_fun_retuns_true() {
            assert_eq!(true, is_password_contains_illegal_special_chars("12345abc###_"));
        }

        #[test]
        fn test_when_password_does_not_contain_illegal_special_chars_fun_retuns_false() {
            assert_eq!(false, is_password_contains_illegal_special_chars("12345abc###"));
        }

        #[test]
        fn test_when_passwords_match_and_it_is_strong_print_strong_and_successful() {
            assert_eq!(
                "Password is Strong\nPassword change was successful",
                print_status(1, true)
            );
        }

        #[test]
        fn test_when_weak_passwords_match_print_weak_and_un_successful() {
            assert_eq!(
                "Password is Weak\nPassword change was un-successful",
                print_status(0, true)
            );
        }


    }
}













































