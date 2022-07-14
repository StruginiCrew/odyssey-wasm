import * as wasm from "odyssey";

const open_exam_quiz_input = `
{
  "uid": "open_exam_quiz",
  "version": 1,
  "title": "Animals exam",
  "description": "How well do you know animals?",
  "mode": "open",
  "minAnsweredQuestions": 4,
  "minCorrectQuestions": 4,
  "maxWrongQuestions": 1,
  "blockAnswerUpdatesFor": ["answeredCorrectly"],
  "sections": [
    {
      "id": 1,
      "title": "Pets",
      "description": "Animals we keep at home",
      "questions": [
        {
          "id": 1,
          "content": "Which animals can be considered pets?",
          "mode": "select",
          "minCorrectEntries": 2,
          "correctEntryMatch": { "id": [1, 2] },
          "answers": [
            { "id": 1, "content": "Cat" },
            { "id": 2, "content": "Dog" },
            { "id": 3, "content": "Lion" }
          ]
        },
        {
          "id": 2,
          "content": "Select two smallest pets",
          "mode": "select",
          "minEntries": 2,
          "maxEntries": 2,
          "minCorrectEntries": 2,
          "correctEntryMatch": { "id": [2, 3] },
          "answers": [
            { "id": 1, "content": "Cat" },
            { "id": 2, "content": "Hamster" },
            { "id": 3, "content": "Rat" },
            { "id": 4, "content": "Dog" }
          ]
        }
      ]
    },
    {
      "id": 2,
      "title": "Farm animals",
      "description": "Animals at farms",
      "questions": [
        {
          "id": 3,
          "optional": true,
          "content": "Which farm animal gives us milk?",
          "mode": "input",
          "minEntries": 2,
          "maxEntries": 2,
          "minCorrectEntries": 2,
          "correctEntryMatch": { "content": ["cow", "goat"] }
        },
        {
          "id": 4,
          "content": "Name at least 3 farm animals",
          "mode": "select",
          "minEntries": 3,
          "minCorrectEntries": 2,
          "maxWrongEntries": 1,
          "correctEntryMatch": { "content": ["cow", "pig", "horse"] },
          "answers": [
            { "id": 1, "content": "Cow" },
            { "id": 2, "content": "Lizard" },
            { "id": 3, "content": "Pig" },
            { "id": 4, "content": "Giraffe" },
            { "id": 5, "content": "Horse" }
          ]
        }
      ]
    }
  ]
}
`

const open_exam_event_log_input = `
{
  "uid": "open_exam_quiz",
  "version": 1,
  "events": [
    { "event": "selectAnswers", "questionId": 1, "answerIds": [1, 2] },
    { "event": "selectAnswers", "questionId": 2, "answerIds": [3, 4] },
    { "event": "clearAnswers", "questionId": 2 },
    { "event": "selectAnswers", "questionId": 2, "answerIds": [2, 3] },
    { "event": "inputAnswers", "questionId": 3, "inputs": ["Cow"] },
    { "event": "selectAnswers", "questionId": 4, "answerIds": [1, 2, 3] }
  ]
}
`

window.wasm = wasm;

window.open_exam_event_log_input = open_exam_event_log_input
window.create_open_exam = function(event_input) { return new wasm.Quiz(open_exam_quiz_input, event_input) }
