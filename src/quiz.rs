use odyssey::{QuizMode, Runner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quiz {
    runner: Runner,
}

#[wasm_bindgen]
impl Quiz {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String, event_input: Option<String>) -> Result<Quiz, JsValue> {
        let runner = match event_input {
            Some(event_input) => Runner::new_with_events(&input, &event_input),
            None => Runner::new(&input),
        };

        match runner {
            Ok(runner) => Ok(Quiz { runner }),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        match self.runner.quiz_view().title() {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        match self.runner.quiz_view().description() {
            Some(description) => description.clone(),
            None => String::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn sections(&self) -> Result<JsValue, JsValue> {
        match serde_wasm_bindgen::to_value(&self.runner.quiz_view().sections()) {
            Ok(sections) => Ok(sections),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn section(&self, section_number: usize) -> Result<JsValue, JsValue> {
        match self.runner.section_view(section_number) {
            Ok(view) => Ok(serde_wasm_bindgen::to_value(&view)?),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn question(&self, question_number: usize) -> Result<JsValue, JsValue> {
        match self.runner.question_view(question_number) {
            Ok(view) => Ok(serde_wasm_bindgen::to_value(&view)?),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn select_answers(
        &mut self,
        question_number: usize,
        answer_numbers: Vec<usize>,
    ) -> Result<(), JsValue> {
        match self.runner.select_answers(question_number, answer_numbers) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn input_answers(
        &mut self,
        question_number: usize,
        inputs: JsValue,
    ) -> Result<(), JsValue> {
        let inputs: Vec<String> = serde_wasm_bindgen::from_value(inputs)?;

        match self.runner.input_answers(question_number, inputs) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen]
    pub fn clear_answers(&mut self, question_number: usize) -> Result<(), JsValue> {
        match self.runner.clear_answers(question_number) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn event_log(&self) -> Result<JsValue, JsValue> {
        match serde_wasm_bindgen::to_value(&self.runner.event_log()) {
            Ok(log) => Ok(log),
            Err(err) => Err(serde_wasm_bindgen::to_value(&format!("{:?}", err))?),
        }
    }
}
