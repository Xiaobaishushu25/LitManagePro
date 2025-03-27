/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html,js,vue,ts,jsx,tsx}",
    "./public/index.html", // 如果你直接在 public 文件夹中使用 HTML
  ],
  theme: {
    extend: {
      fontFamily: {
        'zhan': ['SimSun', 'sans-serif'], // 中文字体：宋体
        'ying': ['"Arial"', 'serif'], // 英文字体：Times New Roman
        'roman': ['"Times New Roman"', 'serif'], // 英文标题字体
        'hv': ['"Times New Roman"', 'serif'], // 英文标题字体
        'label': ['"Open Sans"', 'sans-serif'], // 英文标签字体 (thin version)
        'yahei': ['"Microsoft YaHei"', 'sans-serif'], // 中文标题字体：微软雅黑
        'heiti': ['"SimHei"', 'sans-serif'], // 中文标题字体：微软雅黑
        // 'wenKai': ['"wenKai"', 'sans-serif']
      },
    },
  },
  plugins: [],
}

