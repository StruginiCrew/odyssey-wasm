import * as wasm from "odyssey";

const simple_quiz_input = `
{
  "title": "Simple quiz",
  "description": "Basic quiz without any fancy interactions",
  "sections": {
    "1": {
      "title": "Section A",
      "content": "First section",
      "questions": {
        "1": {
          "title": "Question AA",
          "content": "First question in first section",
          "mode": "SingleSelect",
          "answers": {
            "1": { "content": "Answer AAA" },
            "2": { "content": "Answer AAB" },
            "3": { "content": "Answer AAC" }
          }
        },
        "2": {
          "title": "Question AB",
          "content": "Second question in second section",
          "mode": "MultipleSelect",
          "answers": {
            "1": { "content": "Answer ABA" },
            "2": { "content": "Answer ABB" },
            "3": { "content": "Answer ABC" }
          }
        }
      }
    },
    "2": {
      "title": "Section B",
      "content": "Second section",
      "questions": {
        "1": {
          "title": "Question BA",
          "content": "First question in second section",
          "mode": "SingleSelect",
          "answers": {
            "1": { "content": "Answer BAA" },
            "2": { "content": "Answer BAB" },
            "3": { "content": "Answer BAC" }
          }
        },
        "2": {
          "title": "Question BB",
          "content": "Second question in second section",
          "mode": "SingleInput",
          "answers": {
            "1": { "content": "Answer BBA" }
          }
        }
      }
    }
  }
}
`

window.wasm = wasm;

window.create_simple_quiz = function() { return new wasm.Quiz(simple_quiz_input) }
