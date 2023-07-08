#!/bin/sh
/etc/NX/nxserver --startup
service ssh start
tail -f /usr/NX/var/log/* -f /var/log/*