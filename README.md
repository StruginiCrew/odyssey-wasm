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
q = q.select_answer(1, 1, 1)
```

To test the dynamic quiz:
```js
p = create_point_quiz()
p.title
p.points
p.section(1) // Should have 2 questions
p.section(2) // Should have 0 questions for now (visibility based on points)
p = p.select_answer(1, 1, 1)
p = p.select_answer(1, 2, 1)
p.points // Should have 2 good points
p.section(2) // Should have 1 question now
```
