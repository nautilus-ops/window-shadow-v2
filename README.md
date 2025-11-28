# window shadows v2

This crate is a drop-in alternative to [window-shadows](https://github.com/tauri-apps/window-shadows) for Tauri v2.

Since Tauri v2 can natively enable and disable shadows, the original window-shadows crate is no longer maintained.

## Why is this needed?

My app uses a custom title bar. When shadows are enabled on Windows 10, the top edge shows the following bug:

![img.png](docs/images/img.png)

The top shadow fails to render, leaving a white strip. Applying the native shadow approach from the original window-shadows crate fixes the issue.

## Usage

```rust
.setup(|app| {
    window_shadows_v2::set_shadows(app, true);
})
```
