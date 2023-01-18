# 协议

## 短连接 `protocol::short`

### 请求 `protocol::short::request`

```
<方法>
[可选] Token = <base64url字符串>
Length = <数据字节流长度>
[可选] <data>
```
注
1. 除了**登录**，所有请求都要带上`Token`。
2. 若无数据传输，则写入`Length = 0`。

### 响应 `protocol::short::response`

```
<状态>
Length = <数据字节流长度>
[可选] <data>
```
注
1. 成功响应的状态一律为`ok`。
2. 若无数据传输，则写入`Length = 0`。
3. 未通过鉴权认证的一律响应`unauthorized`。

#### 类型

- **登录**
  - 请求
    - 方法：`login`
    - 数据
      ```
      {
        id: Option<i64>,
        nickname: String,
        password: String,
      }
      ```
  - 响应
    - 数据：语义为**token**，类型为`String`
    - 失败
      1. 用户不存在：`user-not-found`
      2. 密码错误：`wrong-password`

- **获取所有群组的信息**
  - 请求
    - 方法：`groups`
  - 响应
    - 数据
      ```
      [{
        id: i32,
        name: String,
        owner_id: i64,
      }]
      ```

- **创建新群组**
  - 请求
    - 方法：`create-group`
    - 数据
      ```
      {
        name: String,
      }
      ```
  - 响应
    - 数据
      ```
      {
        id: i32,
        name: String,
        owner_id: i64,
      }
      ```
    - 失败：该用户拥有同名群，`group-existed`。

- **加入群组**
  - 请求
    - 方法：`join-group`
    - 数据：语义为**gid**，类型为`i32`
  - 响应：`ok`

- **建立长连接**
  - 请求
    - 方法：`persistence`
  - 响应：`ok`



## 长连接 `protocol::persistent`

### 平包 `protocol::persistent::Horz`

就是长连接协议里包的名字，我实在想不出更动听的了。

#### 格式

```
<方法>
Length = <数据字节流长度>
[可选] <data>
```
注：若无数据传输，则写入`Length = 0`。

#### 类型

- **发起连接测试**
  - 方法：`ping`

- **响应连接测试**
  - 方法：`pong`

- **切断连接**
  - 方法：`close`

- **私信**
  - 方法：`private-message`
  - 数据：`protocol::models::PrivateMessage`

- **群组消息**
  - 方法：`group-message`
  - 数据：`protocol::models::GroupMessage`

- **私信回音**
  - 方法：`echo`
  - 数据：`protocol::models::PrivateMessage`
