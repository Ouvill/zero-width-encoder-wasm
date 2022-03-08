# zero_width_encoder

ゼロ幅文字にエンコード、デコードします｡ 

Rustで開発されたWebAssemblyのパッケージなので、一般的なnpmパッケージと動作が異なる場合があります。

開発段階であり、エンコード、デコードの方式の破壊的な変更があります。

This is WebAssembly npm package

## Install

```
npm install @ouvill/zero-width-encoder-wasm
```

or

```bash
yarn add @ouvill/zero-width-encoder-wasm
```

## Config

WebPackを利用する場合、以下の設定が必要

webpack.config.js

```
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
}
```

## Usage

```js
import { decode, encode } from '@ouvill/zero-width-encoder-wasm'

const encoded = encode("Hello World");
console.log(encoded)
const decoded = decode(encode)
console.log(decoded)
```

## build

```
wasm-pack build --scope ouvill
```

