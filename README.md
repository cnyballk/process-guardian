# Process Guardian (进程守护程序)

简单的进程守护工具，用于监控和自动重启指定的程序。

## 功能特点

- 支持多进程守护
- 可配置重启延迟时间
- 可设置最大重启次数（-1 表示无限重启）
- 支持工作目录配置
- 支持命令行参数

## 使用方法

1. 配置文件 (config.json)
```json
{
    "processes": [
        {
            "name": "程序名称",
            "executable_path": "./exe/your_app.exe",
            "arguments": ["参数1", "参数2"],
            "working_directory": "./exe",
            "restart_delay": 5,
            "max_restarts": -1
        }
    ]
}
```

## 未来计划
- 添加图形用户界面(GUI)
- 系统托盘支持
- 进程状态实时监控
- 资源占用监控（CPU、内存）
- 日志管理功能
- 邮件通知支持
- 远程管理功能
- Web控制面板
- 集群管理支持
- 进程性能分析
- 自定义告警规则