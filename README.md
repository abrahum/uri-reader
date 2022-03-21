A simple URI reader

## Usage

```rust
uri_reader::uget("http://example.com/");
uri_header::uget_with_header("https://example.com/", [("User-Agent","Rust")].into());
uri_header::uget("base64://a_base64_str");
uri_header::uget("file:///path/to/file");
```

## Supported schemes

- [x] `http`
- [x] `https`
- [x] `base64`
- [x] `file`
- [ ] `ftp`

## Notes

The `file` scheme didn't support `\\` in path. Use `/` instead.