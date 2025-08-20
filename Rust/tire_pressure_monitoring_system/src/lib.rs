pub mod tire_pressure_monitoring_system {
    use rand::Rng;
    use std::sync::Arc;

    pub struct Alarm {
        low_pressure_threshold: f64,
        high_pressure_threshold: f64,
        sensor: Arc<dyn PressureSensor>,
        alarm_on: bool,
    }

    impl Alarm {
        pub fn new(sensor: Arc<dyn PressureSensor>) -> Self {
            Alarm {
                low_pressure_threshold: 17.0,
                high_pressure_threshold: 21.0,
                sensor,
                alarm_on: false,
            }
        }

        pub fn check(&mut self) {
            let psi_pressure_value = self.sensor.pop_next_pressure_psi_value();

            if psi_pressure_value < self.low_pressure_threshold
                || psi_pressure_value > self.high_pressure_threshold
            {
                self.alarm_on = true;
            }
        }

        pub fn is_alarm_on(&self) -> bool {
            self.alarm_on
        }
    }

    pub struct RandomPressureSensor {
        offset: f64,
    }

    impl RandomPressureSensor {
        pub fn new() -> Self {
            RandomPressureSensor { offset: 16.0 }
        }

        fn sample_pressure() -> f64 {
            let mut rng = rand::thread_rng();
            let pressure_telemetry_value = 6.0 * rng.gen::<f64>() * rng.gen::<f64>();
            pressure_telemetry_value
        }
    }

    impl PressureSensor for RandomPressureSensor {
        fn pop_next_pressure_psi_value(&self) -> f64 {
            let pressure_telemetry_value = Self::sample_pressure();
            self.offset + pressure_telemetry_value
        }
    }

    pub trait PressureSensor {
        fn pop_next_pressure_psi_value(&self) -> f64;
    }

    #[cfg(test)]
    mod tests {
        use super::{Alarm, PressureSensor};
        use std::sync::Arc;

        #[test]
        fn alarm_is_on_when_pressure_is_above_the_threshold() {
            let mut alarm = Alarm::new(Arc::new(TestablePressureSensor(22.0)));
            alarm.check();
            assert_eq!(true, alarm.is_alarm_on());
        }

        #[test]
        fn alarm_is_on_when_pressure_is_below_the_threshold() {
            let mut alarm = Alarm::new(Arc::new(TestablePressureSensor(16.0)));
            alarm.check();
            assert_eq!(true, alarm.is_alarm_on());
        }

        #[test]
        fn alarm_is_off_when_pressure_is_within_the_thresholds() {
            let mut alarm = Alarm::new(Arc::new(TestablePressureSensor(19.0)));
            alarm.check();
            assert_eq!(false, alarm.is_alarm_on());
        }

        struct TestablePressureSensor(f64);
        impl PressureSensor for TestablePressureSensor {
            fn pop_next_pressure_psi_value(&self) -> f64 {
                self.0
            }
        }
    }
}
