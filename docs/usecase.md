# usecase


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
:User: --> uc2
uc2 --> db2
:User: --> uc3
uc3 --> db3

```
