import * as wasm from "odyssey";

const simple_quiz_input = `
{
  "title": "Simple quiz",
  "description": "Basic quiz without any fancy interactions",
  "sections": [
    {
      "title": "Section A",
      "content": "First section",
      "questions": [
        {
          "title": "Question AA",
          "content": "First question in first section",
          "mode": "select",
          "answers": [
            { "content": "Answer AAA" },
            { "content": "Answer AAB" },
            { "content": "Answer AAC" }
          ]
        },
        {
          "title": "Question AB",
          "content": "Second question in first section",
          "mode": "multiSelect",
          "answers": [
            { "content": "Answer ABA" },
            { "content": "Answer ABB" },
            { "content": "Answer ABC" }
          ]
        }
      ]
    },
    {
      "title": "Section B",
      "content": "Second section",
      "questions": [
        {
          "title": "Question BA",
          "content": "First question in second section",
          "mode": "select",
          "answers": [
            { "content": "Answer BAA" },
            { "content": "Answer BAB" },
            { "content": "Answer BAC" }
          ]
        },
        {
          "title": "Question BB",
          "content": "Second question in second section",
          "mode": "input",
          "answers": [
            { "content": "Answer BBA" }
          ]
        }
      ]
    }
  ]
}
`

const point_quiz_input = `
{
  "title": "Point quiz",
  "description": "Point counting quiz with conditionals based on amount of points",
  "points": {
    "good": "A good person score",
    "bad": "A bad person score"
  },
  "sections": [
    {
      "title": "Section A",
      "content": "First section",
      "questions": [
        {
          "title": "Question AA",
          "content": "First question in first section",
          "mode": "select",
          "answers": [
            { "content": "Answer AAA", "triggers": [{ "condition": true, "actions": [{"changePoint": {"good": 1}}]}] },
            { "content": "Answer AAB", "triggers": [{"changePoint": {"bad": 1}}] },
            { "content": "Answer AAC" }
          ]
        },
        {
          "title": "Question AB",
          "content": "Second question in first section",
          "mode": "multiSelect",
          "visible": {"gt": [{"get": ["answeredQuestionCount"]}, 0]},
          "answers": [
            { "content": "Answer ABA", "triggers": [{ "condition": true, "actions": [{"changePoint": {"good": 1}}]}] },
            { "content": "Answer ABB", "triggers": [{"changePoint": {"bad": 1}}] },
            { "content": "Answer ABC" }
          ]
        }
      ]
    },
    {
      "title": "Section B",
      "content": "Second section",
      "questions": [
        {
          "title": "Question BA",
          "content": "First question in second section",
          "mode": "select",
          "visible": {"gt": [{"get": ["goodPointCount"]}, 1]},
          "answers": [
            { "content": "Answer BAA" },
            { "content": "Answer BAB" },
            { "content": "Answer BAC" }
          ]
        },
        {
          "title": "Question BB",
          "content": "Second question in second section",
          "mode": "input",
          "visible": {"gt": [{"get": ["badPointCount"]}, 1]},
          "answers": [
            { "content": "Answer BBA" }
          ]
        }
      ]
    }
  ]
}
`

window.wasm = wasm;

window.create_simple_quiz = function() { return new wasm.Quiz(simple_quiz_input) }
window.create_point_quiz = function() { return new wasm.Quiz(point_quiz_input) }
