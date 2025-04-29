# ✨ BetterWx-Sterter ✨

基于 Rust 的 实现的批量启动小玩具
支持自动排序、自动登录

## ⚙️ 功能说明

- 请编辑 `conf.ini` 配置文件。

## 📜 conf.ini 说明

```ini
# 程序的列表 program{n} = 程序路径

program = D:\app.exe
program1 = D:\app1.exe
program2 = D:\app2.exe
program3 = D:\app3.exe

# 自动点击登录按钮 true 或者 false
# 会向窗口发送 回车 按键 ，实现自动登录

auto_login = true

# 是否先关闭所有程序 true 或者 false

close_first = true
```

