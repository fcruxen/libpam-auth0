FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive

ENV NOMACHINE_PACKAGE_NAME nomachine_8.6.1_3_amd64.deb
ENV NOMACHINE_BUILD 8.6
ENV NOMACHINE_MD5 d833ad52f92e5b3cc30c96f12686d97f

RUN apt-get update && apt-get install -y vim xterm pulseaudio cups \
    openssh-server mate-desktop-environment-core mate-desktop-environment \
    mate-indicator-applet ubuntu-mate-themes ubuntu-mate-wallpapers firefox nano sudo wget curl

RUN wget "https://download.nomachine.com/download/${NOMACHINE_BUILD}/Linux/${NOMACHINE_PACKAGE_NAME}"
RUN dpkg -i ${NOMACHINE_PACKAGE_NAME}
RUN sed -i "s|#EnableClipboard both|EnableClipboard both |g" /usr/NX/etc/server.cfg


RUN apt-get clean
RUN apt-get autoclean

RUN rm -rf /var/lib/apt/lists/*

ADD ./conf/entrypoint.sh /
ADD ./conf/nx /etc/pam.d/nx
ADD ./conf/sshd /etc/pam.d/sshd
ADD ./target/debug/libpam_auth0.so /lib/x86_64-linux-gnu/security/libpam_auth0.so

RUN chmod +x /entrypoint.sh
RUN touch /var/log/libpam-auth0

EXPOSE 4000

ENTRYPOINT ["/entrypoint.sh"]

