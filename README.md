# PNGme

一个将信息隐藏到PNG文件中的命令行工具，该项目是 [PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/)
的一个实现。

## 用法

将信息编码到 PNG 文件

```shell
pngme encode ./dice.png ruSt "This is a secret message!
```

解码 PNG 文件中存储的信息

```shell
pngme decode ./dice.png ruSt
```

从 PNG 文件中删除信息

```shell
pngme remove ./dice.png ruSt
```

打印所有隐藏的信息

```shell
pngme print ./dice.png
```

> [PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/) 是一个很好的Rust练手项目，强烈推荐！！！
