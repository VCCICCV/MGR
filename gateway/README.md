## 网关
这里使用Apisix作为网关（本来想用pingora，但是不支持windows）
## 代理
* 反向代理
* 正向代理
## 负载均衡

## 零停机升级
## 网关概念
* Route：通过路由规则匹配客户端请求，根据匹配结果**转发**到上游应用
* UpStream：按配置规则进行负载均衡的**服务节点**
* AminAPI：通过HTTP接口对路由、服务节点等配置进行管理
* 南北向流量
* 东西向流量
## APISIX架构
[apisix](https://github.com/apache/apisix/blob/master/docs/assets/images/apisix.png)
## 安装APISIX
* 需要安装Docker用于部署etcd和APISIX
* 需要安装curl或Apifox用于测试

拉取APISIX源码，打包镜像
```
git clone https://github.com/apache/apisix.git

cd apisix
# 打包镜像，如果是windows可以使用wsl
docker build -t apisix-dev-env -f example/build-dev-image.dockerfile .
```
启动etcd
```
docker run -d --name etcd-apisix --net=host pachyderm/etcd:v3.5.2
```
将构建的镜像挂载到`apisix`目录并启动
```
docker run -d --name apisix-dev-env --net=host -v $(pwd):/apisix:rw apisix-dev-env:latest
```
构建 Apache APISIX 运行时并配置测试环境
```
docker exec -it apisix-dev-env make deps

docker exec -it apisix-dev-env ln -s /usr/bin/openresty /usr/bin/nginx
```
## 使用
创建路由
