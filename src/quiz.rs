use odyssey::{parse, Id, Runner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quiz {
    runner: Runner,
}

#[wasm_bindgen]
impl Quiz {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Option<Quiz> {
        match parse(&input) {
            Ok(quiz) => Some(Quiz {
                runner: Runner::new(quiz),
            }),
            _ => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.runner.quiz_view().unwrap().title.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.runner.quiz_view().unwrap().description.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn points(&self) -> JsValue {
        let points = self.runner.points();

        JsValue::from_serde(&points).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn sections(&self) -> JsValue {
        let view = self.runner.quiz_view().unwrap();

        JsValue::from_serde(&view.sections).unwrap()
    }

    #[wasm_bindgen]
    pub fn section(&self, section_number: Id) -> JsValue {
        let view = self.runner.section_view(section_number).unwrap();

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn question(&self, section_number: Id, question_number: Id) -> JsValue {
        let view = self
            .runner
            .question_view(section_number, question_number)
            .unwrap();

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn answer(&self, section_number: Id, question_number: Id, answer_number: Id) -> JsValue {
        let view = self
            .runner
            .answer_view(section_number, question_number, answer_number)
            .unwrap();

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn select_answer(
        mut self,
        section_number: Id,
        question_number: Id,
        answer_number: Id,
    ) -> Self {
        self.runner = self
            .runner
            .select_answer(section_number, question_number, answer_number)
            .unwrap();

        self
    }

    #[wasm_bindgen]
    pub fn input_answer(
        mut self,
        section_number: Id,
        question_number: Id,
        answer_number: Id,
        input: String,
    ) -> Self {
        self.runner = self
            .runner
            .input_answer(section_number, question_number, answer_number, input)
            .unwrap();

        self
    }
}
