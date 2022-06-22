# bincmp
雑にバイナリの比較をするツール in Rust  
微妙に成分の違うデカ～いバイナリを目視で追うのがしんどかったので…  
思い立って数時間くらいで書いたので本当に雑

## 機能
- 2ファイルを先頭 or オフセット位置から比較して、一致するかを判定  
- 一致しなければ、最初の不一致バイトの位置(16進)とその値を出力
- 異サイズのファイルを投げた場合、小さい方に合わせて比較
  - 小さい方の末尾までで不一致が無かった場合、partially matchedとなる

## `diff` とか `cmp` で良くない？
その通り
自分の欲しい情報だけをいい感じにキレイに出したかっただけです

## Build / Install
`cargo`を入れといてください
```
$ cargo build --release
```
**OR**
```
$ cargo intall --path .
```
## Usage
### 先頭から比較
```
$ bincmp -b a.bin -t b.bin
bincmp v0.0.1 by Nanamiiiii

Base binary:    a.bin       334 byte(s)
Target binary:  b.bin       334 byte(s)
Offset: 0 byte(s)

The binaries matched!
```
### オフセット付き
```
$ bincmp -b a.bin -t b.bin -o 100
bincmp v0.0.1 by Nanamiiiii

Base binary:    a.bin       334 byte(s)
Target binary:  b.bin       334 byte(s)
Offset: 100 byte(s)

The binaries unmatched!
Different byte at 0x124
Base byte: 0x24, Target byte: 0xA4
```

### 異サイズ
```
$ bincmp -b a.bin -t b.bin
bincmp v0.0.1 by Nanamiiiii

Base binary:    a.bin       334 byte(s)
Target binary:  b.bin       339 byte(s)
Offset: 100 byte(s)

The binaries partially matched!
Backward 5 byte(s) of target binary not checked because of size difference.
```

## TODO
- [x] 単純比較
- [x] 異サイズ比較
- [x] 共通オフセット
- [ ] 独立オフセット
- [ ] 不一致シーケンス検出
- [ ] ...