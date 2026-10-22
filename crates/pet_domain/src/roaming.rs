//! 原生視窗漫遊的純座標規則；不依賴 Tauri 或 OS。
#[derive(Clone, Copy, Debug)]
pub struct WorkArea {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, PartialEq)]
pub struct RoamStep {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}
pub fn bounded_step(
    position: (i32, i32),
    size: (u32, u32),
    area: WorkArea,
    velocity: (i32, i32),
) -> RoamStep {
    fn axis(position: i32, extent: u32, origin: i32, area: u32, velocity: i32) -> (i32, i32) {
        let min = i64::from(origin);
        let max = (min + i64::from(area) - i64::from(extent)).min(i64::from(i32::MAX));
        let speed = velocity.saturating_abs().max(1);
        if max <= min {
            return (origin, velocity);
        }
        let next = i64::from(position) + i64::from(velocity);
        if next <= min {
            (origin, speed)
        } else if next >= max {
            (max as i32, -speed)
        } else {
            (next as i32, velocity)
        }
    }
    let (x, dx) = axis(position.0, size.0, area.x, area.width, velocity.0);
    let (y, dy) = axis(position.1, size.1, area.y, area.height, velocity.1);
    RoamStep {
        position: (x, y),
        velocity: (dx, dy),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roaming_respects_negative_origins_bounces_and_recovers_from_oversize() {
        let area = WorkArea {
            x: -1920,
            y: -100,
            width: 1920,
            height: 1080,
        };
        let next = bounded_step((-1920, -100), (200, 200), area, (-4, -2));
        assert_eq!(next.position, (-1920, -100));
        assert_eq!(next.velocity, (4, 2));
        assert_eq!(
            bounded_step(next.position, (200, 200), area, next.velocity).position,
            (-1916, -98)
        );
        let pinned = bounded_step((0, 0), (4000, 3000), area, (4, 2));
        assert_eq!(pinned.position, (-1920, -100));
        assert_eq!(
            bounded_step(pinned.position, (200, 200), area, pinned.velocity).position,
            (-1916, -98)
        );
    }
    #[test]
    fn roaming_extreme_coordinates_do_not_overflow_or_leave_work_area() {
        let step = bounded_step(
            (i32::MAX, i32::MIN),
            (200, 200),
            WorkArea {
                x: i32::MAX - 500,
                y: i32::MIN,
                width: 500,
                height: 500,
            },
            (4, -2),
        );
        assert_eq!(step.position, (i32::MAX - 200, i32::MIN));
        assert_eq!(step.velocity, (-4, 2));
    }
}
