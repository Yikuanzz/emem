#[test]
fn it_works() {
    use mini_grep::search;
    
    let query = "nobody";
    let contents = "I'm nobody! Who are you?";
    
    assert_eq!(vec!["I'm nobody! Who are you?"], search(query, contents));
}