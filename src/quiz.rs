use odyssey::{Id, Quiz as QuizData};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quiz {
    quiz_data: QuizData,
}

#[wasm_bindgen]
impl Quiz {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Option<Quiz> {
        match QuizData::new(&input) {
            Ok(quiz_data) => Some(Quiz { quiz_data }),
            _ => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.quiz_data.title().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.quiz_data.description().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn sections(&self) -> JsValue {
        let view = self.quiz_data.quiz_view();

        JsValue::from_serde(&view.sections).unwrap()
    }

    #[wasm_bindgen]
    pub fn section(&self, section_number: Id) -> JsValue {
        let view = self.quiz_data.section_view(section_number);

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn question(&self, section_number: Id, question_number: Id) -> JsValue {
        let view = self
            .quiz_data
            .question_view(section_number, question_number);

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn answer(&self, section_number: Id, question_number: Id, answer_number: Id) -> JsValue {
        let view = self
            .quiz_data
            .answer_view(section_number, question_number, answer_number);

        JsValue::from_serde(&view).unwrap()
    }

    #[wasm_bindgen]
    pub fn select_answer(
        &mut self,
        section_number: Id,
        question_number: Id,
        answer_number: Id,
    ) -> JsValue {
        let events = self
            .quiz_data
            .select_answer(section_number, question_number, answer_number)
            .unwrap();

        JsValue::from_serde(&events).unwrap()
    }

    #[wasm_bindgen]
    pub fn input_answer(
        &mut self,
        section_number: Id,
        question_number: Id,
        answer_number: Id,
        input: String,
    ) -> JsValue {
        let events = self
            .quiz_data
            .input_answer(section_number, question_number, answer_number, input)
            .unwrap();

        JsValue::from_serde(&events).unwrap()
    }

    #[wasm_bindgen]
    pub fn next_question(&self) -> JsValue {
        let view = self.quiz_data.next_question();

        JsValue::from_serde(&view).unwrap()
    }
}
