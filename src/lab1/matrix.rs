use rand::Rng;

pub(crate) struct Matrix {
    #[allow(dead_code)]
    pub size: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    const MAX: f64 = 99.0;

    pub(crate) fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| rng.gen::<f64>() + rng.gen::<f64>() * Self::MAX)
                    .collect()
            })
            .collect();

        Matrix { size, data }
    }

    #[allow(dead_code)]
    fn from_data(data: Vec<Vec<f64>>) -> Self {
        let size = data.len();
        Matrix { size, data }
    }

    #[allow(dead_code)]
    pub(crate) fn print(&self) {
        let size = std::cmp::min(self.size, 4);
        for i in 0..size {
            for j in 0..size {
                print!("{:.2}\t", self.data[i][j]);
            }
            println!();
        }
    }
}
