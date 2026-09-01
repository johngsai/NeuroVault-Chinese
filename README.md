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

## 📖 使用说明

1. **启动**：打开 NeuroVault，先创建/选择一个记忆库（Brain）。所有数据都在本地 `~/.neurovault/`，笔记就是普通 `.md` 文件，随时可导出或备份。
2. **记录**：在「Memories」里直接写笔记，或让接入的 AI 帮你写；把文件拖进窗口可导入。
3. **接入 AI（MCP，核心用法）**：App 内 **Settings → Connections** 可一键配置；或手动配置 Kilo Code / Pi / DSHarness 等客户端（见下方「连接 AI 客户端」）。之后它们就能跨会话回忆你的记忆。
4. **浏览与回忆**：「Today」看最近动态，「Graph」看记忆之间的关联图谱，搜索框检索历史记忆。

> 完整图文教程见上游文档：[neurovault.dathproject.com/docs](https://neurovault.dathproject.com/docs)

## 🔌 连接 AI 客户端（MCP）

NeuroVault 自带本地 MCP 服务（`neurovault-server --mcp-only`），**无需手动启动 App**：首次调用时若检测到本地没有正在运行的服务，它会自动在后台拉起一个无界面的记忆引擎（headless）并转发到本地 `127.0.0.1:8765`；若你开着 NeuroVault App（想用图形界面），则自动连到 App 自带的那个服务。启动日志在 `~/.neurovault/autostart.log`，可用环境变量 `NEUROVAULT_AUTOSTART=0` 关闭自动启动。

> 路径说明：以下为 macOS 默认安装路径。App 不在 `/Applications` 时换成实际路径；Windows 上是 `neurovault-server.exe` 的完整路径。

### Kilo Code —— `~/.config/kilo/kilo.jsonc`

在文件**顶层**加入（若已有 `"mcp"` 键，把 `neurovault` 并进去），保存后重启 Kilo Code：

```jsonc
"mcp": {
  "neurovault": {
    "type": "local",
    "command": ["/Applications/NeuroVault.app/Contents/MacOS/neurovault-server", "--mcp-only"],
    "enabled": true
  }
}
```

### Pi —— `~/.config/mcp/mcp.json`

```json
{
  "mcpServers": {
    "neurovault": {
      "command": "/Applications/NeuroVault.app/Contents/MacOS/neurovault-server",
      "args": ["--mcp-only"]
    }
  }
}
```

### DSHarness（DeepSeek Harness）

把下面的 `insert` 追加到**当前 profile** 的 `cordis.patch.yml`（桌面版一般是 `~/.dsh/profiles/desktop/cordis.patch.yml`，web 版是 `~/.dsh/profiles/web/…`，profile 名不同就换成实际的；文件还是空的 `# []` 时，直接把整段 `- insert:` 放进去）：

```yaml
- insert:
    - id: mcp-neurovault
      name: '@deepseek-ai/dsh-mcp-client'
      config:
        serverName: neurovault
        transport: stdio
        command: /Applications/NeuroVault.app/Contents/MacOS/neurovault-server
        args: ['--mcp-only']
```

> Claude Code / Cursor / Claude Desktop 等其他客户端使用标准 `mcpServers` 格式，配置方法见上游文档。

### 🧪 快速验证（测试例子）

连接好后，用两步就能确认生效：

1. **写入**：对对接好的 AI 说 ——「记住：我的测试宠物叫豆豆，是一只橘猫」（或在 App 的「Memories」里直接写一条笔记）。
2. **跨会话检索**：**新开一个会话**再问 ——「你还记得我养了什么宠物吗？」能答出「豆豆 / 橘猫」即配置成功 ✅；答不上来，就检查 NeuroVault 应用是否在运行、上面对应客户端配置路径是否写对。

> 还不行？把上面「连接 AI 客户端」这一节的配置内容发给你的 AI，对它说「**帮我配置 NeuroVault 的 MCP 连接**」——它会读取对应的配置文件、帮你补好 `neurovault` 条目并提示重启；它只改那一个文件，不会动其他配置，很安全。

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
