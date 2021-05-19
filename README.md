[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/abplus-lab/makbe)


makbeという考え方について、ファームウェア以外にremapのようなノーコードでのキーアサインの設定や、ノーコードでのmoduloデバイスの組み合わせの設定なども加わってきた（きたはいいけど出来るのか？）ということと、社名を前面に出すのはなんか違う（makbeで直接儲けようとは思っていない）ということで、

* ファームウェア: https://github.com/kazhida/makbe-ff
* デバイスの構成（makbe-ffベース）: https://github.com/kazhida/makbe-rc
* QMKベースのリモートビルド・サービス: https://github.com/kazhida/makbe-rb

に分割して引っ越ししました。

----
# makbe

makbeとは、「~~Modulo Architecture の KeyBoard は Eよね！~~ Modulo Architecture KeyBoad Enhancer」の短縮形で、壺を愛でる大佐とは関係ありません。

冗談はさておき、**makbe** はModuloアーキテクチャに基づいて設計されたキーボードのファームウェアフレームワークとその関連サービスの総称です

### 最初のGOAL

examplesにあるように、Seeeduino XIAO で4x4のキーパッドを使えるようにします。

### もう一方のGOAL

とりあえず、Moduloデバイスを組み合わせたときのファームウェアやキーマッピングをビルド環境なしで行えるようなサービスを作れたらいいな（というか作らないとツラすぎ）と思っています。




