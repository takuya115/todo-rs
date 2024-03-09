# ユースケース

```plantuml
left to right direction
package usecase {
    usecase "todoを作成する" as uc1
    usecase "todoを完了する" as uc2
    usecase "todoを取得する" as uc3
}

package db {
    usecase "todoレコード\nを挿入する" as db1
    usecase "todoレコード\nを更新する" as db2
    usecase "todoレコード\n一覧を取得する" as db3
}

:User: --> uc1
uc1 --> db1
uc1 --> db3
:User: --> uc2
uc2 --> db2
:User: --> uc3
uc3 --> db3

```

---

## UC1-TODOを作成する

### UC1-エラー

- バリデーションエラー: 400
- DBエラー
  - DBに接続できない: 500
  - TODOが重複している: 400

## UC2-TODOを完了する

### UC2-エラー

- バリデーションエラー: 400
- DBエラー
  - DBに接続できない: 500
  - 対象のTODOが存在しない: 400

## UC3-todoを取得する

### UC3-エラー

- DBにエラー
  - DBに接続できない: 500
