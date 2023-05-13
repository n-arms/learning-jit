pub struct Bounded<const Low: i64, const High: i64> {
    value: i64,
}

pub enum OutOfBounds<const Low: i64, const High: i64> {
    TooHigh,
    TooLow,
}

impl<const Low: i64, const High: i64> TryFrom<i64> for Bounded<Low, High> {
    type Error = OutOfBounds<Low, High>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > High {
            Err(Self::Error::TooHigh)
        } else if value < Low {
            Err(Self::Error::TooLow)
        } else {
            Ok(Bounded { value })
        }
    }
}

impl<const Low: i64, const High: i64> Into<i64> for Bounded<Low, High> {
    fn into(self) -> i64 {
        self.value
    }
}
