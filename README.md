<h1 align="center">net route rs</h1>

使用 Rust 实现的路由器，支持 IPv4 路由配置和查询。

可通过配置文件或命令行参数对 Windows 系统的路由表进行增删改查操作。


配置文件示例如下：

```json
{
  "routes": [
    {
      "ifindex": 28,
      "domains": [
        "baidu.com",
        "frp-mix.com"
      ],
      "ips": []
    }
  ]
}
```

详情请使用 `net-route-rs --help` 查看帮助信息。