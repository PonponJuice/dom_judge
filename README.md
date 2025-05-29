# setup

まず、`*.env` を `*.env.sample` を参考に作成してください。

次に以下のコマンドを用いて DOM-Judge のサーバーを起動することができます。
```bash
docker compose -f docker-compose-domserver.yml up -d
```

judgehost を起動する場合は、まず domserver の起動後に log の中にある judgeehost のパスワードを確認し、envファイルに設定してください。
その後、以下のコマンドを実行してください。
```bash
docker compose -f docker-compose-judgehost.yml up -d
```

## traQ Bot

もしも traQ 上で参加登録用のBotを動かしたい場合は、traq-bot 内の env ファイルを設定した上で以下のコマンドを実行してください。

```bash
docker compose -f docker-compose-traqbot.yml up -d
```

# Usage
DOM-Judge のサーバーが起動したら、`http://localhost:12345` にアクセスして DOM-Judge を利用することができます。

