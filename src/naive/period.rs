use naive::NaiveDateTime;
use oldtime::Duration;

use std::cmp::{min, max};

/// A period of time between two ISO 8601 dates/times ([NaiveDateTime](NaiveDateTime)s).
/// This is similar to a [`Duration`](Duration), except that it has a fixed start
/// date/time (and thus a fixed end date/time), allowing clients to determine if specific
/// segments of time intersect.
///
/// # Notes
/// This period is not reliant on time zones. For the time being, offsets aren't considered at all.
/// As such, if you need to interset two time periods that are of differing time zones, you're out
/// of luck (patches accepted, though).
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NaivePeriod {
    /// The date at which the period begins.
    pub start: NaiveDateTime,

    /// The date at which the period ends. This is inclusive, meaning that the period runs up to
    /// and including this date/time.
    pub end: NaiveDateTime
}

impl NaivePeriod {
    /// Create a new `NaivePeriod` from two [`NaiveDateTime`](NaiveDateTime) objects.
    ///
    /// # Arguments
    /// - `start`: A `NaiveDateTime` representing when the `NaivePeriod` will start.
    /// - `end`: A `NaiveDateTime` representing when the `NaivePeriod` will end. Note that this
    ///   date/time will be included in the period itself.
    ///
    /// # Returns
    /// - A `NaivePeriod` object having the specified start and end `NaiveDateTime`s.
    ///
    /// # Example
    /// ```
    /// # use chrono::{Duration, NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};
    /// let start = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np1 = NaivePeriod::new(start, end);
    ///
    /// assert_eq!(np1.duration(), Duration::days(366));
    /// ```
    #[inline]
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        NaivePeriod { start: start, end: end }
    }

    /// Create a new `NaivePeriod` from a [`NaiveDateTime`](NaiveDateTime) object and a
    /// [`Duration`](Duration) object.
    ///
    /// # Arguments
    /// - `start`: A `NaiveDateTime` representing when the `NaivePeriod` will start.
    /// - `duration`: A `Duration` object representing the the length of time this `NaivePeriod`
    ///   will cover.
    ///
    /// # Returns
    /// - A `NaivePeriod` object having the specified start `NaiveDateTime` and length of the
    ///   specified `Duration`.
    ///
    /// # Example
    /// ```
    /// # use chrono::{Duration, NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};
    /// let start = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(12, 0, 0));
    ///
    /// let np = NaivePeriod::from_start_duration(start, Duration::days(366));
    ///
    /// assert_eq!(Duration::days(366), np.duration());
    /// ```
    #[inline]
    pub fn from_start_duration(start: NaiveDateTime, duration: Duration) -> Self {
        NaivePeriod { start: start, end: start + duration }
    }

    /// Retrieve the [Duration](Duration) this `NaivePeriod` covers.
    #[inline]
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    /// Retrieve the intersection of this `NaivePeriod` with another `NaivePeriod`, if one exists.
    ///
    /// # Arguments
    /// - `other`: A `NaivePeriod` to intersect with this `NaivePeriod`
    ///
    /// # Returns
    /// - An `Option` containing either the intersection of the two `NaivePeriod`s, if they
    ///   overlap; `None`, otherwise.
    ///
    /// # Examples
    /// ```
    /// # use chrono::{Duration, NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};
    /// let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np1 = NaivePeriod::from_start_duration(start1, Duration::days(366));
    ///
    /// let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np2 = NaivePeriod::new(start2, end);
    ///
    /// let intersection = np1.get_intersection_with(np2);
    ///
    /// assert!(intersection.is_some());
    /// assert_eq!(Duration::days(366), intersection.unwrap().duration())
    /// ```
    ///
    /// ```
    /// # use chrono::{Duration, NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};
    ///
    /// let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end1 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np1 = NaivePeriod::new(start1, end1);
    ///
    /// let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 12, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end2 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 14), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np2 = NaivePeriod::new(start2, end2);
    ///
    /// let start3 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 12, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end3 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np3 = NaivePeriod::new(start3, end3);
    ///
    /// let intersection = np1.get_intersection_with(np2);
    ///
    /// assert_eq!(np3, intersection.unwrap());
    /// ```
    pub fn get_intersection_with(&self, other: NaivePeriod) -> Option<NaivePeriod> {
        // If the start and end of other are both before self.start or both after self.end,
        // then there is no intersection.
        let other_start_ts = other.start.timestamp();
        let other_end_ts = other.end.timestamp();

        if (other_start_ts < self.start.timestamp() && other_end_ts < self.start.timestamp())
           || (other_end_ts > self.end.timestamp() && other_start_ts > self.end.timestamp()) {
           return None;
        }

        // The naive time period we want is from the maximum of other_start_ts and
        // self.start.timestamp() and the minimum of other_end_ts and self.end.timestamp().
        let start_ts = max(other_start_ts, self.start.timestamp());
        let end_ts = min(other_end_ts, self.end.timestamp());

        Some(NaivePeriod {
            start: NaiveDateTime::from_timestamp(start_ts, 0),
            end: NaiveDateTime::from_timestamp(end_ts, 0)
        })
    }

    /// Determine if this `NaivePeriod` intersects with another `NaivePeriod`.
    ///
    /// # Arguments
    /// - `other`: A `NaivePeriod` to intersect with this `NaivePeriod`
    ///
    /// # Returns
    /// - `true`, if this `NaivePeriod` and `other` overlap in some finite time period; `false`,
    ///   otherwise
    ///
    /// # Example
    /// ```
    /// # use chrono::{Duration, NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};
    /// let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np1 = NaivePeriod::from_start_duration(start1, Duration::days(366));
    ///
    /// let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
    /// let end = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));
    ///
    /// let np2 = NaivePeriod::new(start2, end);
    ///
    /// assert!(np1.intersects_with(np2))
    /// ```
    #[inline]
    pub fn intersects_with(&self, other: NaivePeriod) -> bool {
        self.get_intersection_with(other).is_some()
    }
}

