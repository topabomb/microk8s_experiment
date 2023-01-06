FROM nginx:latest
#申明端口
EXPOSE 80
#目录到目录
COPY dist/spa/ /usr/share/nginx/html/
#改权限
RUN chown -R nginx /usr/share/nginx/html/
#运行脚本
RUN service nginx start
