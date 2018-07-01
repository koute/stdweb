use std::fmt;
use std::time::Duration;

pub trait Stopwatch {
    fn now() -> Self;
    fn elapsed( &self ) -> Duration;
}

struct PrettyDuration( Duration );

impl fmt::Display for PrettyDuration {
    fn fmt( &self, fmt: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        if self.0.as_secs() > 0 {
            write!( fmt, "{}.{:03}s", self.0.as_secs(), self.0.subsec_millis() )
        } else if self.0.subsec_millis() > 0 {
            write!( fmt, "{}.{:03}ms", self.0.subsec_millis(), self.0.subsec_micros() - self.0.subsec_millis() * 1000 )
        } else if self.0.subsec_micros() > 0 {
            write!( fmt, "{}.{:03}us", self.0.subsec_micros(), self.0.subsec_nanos() - self.0.subsec_micros() * 1000 )
        } else {
            write!( fmt, "{}ns", self.0.subsec_nanos() )
        }
    }
}

struct PrettyNumber( u64 );

impl fmt::Display for PrettyNumber {
    fn fmt( &self, fmt: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        macro_rules! emit {
            ($unit:expr, $divisor:expr) => {
                if self.0 > $divisor {
                    return write!( fmt, "{}.{:03}{}", self.0 / $divisor, (self.0 - (self.0 / $divisor) * $divisor) / ($divisor / 1_000), $unit );
                }
            }
        }

        emit!( "G", 1_000_000_000 );
        emit!( "M", 1_000_000 );
        emit!( "K", 1_000 );
        write!( fmt, "{}", self.0 )
    }
}

#[cfg(not(feature = "nightly"))]
#[inline(always)]
fn blackbox< T >( value: T ) -> T {
    use std::ptr;
    use std::mem;

    unsafe {
        let result = ptr::read_volatile( &value );
        mem::forget( value );
        result
    }
}

#[cfg(feature = "nightly")]
#[inline(always)]
fn blackbox< T >( value: T ) -> T {
    ::test::black_box( value )
}

fn callibrate< S: Stopwatch, R, F: Fn() -> R >( callback: &F ) -> usize {
    callback();

    let mut iterations_per_cycle = 1;
    loop {
        let ts = S::now();
        for _ in 0..iterations_per_cycle {
            blackbox( callback() );
        }

        let elapsed = ts.elapsed();
        if elapsed.as_secs() as u32 * 1000 + elapsed.subsec_millis() >= 100 {
            break;
        }

        iterations_per_cycle *= 10;
    }

    iterations_per_cycle
}

fn measure< S: Stopwatch, R, F: Fn() -> R >( iterations_per_cycle: usize, callback: &F ) -> (u64, Duration) {
    let mut count: u64 = 0;
    let ts = S::now();
    loop {
        for _ in 0..iterations_per_cycle {
            blackbox( callback() );
        }

        count += iterations_per_cycle as u64;
        let elapsed = ts.elapsed();
        if elapsed.as_secs() > 2 {
            return (count, elapsed);
        }
    }
}

pub struct BenchmarkResult {
    count: u64,
    elapsed: Duration
}

impl fmt::Display for BenchmarkResult {
    fn fmt( &self, fmt: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        let count = self.count;
        let elapsed = self.elapsed;

        write!( fmt, "{} per sec, {} per call, {} iterations over {}",
            PrettyNumber( (count as f64 / (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000_f64)) as u64 ),
            PrettyDuration( elapsed / count as u32 ),
            count,
            PrettyDuration( elapsed )
        )
    }
}

pub fn benchmark< S: Stopwatch, R, F: Fn() -> R >( body: &F ) -> BenchmarkResult {
    let iterations_per_cycle = callibrate::< S, R, _ >( body );
    let (count, elapsed) = measure::< S, R, _ >( iterations_per_cycle, body );

    BenchmarkResult {
        count, elapsed
    }
}
