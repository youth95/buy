# Buy

两个接口

```
生成码的接口。记录生成时间和有效天数。
GET /buy/1
XtF7hoQBAAABAAAAAAAAALj9yfE+QOIUEAAAAAAAAACajMuffpe2mgPtP_Gc0new
```

```
验证码的接口。验证是否被篡改，是否过期。
GET /check/XtF7hoQBAAABAAAAAAAAALj9yfE+QOIUEAAAAAAAAACajMuffpe2mgPtP_Gc0new
{"ok":true,"publish":1668703572318,"expired":1}
```

# TODO

- 限制 `/check/<code>` 接口对于同一code的访问频次