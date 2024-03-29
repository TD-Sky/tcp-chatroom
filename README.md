# TCP聊天室

## 题目

多人聊天室，分别实现服务端和客户端。

### 要求

- [x] 允许多个客户端连接服务端；
- [ ] 客户端连接成功后默认进入全服聊天，发送消息其他客户端可以看到；
- [ ] 客户端可以设置自己的昵称，其他人可以查看当前在线的客户端列表；
- [ ] 客户端可以创建频道，其他人可以进入频道，进入频道的客户端可以同时接收全服聊天和频道聊天的内容；
- [ ] 客户端可以私聊其他在线用户。
- [ ] 拓展要求：服务端程序使用select等io多路复用机制，不允许使用多线程。

## 文档

[聊天室协议](./docs/protocol.md)



## 🚧 施工中...

| 层级 | 对象 |
|:-:|:-|
| **服务层** | 端点1, 端点2, ... |
| **服务层** | 路由 |
| **连接层** | 监听并采取动作 |
| **协议层** | 协议通信使用的包 |
| **I/O层**  | socket |