#[cfg(test)]
mod tests {
    use oldtime::Duration;
    use naive::{NaiveDate, NaiveDateTime, NaivePeriod, NaiveTime};

    #[test]
    fn test_creation_of_naive_period() {
        let start = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np1 = NaivePeriod::new(start, end);

        assert_eq!(np1.duration(), Duration::days(366));
    }

    #[test]
    fn test_intersection_of_year_and_single_day() {
        let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end1 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np1 = NaivePeriod::new(start1, end1);

        let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 2), NaiveTime::from_hms(0, 0, 0));

        let np2 = NaivePeriod::new(start2, end2);

        let intersection = np1.get_intersection_with(np2);

        assert_eq!(intersection.unwrap(), np2);

        // It should also be commutative
        assert_eq!(intersection, np2.get_intersection_with(np1));
    }

    #[test]
    fn test_intersection_that_creates_a_new_period() {
        let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end1 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np1 = NaivePeriod::new(start1, end1);

        let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 12, 1), NaiveTime::from_hms(0, 0, 0));
        let end2 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 14), NaiveTime::from_hms(0, 0, 0));

        let np2 = NaivePeriod::new(start2, end2);

        let start3 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 12, 1), NaiveTime::from_hms(0, 0, 0));
        let end3 = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np3 = NaivePeriod::new(start3, end3);

        let intersection = np1.get_intersection_with(np2);

        assert_eq!(np3, intersection.unwrap());
    }

    #[test]
    fn test_intersection_of_disjoint_periods() {
        let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 4, 12), NaiveTime::from_hms(0, 0, 0));

        let np1 = NaivePeriod::new(start1, end1);

        let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 9, 1), NaiveTime::from_hms(0, 0, 0));
        let end2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 9, 18), NaiveTime::from_hms(0, 0, 0));

        let np2 = NaivePeriod::new(start2, end2);

        let intersection = np2.get_intersection_with(np1);

        assert!(intersection.is_none())
    }

    #[test]
    fn test_creation_of_naive_period_from_duration() {
        let start = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(12, 0, 0));

        let np = NaivePeriod::from_start_duration(start, Duration::days(366));

        assert_eq!(Duration::days(366), np.duration());
    }

    #[test]
    fn test_intersects_with() {
        let start1 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np1 = NaivePeriod::from_start_duration(start1, Duration::days(366));

        let start2 = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(0, 0, 0));
        let end = NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(0, 0, 0));

        let np2 = NaivePeriod::new(start2, end);

        assert!(np1.intersects_with(np2))
    }
}
