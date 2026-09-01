<div align="center">

# 🧠 NeuroVault 中文版

**本地优先的 AI 记忆系统** —— 让 Claude 及任何 MCP 智能体跨会话记住你。
笔记就是普通 Markdown 文件，所有数据存在本地，不联网、不上传。

*An MIT-licensed Chinese-language fork of [sirdath/NeuroVault](https://github.com/sirdath/NeuroVault).*

[![License: MIT](https://img.shields.io/badge/License-MIT-2f7bf6.svg)](LICENSE)

> ✍️ 本文档由 AI 助手撰写，如有笔误欢迎在 [Issues](https://github.com/johngsai/NeuroVault-Chinese/issues) 指正。

</div>

这是 [sirdath/NeuroVault](https://github.com/sirdath/NeuroVault) 的 fork，保留原项目全部能力，并针对**中文用户**做了三项优化：

- 🇨🇳 **中文嵌入模型** —— BAAI/bge-small-zh-v1.5，首次运行自动下载
- 🇨🇳 **中文分词检索** —— jieba-rs，BM25 能正确切分并命中中文关键词
- 🇨🇳 **中文人名 / 实体识别** —— jieba 词性标注，图谱与检索对中文更友好

用法与原版完全一致，**无需任何额外配置**。

## ⬇️ 快速安装（macOS · Apple Silicon · macOS 14+）

到 [Releases](https://github.com/johngsai/NeuroVault-Chinese/releases/latest) 下载 `NeuroVault_*_aarch64.dmg`，打开后把 App 拖入「应用程序」即可 —— **不用安装任何编译环境**。

> ⚠️ 本仓库的 DMG 为本地构建，**未经 Apple 签名公证**。首次打开若被 Gatekeeper 拦截：
> 1. 右键点击 App →「打开」；或
> 2. 终端执行 `xattr -dr com.apple.quarantine /Applications/NeuroVault.app`
>
> 需要官方签名版，可到 [上游 Releases](https://github.com/sirdath/NeuroVault/releases/latest) 下载。

**Windows / Linux / Intel Mac** 暂无预编译包，请从源码构建（见下）。

## 🔧 从源码构建

```bash
git clone https://github.com/johngsai/NeuroVault-Chinese.git
cd NeuroVault-Chinese
npm install
npx tauri dev      # 开发运行
npx tauri build    # 打包安装包（产物在 src-tauri/target/release/bundle/）
```

## 📖 详细文档

使用说明、MCP 接入、架构等完整文档见上游：[neurovault.dathproject.com/docs](https://neurovault.dathproject.com/docs)

## ⚖️ 许可与致谢

[MIT License](LICENSE)。原项目及版权归 [sirdath](https://github.com/sirdath) 所有；本 fork 仅在其基础上增加中文支持。Bug 与建议请到 [Issues](https://github.com/johngsai/NeuroVault-Chinese/issues) / [Discussions](https://github.com/johngsai/NeuroVault-Chinese/discussions) 反馈。
