# window shadows v2

本项目是 [window-shadows](https://github.com/tauri-apps/window-shadows) 这个crate在 tauri v2 上的平替方案。

由于在tauri v2已经支持 禁用和启用 阴影，因此旧版本的 window-shadows 已经停止维护。

## Why we need it?

由于我的应用使用了自定义Titlebar，在Windows 10上开启shadows后会出现以下的bug。

![img.png](docs/images/img.png)

最上方的shadow不起作用，会有一条白色的边，因此我尝试了使用 window-shadows 这个库中的方法添加了原生阴影，并成功解决了我的问题

## Usage

```rust
.setup(|app| {
    window_shadows_v2::set_shadows(app, true);
})
```
