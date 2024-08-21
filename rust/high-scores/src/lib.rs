#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32]
}

impl<'a> HighScores <'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores{
            scores :scores
        }
    }

    pub fn scores(&self) -> &[u32] {

        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vector_list = self.scores.to_vec();
        vector_list.sort_unstable_by(|a,b| b.cmp(a));
        vector_list.into_iter().take(3).collect()
    }
}
