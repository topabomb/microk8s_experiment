# Windows环境MicroK8s的安装过程及疑难解答
## 环境安装
1. microk8s安装程序
[下载链接](https://microk8s.io/microk8s-installer.exe)
2. 执行安装程序
    - 安装multipass时虚拟机引擎 选择Hyper-V
    - 配置虚拟机需求，建议4c8g作为最小配置，后续可以在Hyper-V中调整
    >安装程序首先安装multipass虚拟机管理器，然后安装microk8s本地的工具，通过multipass创建一个hyper-v虚拟机作为microk8s的宿主机，注意，k8s实际是跑在虚拟机中而不是本地操作系统，故本地执行的microk8s的命令实际是代理转发到了microk8s-vm中执行。
3. 通过Hyper-V管理器或Multipass停止vm，然后移动vm宿主机的虚拟磁盘到容量较大的硬盘中；同时如果对前一步的虚拟机配置有所调整的话，可在此步骤一并调整，例如调整为8c8g。随后启动vm。
4. microk8s服务状态命令（本地环境），[命令参考](https://microk8s.io/docs/command-reference)
```
microk8s start
microk8s stop
microk8s reset
microk8s status --wait-ready
```
5. microk8s插件管理命令(本地环境)，[插件清单](https://microk8s.io/docs/addons)
```
microk8s enable dns
microk8s disable dns
microk8s enable dns dashboard registry
microk8s kubectl get pods -A
```
6. 通过proxy访问dashboard，以下方案均需获取访问token才可以操作集群，访问链接可能是(https://172.18.105.88:10443/),注意更换ip为vm的ip地址。
    * 在本地环境通过microk8s启用，输出中会提示ip及端口
    ```
    microk8s dashboard-proxy
    ```
    * 在本地环境查看vm ip,通过multipass启动端口映射
    ```
    multipass info microk8s-vm
    multipass exec microk8s-vm -- sudo microk8s.kubectl port-forward -n kube-system service/kubernetes-dashboard 10443:443 --address 0.0.0.0
    ```
    * 通过visual code插件Kubernetes，Cloud Code均选择集群，右键开启dashboard。
        > 需完成Kubectl工具安装与配置
---

## 安装时的疑难问题
1. 解决gcr.io镜像在国内无法下载所导致的插件安装问题
    - 通过multipass 打开microk8s-vm的shell
    - 在vm宿主机安装docker
    ```
    sudo snap install docker
    ```
    - 在vm宿主机安装pullk8s脚本，[源码](https://github.com/OpsDocker/pullk8s)
    ```
    sudo curl -L "https://raw.githubusercontent.com/OpsDocker/pullk8s/main/pullk8s.sh" -o /usr/local/bin/pullk8s
    sudo chmod +x /usr/local/bin/pullk8s
    ```
    - 查看无法获取的docker image，然后逐一进行拉取，直至所有pods的STATUS=Running
    ```
    sudo pullk8s check --microk8s
    sudo pullk8s pull k8s.gcr.io/pause:3.1 --microk8s
    ```

2. 访问 dashboard提示ERR_CERT_INVALID
    - chrome等较新浏览器提示NET::ERR_CERT_INVALID，可以按下面步骤更换dashboard的证书为合格的证书
    - 通过社区工具生成新的证书（保存在当前目录下的/certs中），在vm中执行，[参考](https://github.com/soulteary/certs-maker/)
    ```
    sudo docker run --rm -it -v `pwd`/certs:/ssl soulteary/certs-maker --FOR_K8S=on
    ```
    - 在vm shell中执行更换证书
    ```
    cd certs/
    sudo microk8s.kubectl -n kube-system delete secret kubernetes-dashboard-certs
    sudo microk8s.kubectl -n kube-system create secret generic kubernetes-dashboard-certs --from-file=./lab.com.k8s.crt --from-file=./lab.com.k8s.key
    ```
    - 在vm shell中打开kubernetes-dashboard的配置（vi编辑器）
    ```
    sudo microk8s.kubectl -n kube-system edit deploy kubernetes-dashboard -o yaml
    ```
    - 查找 --auto-generate-certificates并修改该部分内容为下面的内容，(/搜索,n下一个条目,wq保存退出)
    ```
    spec:
      containers:
      - args:
        - --tls-cert-file=lab.com.k8s.crt
        - --tls-key-file=lab.com.k8s.key
        # - --auto-generate-certificates
    ```
    - 重启启动端口映射，并尝试打开dashboard url。

---
## Kubectl工具的配置修改
1. 安装Kubectl工具，[安装说明](https://kubernetes.io/docs/tasks/tools/install-kubectl-windows/#install-kubectl-binary-with-curl-on-windows)

2. 获取microk8s的配置文件
    ```
    microk8s config
    ```
3. 合并到配置文件,windows当前用户的kebectl默认配置文件位置,[参考网址](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/)

    ```
    %USERPROFILE%\.kube\config
    ```
    > 如果kebectl默认配置文件不存在的话，可直接使用microk8s的配置，如果包含了其他集群的配置，请按照格式进行多个集群的合并
4. visual code的可用插件Kubernetes，Cloud Code应该都可以对microk8s集群进行管理了。