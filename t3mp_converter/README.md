# Simple Markdown2HTML Converter
## How to Use
`test`フォルダに`test.md`を入れて`Cargo test`すると`./test/test/html`が作られる  
## 注意点
* テストの可否に`file`コマンドを使っているので、Linux外だと動作の保証ができない  
(fileコマンドを導入するか、テストを無理やり通るようにするかすればよさそう)  
* `./test`以下にあるCSSやJavaScriptに強く依存してる  
間違って消すと動かない...(将来的には自分の鯖から拾うようにHTML書き換えてコンバートする予定)  

