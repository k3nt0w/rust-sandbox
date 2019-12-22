## TCP と UDP について

### TCP

- まず server socket を生成する
- TCP syn が来るまで server_socket が thread を sleep させる
- accept のところで 3 way handshake
- stream (client socket) を 生成
- TCP ack: data が来るまで stream を sleep
- connection close まで繰り返し
- 他に syn が来たら新たに thread 生成して connection を貼る
  - こんな感じのサーバを並行サーバという
  - connection が生成されるたびに thread を作る必要がある（並行）

### UDP

- いきなり client socket を生成する
- data が来るまで thread を sleep
- コネクションは張らない。来たデータをとりあえず送り返すだけ。
  - こんな感じのサーバを反復サーバという
