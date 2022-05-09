# 高速な MVP 開発

> node.js のバージョン管理は `volta` を使用しています  
> `volta install node@16`

## Next.js アプリ(frontend)の初期化

shell で以下コマンドを打ち、実行します

```shell
yarn create next-app --typescript fast-mvp-frontend
```

## Nest.js アプリ(backend)の初期化

```shell
yarn global add @nestjs/cli
nest new fast-mvp-backend
```

[nestjs#installation](https://docs.nestjs.com/#installation)

package manager は yarn を選択します

```? Which package manager would you ❤️  to use?
? Which package manager would you ❤️  to use?
  npm
❯ yarn
  pnpm
```

以下のように出力されると成功です

```
🚀  Successfully created project fast-mvp-backend
👉  Get started with the following commands:

$ cd fast-mvp-backend
$ yarn run start


                          Thanks for installing Nest 🙏
                 Please consider donating to our open collective
                        to help us maintain this package.


               🍷  Donate: https://opencollective.com/nest
```

このままだとコミットできないので `.git/` ディレクトリを削除して git 管理下から外す必要があります。

```
rm -rf fast-mvp-backend/.git
```
