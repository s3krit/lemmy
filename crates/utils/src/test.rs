use crate::utils::{
  is_valid_community_name,
  is_valid_display_name,
  is_valid_matrix_id,
  is_valid_post_title,
  is_valid_username,
  scrape_text_for_mentions,
};

#[test]
fn test_mentions_regex() {
  let text = "Just read a great blog post by [@tedu@honk.teduangst.com](/u/test). And another by !test_community@fish.teduangst.com . Another [@lemmy@lemmy-alpha:8540](/u/fish)";
  let mentions = scrape_text_for_mentions(text);

  assert_eq!(mentions[0].name, "tedu".to_string());
  assert_eq!(mentions[0].domain, "honk.teduangst.com".to_string());
  assert_eq!(mentions[1].domain, "lemmy-alpha:8540".to_string());
}

#[test]
fn test_valid_register_username() {
  assert!(is_valid_username("Hello_98"));
  assert!(is_valid_username("ten"));
  assert!(!is_valid_username("Hello-98"));
  assert!(!is_valid_username("a"));
  assert!(!is_valid_username(""));
}

#[test]
fn test_valid_display_name() {
  assert!(is_valid_display_name("hello @there"));
  assert!(!is_valid_display_name("@hello there"));

  // Make sure zero-space with an @ doesn't work
  assert!(!is_valid_display_name(&format!(
    "{}@my name is",
    '\u{200b}'
  )));
}

#[test]
fn test_valid_community_name() {
  assert!(is_valid_community_name("example"));
  assert!(is_valid_community_name("example_community"));
  assert!(!is_valid_community_name("Example"));
  assert!(!is_valid_community_name("Ex"));
  assert!(!is_valid_community_name(""));
}

#[test]
fn test_valid_post_title() {
  assert!(is_valid_post_title("Post Title"));
  assert!(is_valid_post_title("   POST TITLE 😃😃😃😃😃"));
  assert!(!is_valid_post_title("\n \n \n \n    		")); // tabs/spaces/newlines
}

#[test]
fn test_valid_matrix_id() {
  assert!(is_valid_matrix_id("@dess:matrix.org"));
  assert!(!is_valid_matrix_id("dess:matrix.org"));
  assert!(!is_valid_matrix_id(" @dess:matrix.org"));
  assert!(!is_valid_matrix_id("@dess:matrix.org t"));
}

// These helped with testing
// #[test]
// fn test_send_email() {
//  let result =  send_email("not a subject", "test_email@gmail.com", "ur user", "<h1>HI there</h1>");
//   assert!(result.is_ok());
// }
