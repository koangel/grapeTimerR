```
                        _______               ___ 
  ___ ________ ____  __/_  __(_)_ _  ___ ____/ _ \
 / _ `/ __/ _ `/ _ \/ -_) / / /  ' \/ -_) __/ , _/
 \_, /_/  \_,_/ .__/\__/_/ /_/_/_/_/\__/_/ /_/|_| 
/___/        /_/                                   grapeTimerR

```
[![made-with-Rust](https://img.shields.io/badge/made%20with-rust-red)](https://www.rust-lang.org/)
[![Open Source Love svg2](https://badges.frapsoft.com/os/v2/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)

## **简介 Intro**

A coarse-grained time scheduler can help you create time tasks quickly and simply through some series.

一款粗粒度的时间调度器，可以帮你通过一些字符串快速并简单的创建时间任务。

grapeTimer的Rust版本，提供相同功能以及相同类型的服务。

[![grapeTimerR](https://img.shields.io/badge/grapeTimerR-rust-blue)](https://github.com/koangel/grapeTimerR)

## **功能 Feature**
- 通过命令格式创建`std::chrono::Datetime`(Created by date format)
- 简洁的Api格式，轻度且可拆分的函数库(Simple Api, light and detachable library)
- 快速创建调度器(Quickly create a scheduler)
- 可控的调度器时间粒度`[需要提前指定]`(Controllable scheduler time granularity `[Need to specify in advance]`)
- 多线程调度模式(Multi-thread scheduling mode)
- 时间周期，次数多模式可控`[支持每天、每周、每月]`(Time period, multi-mode controllable number of times `[support daily, weekly, monthly]`)
- 可以获取下一次执行时间`[Chrono Datetime]`(Can get the string of the next execution time)
- 自定义起始TimerId的种子(Customize the seed of the starting TimerId)
- 自定义TimerId的生成函数`[自生成ID请注意并发场景下的线程争抢]`(Custom TimerId generation trait `[Self-generated ID, please pay attention to thread contention in concurrent scenarios]`)
- TimerId扩展为i64，支持大ID和timestampId生成器(TimerId is i64, supporting large Id and timestampId generator correspondence)

## **日期格式 Date Format**

The scheduler has a light date pattern analysis system, which can provide daily, weekly, and monthly time and date generation methods

调度器有轻度的日期模式分析体系，可提供每日，每周，每月的时间日期生成方式，具体格式如下：

|关键字 Key|格式 Format |说明 Description|
|:----------:|:-------:|:----------:|
|Day|Day 00:00:00|生成每日的日期时间|
|Week|Week 1 00:00:00|生成每周的日期时间， 0~6 分别代表周日到周六 |
|Month|Month 1 00:00:00|生成每月该日期的时间，建议不要使用28日之后的日期|

|Key|Format |Description|
|:----------:|:-------:|:----------:|
|Day|Day 00:00:00|Generate daily date and time|
|Week|Week 1 00:00:00|Generate weekly date and time, 0~6 represent Sunday to Saturday|
|Month|Month 1 00:00:00|The time when the date of the month was generated, it is recommended not to use the date after the 28th|

## **Examples**

**parser date format**

```rust
use grapeTimerR::parsers;
// get next tick Datetime
let next_day = parsers::parser_next("Day 05::00:00").unwrap();
// get next timestamp
let next_dayTime = parsers::parser_timestamp("Day 05::00:00").unwrap();

// utc
let next_dayUtc = parsers::parser_nextUtc("Day 05::00:00").unwrap();
let next_dayTimeUtc = parsers::parser_timestampUtc("Day 05::00:00").unwrap();
```

**add ticker**

```rust

```

**add ticker by trait mode**

```rust

```

## **鸣谢(Thanks)**

Use Jetbrains Ide for project

[![saythanks](https://img.shields.io/badge/say-thanks-ff69b4.svg)](https://saythanks.io/to/kennethreitz)
[![Generic badge](https://img.shields.io/badge/JetBrains-Goland-<COLOR>.svg)](https://shields.io/)
[![Generic badge](https://img.shields.io/badge/JetBrains-CLion-<COLOR>.svg)](https://shields.io/)

