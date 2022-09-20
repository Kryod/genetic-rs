pub trait Evaluator<T> {
    fn evaluator(&self, pop: &T) -> f32;
}

pub struct BasicEvaluation {
    pub solution: String,
}

impl Evaluator<String> for BasicEvaluation {

    fn evaluator(&self, pop: &String) -> f32 {
        let mut val = 0.0;
    
        for (c, cs) in pop.chars().zip(self.solution.chars()) {
            if c == cs {
                val += 1.0;
            } else {
                let diff = (c as u32).abs_diff(cs as u32) + 1;
                val += 1.8 * (1.0/diff as f32);
            }
        }
    
        val
    }
}

pub struct MotusEvaluation {
    pub solution: String,
}

impl Evaluator<String> for MotusEvaluation {

    fn evaluator(&self, pop: &String) -> f32 {
        let mut val = 0.0;
        let mut solution_chars = self.solution.char_indices();
        let mut correct_index_vec = vec![];
        let mut wrong_place_index_vec = vec![];
    
        for (index, c) in pop.char_indices() {
            if let Some((i, _)) = solution_chars.find(|(index_s, cs)| *cs == c && *index_s == index) {
                correct_index_vec.push(i);
                val += 1.0;
            }
        }
    
        for (_, c) in pop.char_indices() {
            if let Some((i, _)) = solution_chars.find(|(index_s, cs)| *cs == c && !correct_index_vec.contains(index_s) && !wrong_place_index_vec.contains(index_s)) {
    
                wrong_place_index_vec.push(i);
                val += 0.5;
            }
        }
    
        val
    }
}
