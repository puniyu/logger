# Changelog

## [0.6.2](https://github.com/puniyu/logger/compare/v0.6.1...v0.6.2) (2025-11-06)


### 🔧 其他更新

* **deps:** update rust crate convert_case to 0.9.0 ([#31](https://github.com/puniyu/logger/issues/31)) ([ccaf06a](https://github.com/puniyu/logger/commit/ccaf06ace5e2e37f958929362ddd8e1d132d5b28))
* **deps:** update rust crate tracing-shared to 0.2.0 ([#29](https://github.com/puniyu/logger/issues/29)) ([bd51c49](https://github.com/puniyu/logger/commit/bd51c49b9833ae7b7cfbd81a33cd7456d66ce574))

## [0.6.1](https://github.com/Puniyu/logger/compare/v0.6.0...v0.6.1) (2025-10-12)


### 🔧 其他更新

* **deps:** 更新依赖并移除未使用包 ([2a29d12](https://github.com/Puniyu/logger/commit/2a29d12f3a26fbdb53d7d4c5ecada58a7d80aea5))
* **release:** 更新发布工作流配置 ([f819ba4](https://github.com/Puniyu/logger/commit/f819ba44b421931b504b4409e700260efabaf9ab))

## [0.6.0](https://github.com/Puniyu/logger/compare/v0.5.6...v0.6.0) (2025-10-11)


### ✨ 新功能

* **tracing:** 添加环境过滤功能支持 ([3f8114d](https://github.com/Puniyu/logger/commit/3f8114d39bdb961410a67b8f02f5ed6d272a9334))


### 🔧 其他更新

* **ci:** 更新发布工作流配置 ([4403265](https://github.com/Puniyu/logger/commit/44032654b70f9ca7b9361598943364870494ddd7))
* **deps:** update actions/checkout action to v5 ([18e496d](https://github.com/Puniyu/logger/commit/18e496dafdda0f332950fd0bf7e54006e24e19d8))
* **deps:** update rust crate owo-colors to v4.2.3 ([36dc794](https://github.com/Puniyu/logger/commit/36dc7946bd2127ec189add97854004457d9a5293))
* **deps:** 更新 renovate 配置以优化自动化合并策略 ([66ec42f](https://github.com/Puniyu/logger/commit/66ec42f9dc9fafbeea912d496a1273df84779e93))
* **deps:** 添加 renovate 配置文件 ([9160327](https://github.com/Puniyu/logger/commit/916032775cd7a190c14a92c438662864c51e1638))

## [0.5.6](https://github.com/Puniyu/logger/compare/v0.5.5...v0.5.6) (2025-09-24)


### ♻️ 代码重构

* **logger:** 调整日志前缀格式生成逻辑 ([68edec4](https://github.com/Puniyu/logger/commit/68edec4082e658055cb9bda9eac01827811d7d8d))

## [0.5.5](https://github.com/Puniyu/logger/compare/v0.5.4...v0.5.5) (2025-09-24)


### ♻️ 代码重构

* **logger:** 支持自定义日志前缀格式 ([6f68865](https://github.com/Puniyu/logger/commit/6f68865314874428d39883ff2dda53fd5402bef9))

## [0.5.4](https://github.com/Puniyu/logger/compare/v0.5.3...v0.5.4) (2025-09-17)


### ♻️ 代码重构

* **logger:** 优化日志模块初始化流程 ([76be7fc](https://github.com/Puniyu/logger/commit/76be7fcb5d69c9d9446db0df13deb6327e66ecd4))

## [0.5.3](https://github.com/Puniyu/logger/compare/v0.5.2...v0.5.3) (2025-09-16)


### 🐛 错误修复

* **logger:** 修复日志颜色导致的垃圾值问题 ([c9f16a0](https://github.com/Puniyu/logger/commit/c9f16a0be56a75f9bd5c3ef322ee1cc416e07b6d))

## [0.5.2](https://github.com/Puniyu/logger/compare/v0.5.1...v0.5.2) (2025-09-16)


### 📦️ 构建系统

* 添加 owo_colors 依赖 ([46fc324](https://github.com/Puniyu/logger/commit/46fc324d5245ee39375860beda77dcd4c9bbb30c))

## [0.5.1](https://github.com/Puniyu/logger/compare/v0.5.0...v0.5.1) (2025-09-14)


### ⚡️ 性能优化

* **logger:** 移除日志 span 事件 ([78c0e07](https://github.com/Puniyu/logger/commit/78c0e072c65d5cff6a4430f6e9d7380ed506daa7))

## [0.5.0](https://github.com/Puniyu/logger/compare/v0.4.0...v0.5.0) (2025-09-14)


### ✨ 新功能

* **tracing:** 添加 span 事件并更新模块导出 ([4501ab0](https://github.com/Puniyu/logger/commit/4501ab02cf1694203bf9a81d838a1a6614c2aa61))

## [0.4.0](https://github.com/Puniyu/logger/compare/v0.3.0...v0.4.0) (2025-09-14)


### ✨ 新功能

* **tracing:** 添加 Level 枚举导入 ([7ae90fa](https://github.com/Puniyu/logger/commit/7ae90fa59afaf3849fa525c30e5df8b86b26f553))

## [0.3.0](https://github.com/Puniyu/logger/compare/v0.2.3...v0.3.0) (2025-09-14)


### ✨ 新功能

* **tracing:** 添加 span 和 event 的导入 ([d3f1bf8](https://github.com/Puniyu/logger/commit/d3f1bf82e8b02c2b56b3353ba830e37eb63b49d3))

## [0.2.3](https://github.com/Puniyu/logger/compare/v0.2.2...v0.2.3) (2025-09-14)


### ♻️ 代码重构

* **logger:** 移除日志等级动态修改功能 ([51dafe4](https://github.com/Puniyu/logger/commit/51dafe4e90e81769eabe8abd81b6754fb16903c9))

## [0.2.2](https://github.com/Puniyu/logger/compare/v0.2.1...v0.2.2) (2025-09-09)


### ♻️ 代码重构

* 更新 tracing_shared 的引用 ([ba65d5f](https://github.com/Puniyu/logger/commit/ba65d5fb64dcbabcef370161b33f83e6ce415ec4))

## [0.2.1](https://github.com/Puniyu/logger/compare/v0.2.0...v0.2.1) (2025-09-09)


### 📦️ 构建系统

* **dependencies:** 更新依赖并添加 tracing-shared ([e6d2504](https://github.com/Puniyu/logger/commit/e6d2504cec9c0e3f0c28bc95b0f9abb72f35b4f3))

## [0.2.0](https://github.com/Puniyu/logger/compare/v0.1.5...v0.2.0) (2025-09-07)


### ✨ 新功能

* 导出 tracing 模块的日志宏 ([44d7f54](https://github.com/Puniyu/logger/commit/44d7f5400cf7286213a675b8440beb2de7c89e73))

## [0.1.5](https://github.com/Puniyu/logger/compare/v0.1.4...v0.1.5) (2025-09-06)


### 🐛 错误修复

* 修复log create无法使用 ([8325419](https://github.com/Puniyu/logger/commit/83254198e1a4234fbbb26aeaf73e8de44b31eed5))

## [0.1.4](https://github.com/Puniyu/logger/compare/v0.1.3...v0.1.4) (2025-09-06)


### ♻️ 代码重构

* **logger:** 重构 LoggerOptions 构造方式 ([541d471](https://github.com/Puniyu/logger/commit/541d471dcdd409bdfd2dffe8d3dd8c5e3463b34b))

## [0.1.3](https://github.com/Puniyu/logger/compare/v0.1.2...v0.1.3) (2025-09-06)


### ♻️ 代码重构

* **logger:** 重构日志初始化函数 ([da0032d](https://github.com/Puniyu/logger/commit/da0032d3c679301e3ee9a514dd8b25e133fcf80d))

## [0.1.2](https://github.com/Puniyu/logger/compare/v0.1.1...v0.1.2) (2025-09-06)


### ♻️ 代码重构

* **logger:** 优化日志初始化流程 ([f64d7ec](https://github.com/Puniyu/logger/commit/f64d7ecc90b15398b759b850cef72bc7dec59cdb))

## [0.1.1](https://github.com/Puniyu/logger/compare/v0.1.0...v0.1.1) (2025-09-05)


### 🔧 其他更新

* 初始化仓库 ([d806e9f](https://github.com/Puniyu/logger/commit/d806e9f794d960f7642af3cf527ad8ae3f2f963c))


### 🎡 持续集成

* 添加 release-please 配置和工作流 ([de4c3ec](https://github.com/Puniyu/logger/commit/de4c3ec74a3cbd5b674b534834bda0e1d5f7b190))
