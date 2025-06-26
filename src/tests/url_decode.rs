use crate::url_decode;

#[test]
fn test_basic_url_decode() {
    assert_eq!(url_decode("C%3A%5CUsers%5Ctest"), "C:\\Users\\test");
    assert_eq!(url_decode("test%20file.txt"), "test file.txt");
    assert_eq!(url_decode("normal_path"), "normal_path");
}

#[test]
fn test_edge_cases() {
    // Empty string
    assert_eq!(url_decode(""), "");
    
    // Single character
    assert_eq!(url_decode("a"), "a");
    
    // Only percent signs
    assert_eq!(url_decode("%"), "%");
    assert_eq!(url_decode("%%"), "%%");
    assert_eq!(url_decode("%%%"), "%%%");
}

#[test]
fn test_invalid_hex_sequences() {
    // Invalid hex characters
    assert_eq!(url_decode("invalid%"), "invalid%");
    assert_eq!(url_decode("bad%2"), "bad%2");
    assert_eq!(url_decode("bad%2G"), "bad%2G");
    assert_eq!(url_decode("bad%G2"), "bad%G2");
    
    // Mixed valid and invalid
    assert_eq!(url_decode("good%20bad%"), "good bad%");
    assert_eq!(url_decode("good%20bad%2"), "good bad%2");
}

#[test]
fn test_common_url_encoded_chars() {
    assert_eq!(url_decode("space%20here"), "space here");
    assert_eq!(url_decode("plus%2Bsign"), "plus+sign");
    assert_eq!(url_decode("hash%23tag"), "hash#tag");
    assert_eq!(url_decode("dollar%24sign"), "dollar$sign");
    assert_eq!(url_decode("percent%25sign"), "percent%sign");
    assert_eq!(url_decode("ampersand%26sign"), "ampersand&sign");
    assert_eq!(url_decode("quote%27mark"), "quote'mark");
    assert_eq!(url_decode("double%22quote"), "double\"quote");
    assert_eq!(url_decode("less%3Cthan"), "less<than");
    assert_eq!(url_decode("greater%3Ethan"), "greater>than");
}

#[test]
fn test_windows_paths() {
    assert_eq!(url_decode("C%3A%5CUsers%5CName%5CDocuments"), "C:\\Users\\Name\\Documents");
    assert_eq!(url_decode("D%3A%5CProgram%20Files%5CApp"), "D:\\Program Files\\App");
    assert_eq!(url_decode("E%3A%5CMy%20Folder%5Cfile.txt"), "E:\\My Folder\\file.txt");
}

#[test]
fn test_unicode_characters() {
    // UTF-8 encoded characters
    assert_eq!(url_decode("caf%C3%A9"), "café");
    assert_eq!(url_decode("na%C3%AFve"), "naïve");
    assert_eq!(url_decode("r%C3%A9sum%C3%A9"), "résumé");
}

#[test]
fn test_mixed_content() {
    assert_eq!(url_decode("file%20with%20spaces%20and%20%28parentheses%29.txt"), 
               "file with spaces and (parentheses).txt");
    assert_eq!(url_decode("path%2Fwith%2Fslashes%20and%20dots%2E%2E"), 
               "path/with/slashes and dots..");
}

#[test]
fn test_consecutive_percent_sequences() {
    assert_eq!(url_decode("test%20%20double%20space"), "test  double space");
    assert_eq!(url_decode("test%25%20%25percent%25"), "test% %percent%");
}

#[test]
fn test_case_sensitivity() {
    // URL encoding should be case-insensitive for hex digits
    assert_eq!(url_decode("test%20file"), "test file");
    assert_eq!(url_decode("test%2Afile"), "test*file");
    assert_eq!(url_decode("test%2afile"), "test*file");
    assert_eq!(url_decode("test%2Afile"), "test*file");
} 