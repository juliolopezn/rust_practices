
#[derive(Debug)]
pub struct StatisticData {
    pub data: Vec<f32>,
    pub mean: f64,
}

impl StatisticData {
    pub fn new() -> Self {
        Self { 
            data: vec![], 
            mean: 0.0
        }
    }
    
    pub fn append(&mut self, val: f32) {
        self.data.push(val);
    }

    pub fn remove(&mut self, val: f32) {
        if let Some(pos) = self.data.iter().position(|&x| x == val) {
            self.data.remove(pos);
        }
    }

    fn compute_mean(&mut self) {
        self.mean = self.data.iter().sum::<f32>() as f64 / self.data.len() as f64;
    }
}

fn main() {
    let mut statistic_data = StatisticData::new();

    statistic_data.append(1.0);
    statistic_data.append(-1.0);
    statistic_data.append(-1.0);

    statistic_data.compute_mean();

    println!("{statistic_data:#?}");
}
