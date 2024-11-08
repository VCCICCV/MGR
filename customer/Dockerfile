FROM rust:latest as builder
# 工作目录
WORKDIR /app
# 将代码复制到镜像中
COPY . .
# 构建镜像
RUN cargo build --release --target x86_64-unknown-linux-musl
# 从scratch镜像开始，这是一个最小化的Linux镜像，不包含任何软件包
FROM scratch
# 将可执行文件从构建镜像复制到新镜像中
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-ddd-start /axum-ddd-start
ENTRYPOINT ["/axum-ddd-start"]
EXPOSE 8080