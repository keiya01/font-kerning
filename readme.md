# Font Kerning を試す

font をいい感じに画像に描画するためのライブラリを探す。
kerning 周りを調整できて OTF + TTF がサポートされていれば良い。

使用するライブラリ

- rusttype
- fontdue
- ab_glyph

## rusttype

OTF のサポートが微妙。
owned_ttf_parser という ttf_parser のラッパーみたいなのを使っているが OTF がうまく動いていない。
NotoSansJp が動かない。

## fontdue

font の load 時に全ての glyph を HashMap にマッピングしており、日本語フォントのような文字数の多いフォントだと非常に読み込みが遅い。
あらかじめ font を読み込んでおいて再利用するケースでは良いかもしれないが使用する文字数が少ない場合に不利。
あらかじめサブセット化する方法もあるがいちいちやるのも大変。

読み込み以外の部分は使いやすかった。
kerning 周りも調整できて OTF も問題なく使えた。

## ab_glyph

これは ttf_parser のシンプルなラッパーという感じで必要最低限のことしかしていなくて使いやすかった。
kerning 周りも制御できる。

ttf_parser は ttf と書いてあるが OTF も扱える。

ab_glyph は OTF を推していそうなので、OTF の便利機能も使えるかも(まだ試していない)。
