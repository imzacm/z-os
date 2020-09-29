FROM rust:1.46.0 as build-env
RUN rustup install nightly && \
  rustup override set nightly && \
  rustup component add rust-src && \
  rustup component add llvm-tools-preview && \
  cargo install bootimage

FROM build-env as build-workspace
WORKDIR /app
ADD . /app

RUN apt-get update && \
  apt-get install -y qemu-system && \
  cargo test && \
  cargo bootimage

FROM alpine:edge as exec-env
ENV HOME=/root \
  DEBIAN_FRONTEND=noninteractive \
  LANG=en_GB.UTF-8 \
  LANGUAGE=en_GB.UTF-8 \
  LC_ALL=C.UTF-8 \
  DISPLAY=:0.0 \
  DISPLAY_WIDTH=1024 \
  DISPLAY_HEIGHT=768

RUN echo "http://dl-3.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories && \
  apk --update --upgrade add bash fluxbox git supervisor xvfb x11vnc wget python3 ttf-dejavu && \
  ln -s /usr/bin/python3 /usr/bin/python && \
  git clone --depth 1 https://github.com/novnc/noVNC.git /root/noVNC && \
  git clone --depth 1 https://github.com/novnc/websockify /root/noVNC/utils/websockify && \
  rm -rf /root/noVNC/.git && \
  rm -rf /root/noVNC/utils/websockify/.git && \
  apk del git && \
  sed -i -- "s/ps -p/ps -o pid | grep/g" /root/noVNC/utils/launch.sh && \
  wget https://raw.githubusercontent.com/uphy/novnc-alpine-docker/master/supervisord.conf -O /etc/supervisord.conf

EXPOSE 8080
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]

FROM exec-env as runner
RUN mkdir -p /usr/local/z_os
WORKDIR /usr/local/z_os
COPY --from=build-workspace /app/target/x86_64-z_os/debug/bootimage-z_os.bin /app/qemu.sh /usr/local/z_os/
RUN apk update && apk add qemu-system-x86_64 qemu-ui-gtk xterm && \
  echo '[program:xterm]' >> /etc/supervisord.conf && \
  echo 'command=xterm' >> /etc/supervisord.conf && \
  echo 'autorestart=true' >> /etc/supervisord.conf && \
  echo '[program:z_os]' >> /etc/supervisord.conf && \
  echo 'command=/usr/local/z_os/qemu.sh /usr/local/z_os/bootimage-z_os.bin' >> /etc/supervisord.conf && \
  echo 'autorestart=true' >> /etc/supervisord.conf
