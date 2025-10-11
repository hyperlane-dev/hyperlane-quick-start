# 实施计划

- [-] 1. 创建统一的数据模型和类型定义
  - 创建DatabaseOperation泛型结构体用于统一操作请求格式
  - 创建OperationType枚举定义操作类型（Create, Read, Update, Delete）
  - 创建ApiResponse泛型结构体用于统一API响应格式
  - 为新模型添加必要的derive宏（Serialize, Deserialize, ToSchema等）
  - _需求: 1.4, 4.1, 4.2, 4.3_

- [ ] 2. 重构MySQL控制器接口
  - [ ] 2.1 创建统一的MySQL POST接口处理函数
    - 实现handle_mysql_operation函数接收DatabaseOperation<MysqlRecord>
    - 根据operation字段分发到相应的服务函数（create, read, update, delete）
    - 实现统一的错误处理和响应格式化
    - _需求: 1.1, 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ] 2.2 更新MySQL路由配置
    - 将所有MySQL路由改为单一的POST /api/mysql路径
    - 移除旧的分离路由（/api/mysql/create, /api/mysql/delete等）
    - 更新OpenAPI文档注解
    - _需求: 1.1_

- [ ] 3. 重构PostgreSQL控制器接口
  - [ ] 3.1 创建统一的PostgreSQL POST接口处理函数
    - 实现handle_postgresql_operation函数接收DatabaseOperation<PostgresqlRecord>
    - 根据operation字段分发到相应的服务函数
    - 实现统一的错误处理和响应格式化
    - _需求: 1.2, 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ] 3.2 更新PostgreSQL路由配置
    - 将所有PostgreSQL路由改为单一的POST /api/postgresql路径
    - 移除旧的分离路由
    - 更新OpenAPI文档注解
    - _需求: 1.2_

- [ ] 4. 重构Redis控制器接口
  - [ ] 4.1 创建统一的Redis POST接口处理函数
    - 实现handle_redis_operation函数接收DatabaseOperation<RedisRecord>
    - 根据operation字段分发到相应的服务函数
    - 处理Redis特有的批量查询逻辑（keys字段）
    - 实现统一的错误处理和响应格式化
    - _需求: 1.3, 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ] 4.2 更新Redis路由配置
    - 将所有Redis路由改为单一的POST /api/redis路径
    - 移除旧的分离路由
    - 更新OpenAPI文档注解
    - _需求: 1.3_

- [ ] 5. 实现前端AJAX请求基础设施
  - [ ] 5.1 创建统一的AJAX请求函数
    - 实现performDatabaseOperation函数使用fetch API
    - 支持不同数据库类型和操作类型的参数
    - 实现请求错误处理和重试机制
    - _需求: 2.1, 2.3_
  
  - [ ] 5.2 创建响应处理和UI更新函数
    - 实现handleApiResponse函数处理统一响应格式
    - 创建showErrorMessage函数显示错误信息
    - 创建updateRecordsList函数动态更新记录列表
    - _需求: 2.2, 2.3_

- [ ] 6. 重构MySQL前端页面
  - [ ] 6.1 移除HTMX依赖并实现AJAX表单提交
    - 移除所有hx-*属性从MySQL页面表单
    - 为表单添加事件监听器阻止默认提交行为
    - 实现创建、更新、删除操作的AJAX调用
    - _需求: 2.1, 2.4_
  
  - [ ] 6.2 实现动态内容更新
    - 实现记录列表的动态刷新功能
    - 添加操作成功/失败的用户反馈
    - 实现编辑模态框的AJAX提交
    - _需求: 2.2, 2.3_

- [ ] 7. 重构PostgreSQL前端页面
  - [ ] 7.1 移除HTMX依赖并实现AJAX表单提交
    - 移除所有hx-*属性从PostgreSQL页面表单
    - 为表单添加事件监听器阻止默认提交行为
    - 实现创建、更新、删除操作的AJAX调用
    - _需求: 2.1, 2.4_
  
  - [ ] 7.2 实现动态内容更新
    - 实现记录列表的动态刷新功能
    - 添加操作成功/失败的用户反馈
    - 实现编辑模态框的AJAX提交
    - _需求: 2.2, 2.3_

- [ ] 8. 重构Redis前端页面
  - [ ] 8.1 移除HTMX依赖并实现AJAX表单提交
    - 移除所有hx-*属性从Redis页面表单
    - 为表单添加事件监听器阻止默认提交行为
    - 实现创建、更新、删除操作的AJAX调用
    - 处理Redis特有的批量查询功能
    - _需求: 2.1, 2.4_
  
  - [ ] 8.2 实现动态内容更新
    - 实现记录列表的动态刷新功能
    - 添加操作成功/失败的用户反馈
    - 实现编辑模态框的AJAX提交
    - _需求: 2.2, 2.3_

- [ ]* 9. 编写单元测试
  - [ ]* 9.1 为统一数据模型编写测试
    - 测试DatabaseOperation和ApiResponse的序列化/反序列化
    - 测试OperationType枚举的各种情况
    - _需求: 4.1, 4.2, 4.3_
  
  - [ ]* 9.2 为控制器函数编写测试
    - 测试MySQL、PostgreSQL、Redis控制器的操作分发逻辑
    - 测试错误处理和响应格式化
    - 测试各种边界条件和异常情况
    - _需求: 1.1, 1.2, 1.3, 3.1, 3.2, 3.3, 3.4, 3.5_

- [ ]* 10. 编写集成测试
  - [ ]* 10.1 测试完整的API请求-响应流程
    - 测试从前端AJAX请求到后端响应的完整流程
    - 测试不同操作类型的正确执行
    - 测试数据库连接和数据持久化
    - _需求: 1.1, 1.2, 1.3, 2.1, 2.2_
  
  - [ ]* 10.2 测试前端用户界面交互
    - 测试表单提交和页面更新
    - 测试错误处理和用户反馈
    - 测试不同浏览器的兼容性
    - _需求: 2.2, 2.3, 2.4_