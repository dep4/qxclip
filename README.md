On host
-------

```
qemu \
-device virtio-serial \
-chardev socket,path=/tmp/foo,server,nowait,id=foo \
-device virtserialport,chardev=foo

qxclip /tmp/foo
```

On guest
--------

```
qxclip /dev/vport2p1
```
