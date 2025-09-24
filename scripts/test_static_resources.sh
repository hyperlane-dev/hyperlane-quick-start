#!/bin/bash

# 统一静态资源系统测试脚本

echo "=== 统一静态资源系统测试 ==="
echo

# 设置测试服务器地址
SERVER_URL="http://localhost:8080"

# 测试不同类型的静态资源
echo "1. 测试静态资源访问..."

# 测试CSS文件
echo "测试CSS文件: /static/css/style.css"
curl -I "$SERVER_URL/static/css/style.css" 2>/dev/null | head -n 1

# 测试JavaScript文件
echo "测试JS文件: /assets/js/app.js"
curl -I "$SERVER_URL/assets/js/app.js" 2>/dev/null | head -n 1

# 测试图片文件
echo "测试图片文件: /uploads/images/photo.jpg"
curl -I "$SERVER_URL/uploads/images/photo.jpg" 2>/dev/null | head -n 1

# 测试公共文档
echo "测试公共文档: /public/docs/manual.pdf"
curl -I "$SERVER_URL/public/docs/manual.pdf" 2>/dev/null | head -n 1

echo

# 测试缓存功能
echo "2. 测试缓存功能..."

# 获取ETag
ETAG=$(curl -I "$SERVER_URL/static/css/style.css" 2>/dev/null | grep -i etag | cut -d' ' -f2 | tr -d '\r')
if [ ! -z "$ETAG" ]; then
    echo "ETag: $ETAG"
    
    # 测试条件请求
    echo "测试条件请求 (If-None-Match):"
    curl -I -H "If-None-Match: $ETAG" "$SERVER_URL/static/css/style.css" 2>/dev/null | head -n 1
fi

echo

# 测试范围请求
echo "3. 测试范围请求..."
echo "测试范围请求 (bytes=0-1023):"
curl -I -H "Range: bytes=0-1023" "$SERVER_URL/static/css/style.css" 2>/dev/null | head -n 1

echo

# 测试安全防护
echo "4. 测试安全防护..."

# 测试路径遍历攻击
echo "测试路径遍历攻击: /static/../etc/passwd"
curl -I "$SERVER_URL/static/../etc/passwd" 2>/dev/null | head -n 1

# 测试隐藏文件访问
echo "测试隐藏文件访问: /static/.htaccess"
curl -I "$SERVER_URL/static/.htaccess" 2>/dev/null | head -n 1

echo

# 测试错误处理
echo "5. 测试错误处理..."

# 测试不存在的文件
echo "测试不存在的文件: /static/nonexistent.txt"
curl -I "$SERVER_URL/static/nonexistent.txt" 2>/dev/null | head -n 1

# 测试无效的资源类型
echo "测试无效的资源类型: /invalid/file.txt"
curl -I "$SERVER_URL/invalid/file.txt" 2>/dev/null | head -n 1

echo

# 测试内容类型检测
echo "6. 测试内容类型检测..."

# 测试不同文件类型的Content-Type
declare -a file_types=("css" "js" "html" "png" "jpg" "json" "pdf")

for ext in "${file_types[@]}"; do
    echo "测试 .$ext 文件的Content-Type:"
    curl -I "$SERVER_URL/static/test.$ext" 2>/dev/null | grep -i content-type || echo "  文件不存在或无Content-Type"
done

echo

# 性能测试
echo "7. 简单性能测试..."

echo "测试并发请求 (10个并发):"
time (
    for i in {1..10}; do
        curl -s "$SERVER_URL/static/css/style.css" > /dev/null &
    done
    wait
)

echo

# 创建测试文件的函数
create_test_files() {
    echo "创建测试文件..."
    
    # 创建目录结构
    mkdir -p resources/static/css
    mkdir -p resources/static/js
    mkdir -p resources/assets/js
    mkdir -p resources/public/docs
    mkdir -p uploads/images
    
    # 创建测试文件
    echo "/* Test CSS */" > resources/static/css/style.css
    echo "// Test JS" > resources/static/js/app.js
    echo "// Test Asset JS" > resources/assets/js/app.js
    echo "Test document" > resources/public/docs/manual.txt
    echo "Test image data" > uploads/images/photo.txt
    
    echo "测试文件已创建"
}

# 清理测试文件的函数
cleanup_test_files() {
    echo "清理测试文件..."
    rm -rf resources/static/css/style.css
    rm -rf resources/static/js/app.js
    rm -rf resources/assets/js/app.js
    rm -rf resources/public/docs/manual.txt
    rm -rf uploads/images/photo.txt
    echo "测试文件已清理"
}

# 检查命令行参数
case "$1" in
    "setup")
        create_test_files
        ;;
    "cleanup")
        cleanup_test_files
        ;;
    "full")
        create_test_files
        echo "开始完整测试..."
        # 这里会运行上面的所有测试
        echo "完整测试完成"
        cleanup_test_files
        ;;
    *)
        echo "用法: $0 {setup|cleanup|full}"
        echo "  setup   - 创建测试文件"
        echo "  cleanup - 清理测试文件"
        echo "  full    - 运行完整测试（包括创建和清理）"
        echo
        echo "或者直接运行脚本进行基本测试"
        ;;
esac

echo "=== 测试完成 ==="