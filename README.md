# 天书 - 轻量级文献管理软件

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yourusername/litelit/Rust%20CI)
![GitHub License](https://img.shields.io/github/license/yourusername/litelit)
![GitHub Repo Size](https://img.shields.io/github/repo-size/yourusername/litelit)

## 项目简介

天书 是一款专为研究人员和学者设计的轻量级文献管理软件。它摒弃了传统的文件夹分类方式，采用灵活的标签管理系统，让文献整理变得更加高效和直观。无论你是整理学术论文、研究报告还是阅读笔记，LiteLit 都能助你一臂之力。

## 核心功能

- **纯标签管理**：告别文件夹分类的困扰，采用灵活的标签系统，自由组合标签筛选文献。
- **双重筛选机制**：标签栏分为上下两栏，上栏标签必须包含，下栏标签任意包含，精准定位目标文献。
- **快捷标签组**：自定义标签组合，一键保存，快速筛选，提高文献管理效率。
- **AI 摘要生成**：集成 AI 模块，自动总结文献核心内容，快速了解文献主旨。
- **核心信息提取**：自动提取文献的关键信息，包括作者、刊物、时间等，无需手动输入。

## 技术栈

- **前端**：Vue.js
- **后端**：Rust

## 快速开始

### 克隆项目

```bash
git clone https://github.com/yourusername/litelit.git
```

### 安装依赖

```bash
cd litelit
cargo install
```

### 运行项目

```bash
pnpm tauri dev
```

## 使用方法

1. **导入文献**：支持 PDF、TXT 等多种格式文献的导入。
2. **添加标签**：为每篇文献添加合适的标签，方便后续筛选。
3. **筛选文献**：通过标签组合或快捷标签组，快速找到目标文献。
4. **查看摘要**：AI 自动生成文献摘要，快速了解文献内容。

## 贡献指南

欢迎各位开发者参与 LiteLit 的开发，共同完善这款文献管理工具。如需贡献代码或提出建议，请参考 [CONTRIBUTING.md](CONTRIBUTING.md) 文件。

## 许可证

本项目采用 [MIT License](LICENSE) 许可证。

