# Web testing

Prerequisites:
* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

In root directory:
```bash
wasm-pack build
```

In `www` directory:
```bash
npm install
npm start
```

Open browser at `localhost:8080` and type `q = create_simple_quiz()`.

Now you can interact with quiz:
```js
q = create_simple_quiz()
q.title
q.description
q.sections
q.section(1)
q.section(2)
q.question(1, 1)
q.select_answer(1, 1, 1)
q.next_question()
```
