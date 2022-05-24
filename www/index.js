import * as wasm from "odyssey";

const open_survey_quiz_input = `
{
  "title": "Open survey quiz",
  "description": "A free form survey",
  "mode": "open",
  "sections": [
    {
      "id": 1,
      "title": "s1 title",
      "description": "s1 description",
      "questions": [
        {
          "id": 1,
          "title": "q1 title",
          "content": "q1 content",
          "mode": "select",
          "minEntries": 1,
          "maxEntries": 1,
          "answers": [
            {
              "id": 1,
              "content": "q1a1 content"
            },
            {
              "id": 2,
              "content": "q1a2 content"
            }
          ]
        },
        {
          "id": 2,
          "title": "q2 title",
          "content": "q2 content",
          "mode": "select",
          "minEntries": 2,
          "answers": [
            {
              "id": 1,
              "content": "q2a1 content"
            },
            {
              "id": 2,
              "content": "q2a2 content"
            },
            {
              "id": 3,
              "content": "q2a3 content"
            }
          ]
        }
      ]
    },
    {
      "id": 2,
      "title": "s2 title",
      "description": "s2 description",
      "questions": [
        {
          "id": 3,
          "title": "q3 title",
          "content": "q3 content",
          "mode": "input",
          "maxEntries": 1
        },
        {
          "id": 4,
          "title": "q4 title",
          "content": "q4 content",
          "mode": "input",
          "maxEntries": 3,
          "minEntries": 1
        }
      ]
    }
  ]
}
`

const open_exam_quiz_input = `
{
  "title": "Open exam quiz",
  "description": "A free form exam",
  "mode": "open",
  "sections": [
    {
      "id": 1,
      "title": "s1 title",
      "description": "s1 description",
      "questions": [
        {
          "id": 1,
          "title": "q1 title",
          "content": "q1 content",
          "mode": "select",
          "minEntries": 1,
          "maxEntries": 1,
          "answers": [
            {
              "id": 1,
              "content": "q1a1 content",
              "correct": true
            },
            {
              "id": 2,
              "content": "q1a2 content"
            }
          ]
        },
        {
          "id": 2,
          "title": "q2 title",
          "content": "q2 content",
          "mode": "select",
          "minEntries": 2,
          "minCorrectEntries": 1,
          "answers": [
            {
              "id": 1,
              "content": "q2a1 content",
              "correct": true
            },
            {
              "id": 2,
              "content": "q2a2 content",
              "correct": true
            },
            {
              "id": 3,
              "content": "q2a3 content"
            }
          ]
        }
      ]
    },
    {
      "id": 2,
      "title": "s2 title",
      "description": "s2 description",
      "questions": [
        {
          "id": 3,
          "title": "q3 title",
          "content": "q3 content",
          "mode": "input",
          "maxEntries": 1
        },
        {
          "id": 4,
          "title": "q4 title",
          "content": "q4 content",
          "mode": "input",
          "maxEntries": 3,
          "minEntries": 1
        }
      ]
    }
  ]
}
`

const event_input = JSON.stringify([
  { event: "selectAnswer", questionId: 1, answerId: 1}
])

window.wasm = wasm;

window.event_input = event_input
window.create_open_survey = function(event_input) { return new wasm.Quiz(open_survey_quiz_input, event_input) }
window.create_open_exam = function(event_input) { return new wasm.Quiz(open_exam_quiz_input, event_input) }
