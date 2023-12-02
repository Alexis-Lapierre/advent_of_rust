#[derive(Debug)]
pub struct AOCResult {
    silver: String,
    gold: String,
}

impl<SILVER, GOLD> From<(SILVER, GOLD)> for AOCResult
where
    SILVER: ToString,
    GOLD: ToString,
{
    fn from((silver, gold): (SILVER, GOLD)) -> Self {
        Self {
            silver: silver.to_string(),
            gold: gold.to_string(),
        }
    }
}

impl std::fmt::Display for AOCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.silver, self.gold)
    }
}
