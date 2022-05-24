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
q = create_open_survey()
q.title // Quiz title
q.description // Quiz description
q.sections // Check all available sections
q.section(1) // Check contents of Section 1
q.question(1) // Check answers and their IDs in Question 1
q.select_answer(1, 1) // Answer 1 in Question 1
q.question(1) // Check that Question 1 is now answered and Answer 1 is now selected
q.input_answer(3, "yo") // Input "yo" as answer to Question 3
q.question(1) // Check that Question 3 is now answered and Answer 1 is now selected and contains "yo"
```

Quiz can be restored with events:
```js
events_json = JSON.stringify([{event: "selectAnswer", questionId: 1, answerId: 1}])
q = create_open_survey(events_json)
q.question(1) // Check that Question 1 is now answered and Answer 1 is now selected
```
