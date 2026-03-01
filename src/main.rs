use leptos::prelude::*; // 包含signal, Effect, set_interval, #[component], view!, mount_to_body等
use std::time::Duration;

fn get_time() -> String {
    // js_sys 通过 leptos::web_sys::js_sys 访问
    let now = leptos::web_sys::js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    )
}

#[component]
fn Clock() -> impl IntoView {
    // 返回类型固定为IntoView
    // signal创建一个响应式状态，返回一个读写对
    // time -> ReadSignal<String>,用来读取值
    // set_time -> WriteSignal<String>,用来更新值
    // 初始值是调用get_time()的结果
    let (time, set_time) = signal(get_time());

    // set_interval 是 leptos::prelude 内置的工具函数，封装了浏览器的 setInterval
    // 第一个参数是闭包（每次触发时执行的逻辑）
    // 第二个参数是间隔时长，使用标准库的 Duration
    // move 关键字让闭包【获得】set_time 的所有权，而不是借用
    // 因为这个闭包的生命周期比当前函数更长，所以必须用 move
    set_interval(move || set_time.set(get_time()), Duration::from_secs(1));

    // view! 宏让你在 Rust 里写类似 HTML 的模板
    // 编译期会把它转换成构建真实 DOM 的 Rust 代码
    view! {
        <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            font-family: monospace;
            background: #1a1a2e;
            color: #e0e0e0;
        ">
            // 【重要】纯文字内容必须加引号，否则编译器会把它当 Rust 标识符报错
            <h1 style="font-size: 1.5rem; margin-bottom: 1rem; color: #888;">
                "当前时间"
            </h1>
            // {} 是插值语法，里面放实现了 IntoView 的任意值
            // 这里放的是 ReadSignal，Leptos 会自动追踪它
            // 每当 time 信号更新，【只有这一处】DOM 节点会精确更新
            // 不会重新执行整个组件函数，也不需要虚拟 DOM diff
            <div style="font-size: 5rem; font-weight: bold; color: #00d4ff; letter-spacing: 0.1em;">
                {time}
            </div>
        </div>
    }
}

fn main() {
    // mount_to_body 把 Leptos 应用挂载到 <body> 上，作为入口
    // 里面用 view! 宏创建根组件，写法和在模板里使用组件一样
    // <Clock /> 就是调用上面定义的 Clock 组件
    mount_to_body(|| view! { <Clock /> })
}
