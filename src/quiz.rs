use odyssey::{Id, QuizMode, Runner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quiz {
    runner: Runner,
}

#[wasm_bindgen]
impl Quiz {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Quiz {
        Quiz {
            runner: Runner::new(&input).unwrap(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.runner.quiz_view().unwrap().title()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.runner.quiz_view().unwrap().description()
    }

    #[wasm_bindgen(getter)]
    pub fn answered_questions_count(&self) -> usize {
        self.runner.quiz_view().unwrap().answered_question_count()
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> String {
        match self.runner.quiz_view().unwrap().mode() {
            QuizMode::Open => "open".to_string(),
            QuizMode::Linear => "linear".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn sections(&self) -> JsValue {
        let view = self.runner.quiz_view().unwrap();

        JsValue::from_serde(&view.sections()).unwrap()
    }

    #[wasm_bindgen]
    pub fn section(&self, section_number: Id) -> JsValue {
        let view = self.runner.section_view(section_number).unwrap();

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn question(&self, question_number: Id) -> JsValue {
        let view = self.runner.question_view(question_number).unwrap();

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn select_answer(&mut self, question_number: Id, answer_number: Id) {
        self.runner
            .select_answer(question_number, answer_number)
            .unwrap();
    }

    #[wasm_bindgen]
    pub fn input_answer(&mut self, question_number: Id, input: String) {
        self.runner.input_answer(question_number, input).unwrap();
    }
}
