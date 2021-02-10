```

```
[![made-with-Rust](https://img.shields.io/badge/made%20with-rust-red)](https://www.rust-lang.org/)
[![Open Source Love svg2](https://badges.frapsoft.com/os/v2/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)

## **简介 Intro**

一款粗粒度的时间调度器，可以帮你通过一些字符串快速并简单的创建时间任务。

用于游戏服务端的优化设计，大量并行的时间调度方式。

目前可支持任意类型的函数(无返回值)以及任意参数数量和参数类型。

grapeTimer的Rust版本，提供相同功能以及相同类型的服务。

[![grapeTimer](https://img.shields.io/badge/grapeTimer-go-blue)](https://github.com/koangel/grapeTimer)

## **功能 Feature**
- 通过命令格式创建`std::chrono::Datetime`
- 简洁的Api格式，轻度且可拆分的函数库
- 快速创建调度器
- 可控的调度器时间粒度`[需要提前指定]`
- 高性能的并发调度
- 时间周期，次数多模式可控`[支持每天、每周、每月]`
- 可以获取下一次执行时间的字符串`[支持自定义格式]`
- 可选择对调度器保存或内存执行
- 生成可保存的调度器字符串并反向分析他生成调度器`[保存到Json再通过Json创建Timer]`
- 处理Panic打印相关的数据信息，用于记录崩溃原因
- 自定义起始TimerId的种子
- 自定义TimerId的生成函数`[自生成ID请注意并发场景下的线程争抢]`
- TimerId扩展为int64，支持大ID和CRC64ID生成器对应



## **鸣谢(Thanks)**

Use Jetbrains Ide for project

[![saythanks](https://img.shields.io/badge/say-thanks-ff69b4.svg)](https://saythanks.io/to/kennethreitz)
[![Generic badge](https://img.shields.io/badge/JetBrains-Goland-<COLOR>.svg)](https://shields.io/)
[![Generic badge](https://img.shields.io/badge/JetBrains-CLion-<COLOR>.svg)](https://shields.io/)

