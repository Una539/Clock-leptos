# Leptos Clock

一个用 Rust + Leptos 构建的极简实时时钟网页应用，可用于学习，也有一定价值。

## 预览

https://Una539.github.io/Clock-leptos/

## 技术栈

- [Rust](https://www.rust-lang.org/)
- [Leptos](https://leptos.dev/) 0.7
- [Trunk](https://trunkrs.dev/)

## 本地运行

安装依赖：

​```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
​```

启动开发服务器：

​```bash
trunk serve
​```

打开浏览器访问 `http://localhost:8080`

## 构建

​```bash
trunk build --release --public-url /Clock-leptos/
​```

编译结果在 `dist/` 目录。
```

## 许可证
本项目基于 [GPL-3.0](LICENSE) 许可证开源。
