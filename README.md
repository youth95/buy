# Buy

两个接口

```
生成码的接口。记录生成时间和有效天数。
GET /buy/1
XtF7hoQBAAABAAAAAAAAALj9yfE+QOIUEAAAAAAAAACajMuffpe2mgPtP_Gc0new
```

验证码的接口。验证是否被篡改，是否过期。`expired`为申请的天数。`publish`为申请时候的时间戳(ms)。`balance`为剩余的时间(ms)。`balance_str`为剩余时间的文本表示。
```
GET /check/XtF7hoQBAAABAAAAAAAAALj9yfE+QOIUEAAAAAAAAACajMuffpe2mgPtP_Gc0new
{"ok":true,"publish":1668778492674,"expired":1,"balance":86373622,"balance_str":"23.99H"}
```

# TODO

- 限制 `/check/<code>` 接口对于同一code的访问频次