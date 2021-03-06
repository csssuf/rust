// compile-flags:-Z unstable-options --static-root-path /cache/

// @has static_root_path/struct.SomeStruct.html
// @matches - '"/cache/main\.js"'
// @!matches - '"\.\./main\.js"'
// @matches - '"\.\./search-index\.js"'
// @!matches - '"/cache/search-index\.js"'
pub struct SomeStruct;

// @has src/static_root_path/static-root-path.rs.html
// @matches - '"/cache/source-script\.js"'
// @!matches - '"\.\./\.\./source-script\.js"'
// @matches - '"\.\./\.\./source-files.js"'
// @!matches - '"/cache/source-files\.js"'
