use crate::calculations::{Interpolable, ScalesTo, ScalesWith};
use crate::util::pairwise;

#[derive(Debug, Clone, PartialEq)]
pub struct FanCurve<OperatingPoint> {
    points: Vec<OperatingPoint>,
}

impl<OP> FromIterator<OP> for FanCurve<OP> {
    // Required method
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = OP>,
    {
        FanCurve {
            points: Vec::<OP>::from_iter(iter),
        }
    }
}

impl<OP> IntoIterator for FanCurve<OP> {
    type Item = OP;
    type IntoIter = <Vec<OP> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<T> AsRef<Vec<T>> for FanCurve<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.points
    }
}

// impl <T> AsVe<Vec<T>> for FanCurve<T> {
//     fn as_ref(&self) -> &Vec<T> {
//         &self.points
//     }
// }

impl<T, OP> ScalesWith<T> for FanCurve<OP>
where
    OP: ScalesWith<T>,
    T: PartialOrd, // Slight hack so we dont impliment scaling a fan curve be one of its operating points.
                   // This kinda makes sense if you think about members of the perating points
                   // being orderable amongst themselves, but a whole OP not being comparable
                   // Perhaps this is better captured on the ScalesWith type itself
{
    fn scale(self, from: &T, to: &T) -> Self {
        self.into_iter().map(|op| op.scale(from, to)).collect()
    }
}

impl<OP, T> ScalesTo<T> for FanCurve<OP>
where
    OP: ScalesTo<T>,
{
    fn scale_to(self, new_airflow: &T) -> Self {
        self.into_iter()
            .map(|op| op.scale_to(new_airflow))
            .collect()
    }
}

pub trait InterpolableFanCurve<X, OP>
where
    X: Clone + PartialOrd,
    OP: AsRef<X> + Interpolable<X>,
    Self: AsRef<Vec<OP>>,
{
    fn interpolation_pairs(&self) -> Vec<(X, OP)> {
        let mut ops = self.as_ref().clone();
        ops.sort_by(|a, b| {
            let sp: &X = a.as_ref();
            sp.partial_cmp(b.as_ref()).unwrap()
        });
        ops.iter()
            .map(|op| {
                let sp: &X = op.as_ref();
                (sp.clone(), op.clone())
            })
            .collect()
    }

    fn interpolate(&self, target: &X) -> Result<OP, String> {
        let bounds = pairwise(self.interpolation_pairs())
            .find(|(_, (static_pressure, _))| static_pressure >= target);
        //set up variables
        if let Some((store_low, store_high)) = bounds {
            Ok(OP::interpolate_between(store_low, store_high, target))
        } else {
            Err("Out of bounds".to_string())
        }
    }
}
