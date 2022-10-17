use error_chain::bail;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};
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
        self.to_hashable_string(true).hash(state);
    }
}

impl PathValue {
    pub fn from_string(i: &str) -> crate::errors::Result<PathValue> {
        let p: Vec<PathSegment> =
            PathParser::from(i).collect::<Result<Vec<PathSegment>, svgtypes::Error>>()?;
        // Convert the segments to relative
        let mut new_segs = Vec::new();
        if let Some(first) = p.first() {
            if let PathSegment::MoveTo { abs: _, x, y } = first {
                new_segs.push(PathSegment::MoveTo {
                    abs: true,
                    x: *x,
                    y: *y,
                });
                let mut last_pos = (*x, *y);
                for seg in p.into_iter().skip(1) {
                    let (new_seg, lp) = Self::relative_path_segment(seg, last_pos);
                    last_pos = lp;
                    new_segs.push(new_seg);
                }
            } else {
                bail!("path does not begin with move to");
            }
        }
        Ok(PathValue { segments: new_segs })
    }

    fn relative_path_segment(orig: PathSegment, last_pos: (f64, f64)) -> (PathSegment, (f64, f64)) {
        match orig {
            PathSegment::MoveTo { abs, x, y } => {
                if abs {
                    (
                        PathSegment::MoveTo {
                            abs: false,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::LineTo { abs, x, y } => {
                if abs {
                    (
                        PathSegment::LineTo {
                            abs: false,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::HorizontalLineTo { abs, x } => {
                if abs {
                    (
                        PathSegment::HorizontalLineTo {
                            abs: false,
                            x: x - last_pos.0,
                        },
                        (x, last_pos.1),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1))
                }
            }
            PathSegment::VerticalLineTo { abs, y } => {
                if abs {
                    (
                        PathSegment::VerticalLineTo {
                            abs: false,
                            y: y - last_pos.0,
                        },
                        (last_pos.0, y),
                    )
                } else {
                    (orig, (last_pos.0, last_pos.1 + y))
                }
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
                if abs {
                    (
                        PathSegment::CurveTo {
                            abs: false,
                            x1: x1 - last_pos.0,
                            y1: y1 - last_pos.1,
                            x2: x2 - last_pos.0,
                            y2: y2 - last_pos.1,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                if abs {
                    (
                        PathSegment::SmoothCurveTo {
                            abs: false,
                            x2: x2 - last_pos.0,
                            y2: y2 - last_pos.1,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => {
                if abs {
                    (
                        PathSegment::Quadratic {
                            abs: false,
                            x1: x1 - last_pos.0,
                            y1: y1 - last_pos.1,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::SmoothQuadratic { abs, x, y } => {
                if abs {
                    (
                        PathSegment::SmoothQuadratic {
                            abs: false,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
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
                if abs {
                    (
                        PathSegment::EllipticalArc {
                            abs: false,
                            rx: rx - last_pos.0,
                            ry: ry - last_pos.1,
                            x_axis_rotation,
                            large_arc,
                            sweep,
                            x: x - last_pos.0,
                            y: y - last_pos.1,
                        },
                        (x, y),
                    )
                } else {
                    (orig, (last_pos.0 + x, last_pos.1 + y))
                }
            }
            PathSegment::ClosePath { abs } => {
                if abs {
                    (PathSegment::ClosePath { abs: false }, last_pos)
                } else {
                    (orig, last_pos)
                }
            }
        }
    }

    pub fn hash_with_modifier<H: Hasher>(&self, with_pos: bool, hasher: &mut H) {
        self.to_hashable_string(with_pos).hash(hasher);
    }

    pub fn to_hashable_string(&self, with_pos: bool) -> String {
        let mut res = "".to_string();
        for seg in self.segments.iter().skip(if with_pos { 0 } else { 1 }) {
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
                    PathSegment::ClosePath { abs } => (if *abs { "Z" } else { "z" }).to_string(),
                }
                .as_str(),
            );
        }
        res.to_string()
    }
}

impl Display for PathValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for seg in self.segments.iter() {
            match seg {
                PathSegment::MoveTo { abs, x, y } => {
                    write!(f, "{} {} {}", if *abs { "M" } else { "m" }, x, y)?
                }
                PathSegment::LineTo { abs, x, y } => {
                    write!(f, "{} {} {}", if *abs { "L" } else { "l" }, x, y)?
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    write!(f, "{} {}", if *abs { "H" } else { "h" }, x)?
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    write!(f, "{} {}", if *abs { "V" } else { "v" }, y)?
                }
                PathSegment::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => write!(
                    f,
                    "{} {} {}, {} {}, {} {}",
                    if *abs { "C" } else { "c" },
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y
                )?,
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => write!(
                    f,
                    "{} {} {}, {} {}",
                    if *abs { "S" } else { "s" },
                    x2,
                    y2,
                    x,
                    y
                )?,
                PathSegment::Quadratic { abs, x1, y1, x, y } => write!(
                    f,
                    "{} {} {}, {} {}",
                    if *abs { "Q" } else { "q" },
                    x1,
                    y1,
                    x,
                    y
                )?,
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    write!(f, "{} {} {}", if *abs { "T" } else { "t" }, x, y)?
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
                } => write!(
                    f,
                    "{} {} {} {} {} {} {} {}",
                    rx,
                    ry,
                    if *abs { "A" } else { "a" },
                    x_axis_rotation,
                    if *large_arc { 1 } else { 0 },
                    if *sweep { 1 } else { 0 },
                    x,
                    y
                )?,
                PathSegment::ClosePath { abs } => write!(f, "{}", if *abs { "Z" } else { "z" })?,
            };
        }
        Ok(())
    }
}
