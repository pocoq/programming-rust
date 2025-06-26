#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl TimeUnit {
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "June 26, 2019, at 9:49 AM".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

pub fn handle_rough_time() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
	let a_hour_ago = RoughTime::InTheFuture(TimeUnit::Hours, 1);

    rough_time_to_english(four_score_and_seven_years_ago);
    rough_time_to_english(three_hours_from_now);
    rough_time_to_english(RoughTime::JustNow);
	rough_time_to_english(a_hour_ago);

    println!(
        "Four score and seven years ago: {}",
        rough_time_to_english(four_score_and_seven_years_ago)
    );
    println!(
        "Three hours from now: {}",
        rough_time_to_english(three_hours_from_now)
    );
    println!("Just now: {}", rough_time_to_english(RoughTime::JustNow));
	println!("An hour ago: {}", rough_time_to_english(a_hour_ago));
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
		RoughTime::InTheFuture(units, 1) => format!("a {} from now",  units.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Sphere { center: f64, radius: f64 },
    Cuboid { corner1: f64, corner2: f64 },
}

pub fn handle_shape() {
    let unit_sphere = Shape::Sphere {
        center: 2.0,
        radius: 1.0,
    };
    let unit_cuboid = Shape::Cuboid {
        corner1: 0.0,
        corner2: 1.0,
    };
    println!("Unit sphere: {:?}", unit_sphere);
    println!("Unit cuboid: {:?}", unit_cuboid);
}
