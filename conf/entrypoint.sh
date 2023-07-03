#!/bin/sh
/etc/NX/nxserver --startup
tail -f /usr/NX/var/log/* -f /var/log/*