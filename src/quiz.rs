use odyssey::{Id, QuizMode, Runner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quiz {
    runner: Runner,
}

#[wasm_bindgen]
impl Quiz {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String, event_input: Option<String>) -> Quiz {
        let runner = match event_input {
            Some(event_input) => Runner::new_with_events(&input, &event_input).unwrap(),
            None => Runner::new(&input).unwrap(),
        };

        Quiz { runner }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Result<String, JsValue> {
        match self.runner.quiz_view() {
            Ok(quiz_view) => Ok(quiz_view.title()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Result<String, JsValue> {
        match self.runner.quiz_view() {
            Ok(quiz_view) => Ok(quiz_view.description()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn answered_questions_count(&self) -> Result<usize, JsValue> {
        match self.runner.quiz_view() {
            Ok(quiz_view) => Ok(quiz_view.answered_question_count()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Result<String, JsValue> {
        match self.runner.quiz_view() {
            Ok(quiz_view) => Ok(match quiz_view.mode() {
                QuizMode::Open => "open".to_string(),
                QuizMode::Linear => "linear".to_string(),
            }),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn sections(&self) -> Result<JsValue, JsValue> {
        match self.runner.quiz_view() {
            Ok(quiz_view) => Ok(serde_wasm_bindgen::to_value(&quiz_view.sections())?),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn section(&self, section_number: Id) -> Result<JsValue, JsValue> {
        match self.runner.section_view(section_number) {
            Ok(view) => Ok(serde_wasm_bindgen::to_value(&view)?),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn question(&self, question_number: Id) -> Result<JsValue, JsValue> {
        match self.runner.question_view(question_number) {
            Ok(view) => Ok(serde_wasm_bindgen::to_value(&view)?),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn select_answer(&mut self, question_number: Id, answer_number: Id) -> Result<(), JsValue> {
        match self
            .runner
            .select_answers(question_number, vec![answer_number])
        {
            Ok(_) => Ok(()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn input_answer(&mut self, question_number: Id, input: String) -> Result<(), JsValue> {
        match self.runner.input_answers(question_number, vec![input]) {
            Ok(_) => Ok(()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn select_answers(
        &mut self,
        question_number: Id,
        answer_numbers: Vec<Id>,
    ) -> Result<(), JsValue> {
        match self.runner.select_answers(question_number, answer_numbers) {
            Ok(_) => Ok(()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn input_answers(&mut self, question_number: Id, inputs: JsValue) -> Result<(), JsValue> {
        let inputs: Vec<String> = serde_wasm_bindgen::from_value(inputs)?;

        match self.runner.input_answers(question_number, inputs) {
            Ok(_) => Ok(()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn clear_answers(&mut self, question_number: Id) -> Result<(), JsValue> {
        match self.runner.clear_answers(question_number) {
            Ok(_) => Ok(()),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn event_log(&self) -> Result<JsValue, JsValue> {
        match serde_wasm_bindgen::to_value(&self.runner.event_log().unwrap().to_string()) {
            Ok(log) => Ok(log),
            Err(err) => return Err(serde_wasm_bindgen::to_value(&format!("{}", err))?),
        }
    }
}
