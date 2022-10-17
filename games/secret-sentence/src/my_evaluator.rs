
use genetic_rs::evaluator::Evaluator;

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

pub struct LevensteinEvaluation {
    pub solution: String,
}

impl LevensteinEvaluation {

    fn levenshtein_distance(&self, lhs: &String, rhs: &String) -> usize {
        let l_len = lhs.chars().count() + 1;
        let r_len = rhs.chars().count() + 1;
    
        let vlen = l_len * r_len;
        let mut dist = vec![0; vlen];
    
        for i in 1..l_len {
            dist[i * r_len] = i;
        }
        for i in 1..r_len {
            dist[i] = i;
        }
    
        for i in 1..r_len {
            for j in 1..l_len {
                let lchar = lhs.chars().nth(j - 1).unwrap();
                let rchar = rhs.chars().nth(i - 1).unwrap();
                let cost = if lchar == rchar { 0 } else { 1 };
    
                let d = (dist[(i - 1) + (j - 1) * r_len] + cost)
                    .min(dist[i + (j - 1) * r_len] + 1)
                    .min(dist[(i - 1) + j * r_len] + 1);
    
                dist[i + j * r_len] = d;
            }
        }
    
        dist[vlen - 1]
    }
}

impl Evaluator<String> for LevensteinEvaluation {

    fn evaluator(&self, pop: &String) -> f32 {

        (self.solution.len() - self.levenshtein_distance(&self.solution, pop)) as f32
    }
}
