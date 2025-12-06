# Simple Cut Tauri

一个基于 Tauri + React + TypeScript 开发的轻量级视频剪辑工具，提供简洁的界面和实用的功能。

## ✨ 功能特点

### 🎬 素材设置
- 支持添加、移除视频文件
- 可视化排序功能（向上/向下移动）
- 为每个视频设置单独的开始时间和结束时间
- 序号自动管理，确保导出顺序正确

### 📤 导出设置
- 自定义导出文件名
- 灵活的导出码率选择
- 可选择导出路径，支持默认使用第一个视频的路径
- 支持合并多音轨选项
- 导出过程可视化反馈

### 📖 使用说明
- 详细的功能使用指南
- 操作步骤说明

### 📝 关于
- 软件版本信息
- 开发者信息

## 🛠️ 技术栈

- **前端框架**: React 18 + TypeScript
- **UI组件库**: Ant Design
- **构建工具**: Vite
- **桌面应用框架**: Tauri
- **样式方案**: CSS-in-JS

## 📦 安装步骤

### 前置要求

- Node.js (v16.x 或更高版本)
- Rust 开发环境
- Tauri CLI

### 安装依赖

```bash
# 使用 npm
npm install

# 或使用 pnpm
pnpm install
```

### 开发模式

```bash
npm run dev
```

### 构建生产版本

```bash
npm run build
```

## 🚀 使用说明

### 1. 素材设置

1. 点击「添加文件」按钮添加视频素材
2. 使用表格中的输入框设置每个视频的开始时间和结束时间
3. 选择视频后，可以使用「向上移动」或「向下移动」调整顺序
4. 使用「移除文件」删除不需要的素材
5. 点击「清除全部」可以清空所有素材

### 2. 导出设置

1. 输入导出文件名
2. 选择合适的导出码率
3. 设置导出路径（可选择使用第一个视频的路径作为默认）
4. 根据需要选择是否合并多音轨
5. 点击「导出」按钮开始导出过程

## 📁 项目结构

```
simple-cut-tauri/
├── src/                  # 前端源代码
│   ├── components/       # React 组件
│   ├── pages/            # 页面组件
│   ├── App.tsx           # 应用主组件
│   └── main.tsx          # 应用入口
├── src-tauri/            # Tauri 后端代码
│   ├── src/              # Rust 源代码
│   └── tauri.conf.json   # Tauri 配置文件
├── package.json          # 项目依赖
├── tsconfig.json         # TypeScript 配置
├── vite.config.ts        # Vite 配置
└── README.md             # 项目说明文档
```

## 🔧 开发说明

### 添加新功能

1. 在 `src/pages/` 目录下创建新页面组件
2. 在 `src/App.tsx` 中配置导航菜单
3. 在 `src/components/` 目录下创建可复用组件

### 调试应用

- 前端调试：使用浏览器开发者工具
- 后端调试：查看 Rust 控制台输出

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📧 联系方式

如有问题或建议，请通过以下方式联系：

- 项目地址：[GitHub Repository]
- 开发者：[Your Name]

---

**Simple Cut Tauri** - 让视频剪辑变得简单高效！
