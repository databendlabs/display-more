// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use chrono::DateTime;
use chrono::Utc;

pub struct DisplayUnixTimeStamp {
    /// The duration since the UNIX epoch.
    duration: Option<Duration>,

    in_millis: bool,

    with_timezone: bool,
}

impl fmt::Display for DisplayUnixTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = match self.duration {
            Some(d) => d,
            None => return write!(f, "None"),
        };

        let system_time = UNIX_EPOCH + duration;
        let datetime: DateTime<Utc> = system_time.into();

        let fmt = if self.in_millis {
            if self.with_timezone {
                "%Y-%m-%dT%H:%M:%S%.3fZ%z"
            } else {
                "%Y-%m-%dT%H:%M:%S%.3f"
            }
        } else if self.with_timezone {
            "%Y-%m-%dT%H:%M:%S%.6fZ%z"
        } else {
            "%Y-%m-%dT%H:%M:%S%.6f"
        };

        write!(f, "{}", datetime.format(fmt))
    }
}

impl DisplayUnixTimeStamp {
    pub fn new(duration: Option<Duration>) -> Self {
        Self {
            duration,
            in_millis: false,
            with_timezone: true,
        }
    }

    pub fn in_millis(self, in_millis: bool) -> Self {
        Self { in_millis, ..self }
    }

    pub fn with_timezone(self, with_timezone: bool) -> Self {
        Self {
            with_timezone,
            ..self
        }
    }
}

/// Implement `Display` for `Duration` to display the duration since the UNIX epoch.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use display_more::DisplayUnixTimeStampExt;
///
/// let duration = Duration::from_millis(1723102819023);
/// assert_eq!(
///     duration.display_unix_timestamp().to_string(),
///     "2024-08-08T07:40:19.023000Z+0000"
/// );
/// ```
pub trait DisplayUnixTimeStampExt {
    fn display_unix_timestamp(&self) -> DisplayUnixTimeStamp;

    /// Display the duration since the UNIX epoch in milliseconds without timezone.
    fn display_unix_timestamp_short(&self) -> DisplayUnixTimeStamp {
        self.display_unix_timestamp()
            .in_millis(true)
            .with_timezone(false)
    }
}

impl DisplayUnixTimeStampExt for Duration {
    fn display_unix_timestamp(&self) -> DisplayUnixTimeStamp {
        DisplayUnixTimeStamp::new(Some(*self))
    }
}

impl DisplayUnixTimeStampExt for Option<Duration> {
    fn display_unix_timestamp(&self) -> DisplayUnixTimeStamp {
        DisplayUnixTimeStamp::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_display_unix_epoch() {
        let epoch = Duration::from_millis(0);
        let display = epoch.display_unix_timestamp();
        assert_eq!(format!("{}", display), "1970-01-01T00:00:00.000000Z+0000");

        let epoch = Duration::from_millis(1723102819023);
        let display = epoch.display_unix_timestamp();
        assert_eq!(format!("{}", display), "2024-08-08T07:40:19.023000Z+0000");

        let display = epoch.display_unix_timestamp_short();
        assert_eq!(format!("{}", display), "2024-08-08T07:40:19.023");

        // Option<Duration>: Some
        let some = Some(Duration::from_millis(1723102819023));
        assert_eq!(
            some.display_unix_timestamp().to_string(),
            "2024-08-08T07:40:19.023000Z+0000"
        );
        assert_eq!(
            some.display_unix_timestamp_short().to_string(),
            "2024-08-08T07:40:19.023"
        );

        // Option<Duration>: None
        let none: Option<Duration> = None;
        assert_eq!(none.display_unix_timestamp().to_string(), "None");
        assert_eq!(none.display_unix_timestamp_short().to_string(), "None");
    }
}
