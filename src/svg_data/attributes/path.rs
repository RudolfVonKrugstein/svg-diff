use serde::{Serialize, Serializer};
use std::hash::{Hash, Hasher};
use svgtypes;
use svgtypes::{PathParser, PathSegment};

#[derive(Debug, Clone, PartialEq)]
pub struct PathValue {
    segments: Vec<svgtypes::PathSegment>,
}

impl Serialize for PathValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

// Don't use for indexing hash maps!
// But this is good enough for comparing values for equality in our case.
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for PathValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_hashable_string().hash(state);
    }
}

impl PathValue {
    pub fn from_string(i: &String) -> crate::errors::Result<PathValue> {
        let p: Vec<PathSegment> =
            PathParser::from(i.as_str()).collect::<Result<Vec<PathSegment>, svgtypes::Error>>()?;
        Ok(PathValue { segments: p })
    }

    pub fn to_string(&self) -> String {
        let mut res = "".to_string();
        for seg in self.segments.iter() {
            res.push_str(
                match seg {
                    PathSegment::MoveTo { abs, x, y } => {
                        format!("{} {} {}", if *abs { "M" } else { "m" }, x, y)
                    }
                    PathSegment::LineTo { abs, x, y } => {
                        format!("{} {} {}", if *abs { "L" } else { "l" }, x, y)
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        format!("{} {}", if *abs { "H" } else { "h" }, x)
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        format!("{} {}", if *abs { "V" } else { "v" }, y)
                    }
                    PathSegment::CurveTo {
                        abs,
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        format!(
                            "{} {} {}, {} {}, {} {}",
                            if *abs { "C" } else { "c" },
                            x1,
                            y1,
                            x2,
                            y2,
                            x,
                            y
                        )
                    }
                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        format!(
                            "{} {} {}, {} {}",
                            if *abs { "S" } else { "s" },
                            x2,
                            y2,
                            x,
                            y
                        )
                    }
                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        format!(
                            "{} {} {}, {} {}",
                            if *abs { "Q" } else { "q" },
                            x1,
                            y1,
                            x,
                            y
                        )
                    }
                    PathSegment::SmoothQuadratic { abs, x, y } => {
                        format!("{} {} {}", if *abs { "T" } else { "t" }, x, y)
                    }
                    PathSegment::EllipticalArc {
                        abs,
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc,
                        sweep,
                        x,
                        y,
                    } => {
                        format!(
                            "{} {} {} {} {} {} {} {}",
                            rx,
                            ry,
                            if *abs { "A" } else { "a" },
                            x_axis_rotation,
                            if *large_arc { 1 } else { 0 },
                            if *sweep { 1 } else { 0 },
                            x,
                            y
                        )
                    }
                    PathSegment::ClosePath { abs } => {
                        format!("{}", if *abs { "Z" } else { "z" })
                    }
                }
                .as_str(),
            );
        }
        res.to_string()
    }

    pub fn to_hashable_string(&self) -> String {
        let mut res = "".to_string();
        for seg in self.segments.iter() {
            res.push_str(
                match seg {
                    PathSegment::MoveTo { abs, x, y } => {
                        format!("{} {:.8} {:.8}", if *abs { "M" } else { "m" }, x, y)
                    }
                    PathSegment::LineTo { abs, x, y } => {
                        format!("{} {:.8} {:.8}", if *abs { "L" } else { "l" }, x, y)
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        format!("{} {:.8}", if *abs { "H" } else { "h" }, x)
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        format!("{} {:.8}", if *abs { "V" } else { "v" }, y)
                    }
                    PathSegment::CurveTo {
                        abs,
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        format!(
                            "{} {:.8} {:.8}, {:.8} {:.8}, {:.8} {:.8}",
                            if *abs { "C" } else { "c" },
                            x1,
                            y1,
                            x2,
                            y2,
                            x,
                            y
                        )
                    }
                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        format!(
                            "{} {:.8} {:.8}, {:.8} {:.8}",
                            if *abs { "S" } else { "s" },
                            x2,
                            y2,
                            x,
                            y
                        )
                    }
                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        format!(
                            "{} {:.8} {:.8}, {:.8} {:.8}",
                            if *abs { "Q" } else { "q" },
                            x1,
                            y1,
                            x,
                            y
                        )
                    }
                    PathSegment::SmoothQuadratic { abs, x, y } => {
                        format!("{} {:.8} {:.8}", if *abs { "T" } else { "t" }, x, y)
                    }
                    PathSegment::EllipticalArc {
                        abs,
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc,
                        sweep,
                        x,
                        y,
                    } => {
                        format!(
                            "{} {:.8} {:.8} {:.8} {} {} {:.8} {:.8}",
                            if *abs { "A" } else { "a" },
                            rx,
                            ry,
                            x_axis_rotation,
                            if *large_arc { 1 } else { 0 },
                            if *sweep { 1 } else { 0 },
                            x,
                            y
                        )
                    }
                    PathSegment::ClosePath { abs } => {
                        format!("{}", if *abs { "Z" } else { "z" })
                    }
                }
                .as_str(),
            );
        }
        res.to_string()
    }
}
