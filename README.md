# SAORI pie chart

[GitHub repository](https://github.com/tukinami/saori-pie-chart)

## これは何?

デスクトップマスコット、「伺か」で使用できるSAORIの一種です。

円グラフを作成し、pngとして出力します。

「伺か」「SAORI」等の用語については詳しく説明いたしませんのでご了承下さい。

## 使い方

SAORI自体の使い方は、使用するSHIORIなどによって異なりますので、ご自身でお調べ下さい。

ここではこのSAORIの使い方について説明いたします。

Argument0〜9までの引数が必須項目です。
Argument10〜12、13〜15……と、以後3つ一組で一つの項目を入力します。

成功した場合、0という文字列を返します。
それ以外の場合は失敗理由を返します。

### 必須項目

+ Argument0: 出力するpngファイルのパス(SAORIからの相対パスになります)
+ Argument1: 出力するpngファイルの幅(px)
+ Argument2: 出力するpngファイルの高さ(px)
+ Argument3: 円グラフの半径(px)
+ Argument4: ラベル文字の色(RGBのR(0~255))
+ Argument5: ラベル文字の色(RGBのG(0~255))
+ Argument6: ラベル文字の色(RGBのB(0~255))
+ Argument7: ラベル文字のフォント(システムにインストールされていなければデフォルトのものが使用されます)
+ Argument8: ラベル文字の大きさ(px)
+ Argument9: ラベル文字の位置(中心からの距離)

### 必須項目後、項目ごとに一組ずつ

+ Argument*: ラベル文字(文字列)
+ Argument*: 円グラフの大きさ(全体を1としたときの割合)
+ Argument*: 円グラフの色([CSSで色として指定できる文字列](https://developer.mozilla.org/ja/docs/Web/CSS/color_value))

### 例: YAYA

```
On_Test
{
    // 幅: 100px, 高さ: 100px, 半径: 40px, ラベル文字の色: 0, 0, 0 (黒),
    // ラベル文字のフォント: sans-self (ゴシック体デフォルト),
    // ラベル文字の大きさ: 10px, ラベル文字の位置: 20px
    // 項目1:: ラベル文字: Red, 大きさ: 0.5, 色: #fe5555(少し明るい赤)
    // 項目2:: ラベル文字: Green, 大きさ: 0.1, 色: #55fe55(少し明るい緑)
    // 項目3:: ラベル文字: Blue, 大きさ: 0.25, 色: #5555fe(少し明るい青)
    // 項目4:: ラベル文字: Other, 大きさ: 0.15, 色: #999(灰色)
    _result = FUNCTIONEX('path/to/piechart.dll', 'path/to/output.png', 100, 100, 40, 0, 0, 0, 'sans-serif', 10, 20, 'Red', 0.5, '#fe5555', 'Green', 0.1, '#55fe55', 'Blue', 0.25, '#5555fe', 'Other', 0.15, '#999')
    
    if _result == 0 {
        // 成功時の処理
    }
    else {
        // 失敗時の処理
    }
}
```

### 例: 里々

#### satori_conf.txt

```
＠SAORI
お好きな登録名,path/to/piechart.dll
```

#### 使用時

```
＊テスト
＃ 幅: 100px, 高さ: 100px, 半径: 40px, ラベル文字の色: 0, 0, 0 (黒),
＃ ラベル文字のフォント: sans-self (ゴシック体デフォルト),
＃ ラベル文字の大きさ: 10px, ラベル文字の位置: 20px
＃ 項目1:: ラベル文字: Red, 大きさ: 0.5, 色: #fe5555(少し明るい赤)
＃ 項目2:: ラベル文字: Green, 大きさ: 0.1, 色: #55fe55(少し明るい緑)
＃ 項目3:: ラベル文字: Blue, 大きさ: 0.25, 色: #5555fe(少し明るい青)
＃ 項目4:: ラベル文字: Other, 大きさ: 0.15, 色: #999(灰色)
＄円グラフ結果＝（お好きな登録名,path/to/output.png,100,100,40,0,0,0,sans-serif,10,20,Red,0.5,#fe5555,Green,0.1,#55fe55,Blue,0.25,#5555fe,Other,0.15,#999）
＞円グラフ成功	（円グラフ結果）==0
＞円グラフ失敗
```

## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [winapi\_rs](https://github.com/retep998/winapi-rs) / Peter Atashian
+ [resvg](https://github.com/RazrFalcon/resvg) / Yevhenii Reizner
+ [png](https://github.com/image-rs/image-png) / The image-rs Developers
+ (テスト実行時) [encoding\_rs](https://github.com/hsivonen/encoding_rs) / Henri Sivonen
+ (テスト実行時) [tempfile](https://github.com/Stebalien/tempfile) / Steven Allen, The Rust Project Developers, Ashley Mannix, Jason White

また、自作ライブラリの[svg-pie-chart](https://github.com/tukinami/svg-pie-chart)を使用しています。

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
