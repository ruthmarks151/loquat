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

impl<T, OP> ScalesTo<T> for FanCurve<OP>
where
    OP: ScalesWith<T> + AsRef<T>,
    T: PartialOrd + Clone,
    // Slight hack so we dont impliment scaling a fan curve be one of its operating points.
    // This kinda makes sense if you think about members of the perating points
    // being orderable amongst themselves, but a whole OP not being comparable
    // Perhaps this is better captured on the ScalesWith type itself
{
    fn scale_to(self, to: &T) -> Self {
        self.into_iter()
            .map(|op| {
                let from: T = op.as_ref().clone();
                op.scale(&from, to)
            })
            .collect()
    }
}

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

pub trait InterpolableFanCurve<X, Y>
where
    Y: Interpolable<X, Y>,
    X: PartialOrd + Clone,
{
    fn as_interpolation_vec(&self) -> Vec<(X, Y)>;

    fn interpolation_pairs(&self) -> Vec<(X, Y)> {
        let mut ops = self.as_interpolation_vec();
        ops.sort_by(|(a_x, _a_y), (b_x, _b_y)| a_x.partial_cmp(b_x).unwrap());
        ops
    }

    fn interpolate(&self, target: &X) -> Result<Y, String> {
        let bounds = pairwise(self.interpolation_pairs())
            .find(|((low_x, _), (high_x, _))| high_x >= target && low_x <= target);
        //set up variables
        if let Some((store_low, store_high)) = bounds {
            Ok(Y::interpolate_between(store_low, store_high, target))
        } else {
            Err("Out of bounds".to_string())
        }
    }
}

impl<X, Y, OP> InterpolableFanCurve<X, Y> for FanCurve<OP>
where
    OP: Clone + AsRef<X>,
    Y: Clone + From<OP> + Interpolable<X, Y>,
    X: Clone + PartialOrd,
{
    fn as_interpolation_vec(&self) -> Vec<(X, Y)> {
        self.clone()
            .into_iter()
            .map(|op| {
                let x: X = (op.as_ref() as &X).clone();
                let y: Y = op.into();
                (x, y)
            })
            .collect()
    }
}

pub trait FanCurveScalesWith<Env, OP>
where
    FanCurve<OP>: Clone + ScalesWith<Env>,
    OP: ScalesWith<Env>,
{
    fn current_value(&self) -> Env;

    fn fan_curve(&self) -> FanCurve<OP>;

    fn fan_curve_for_value(&self, new_constant: &Env) -> FanCurve<OP> {
        self.fan_curve().scale(&self.current_value(), new_constant)
    }
}
