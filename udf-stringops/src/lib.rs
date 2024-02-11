//! A function to calculate the levenshtein distance between two strings.
//!
//! If a third argument is provided, the value returned by this function will not exceed
//! that limit.
//!
//! # Usage
//!
//! ```sql
//! CREATE FUNCTION levenshtein RETURNS integer SONAME 'libudf_stringops.so';
//! CREATE FUNCTION levenshtein_normalized RETURNS real SONAME 'libudf_stringops.so';
//! ```

use rapidfuzz::distance::levenshtein;
use udf::prelude::*;

const I32_MAX: usize = i32::MAX as usize;

struct Levenshtein;

#[register(name = "levenshtein")]
impl BasicUdf for Levenshtein {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_check_args(args, SqlType::Int)?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        // Unwraps are OK because we set coercions already
        let a_arg = args.get(0).unwrap().value();
        let b_arg = args.get(1).unwrap().value();
        let a = a_arg.as_string().unwrap();
        let b = b_arg.as_string().unwrap();

        if a.len() > I32_MAX || b.len() > I32_MAX {
            return Err(ProcessError);
        }

        let res = match args.get(2) {
            Some(arg) => {
                let limit_i64 = arg.value().as_int().unwrap().clamp(0, i64::MAX);
                let limit = usize::try_from(limit_i64).unwrap();
                let args = levenshtein::Args::default().score_cutoff(limit);
                levenshtein::distance_with_args(a.bytes(), b.bytes(), &args).unwrap_or(limit)
            }
            None => levenshtein::distance(a.bytes(), b.bytes()),
        };

        Ok(res.try_into().unwrap())
    }
}

struct LevenshteinNormalized;

#[register(name = "levenshtein_normalized")]
impl BasicUdf for LevenshteinNormalized {
    type Returns<'a> = f64;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_check_args(args, SqlType::Real)?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        // Unwraps are OK because we set coercions already
        let a_arg = args.get(0).unwrap().value();
        let b_arg = args.get(1).unwrap().value();
        let a = a_arg.as_string().unwrap();
        let b = b_arg.as_string().unwrap();

        if a.len() > I32_MAX || b.len() > I32_MAX {
            return Err(ProcessError);
        }

        let res = match args.get(2) {
            Some(arg) => {
                let limit = arg.value().as_real().unwrap().clamp(0.0, 1.0);
                let args = levenshtein::Args::default().score_cutoff(limit);
                levenshtein::normalized_distance_with_args(a.bytes(), b.bytes(), &args)
                    .unwrap_or(limit)
            }
            None => levenshtein::normalized_distance(a.bytes(), b.bytes()),
        };

        Ok(res)
    }
}

/// Perform arg checks needed for initialization
fn init_check_args(args: &ArgList<Init>, limit_coercion: SqlType) -> Result<(), String> {
    // Lazy error message generation
    let make_emsg = || {
        format!(
            "usage: levenshtein(a: str, b: str [, limit: int]). Got {} args",
            args.len()
        )
    };

    let (Some(mut a_arg), Some(mut b_arg)) = (args.get(0), args.get(1)) else {
        return Err(make_emsg());
    };

    if args.len() > 3 {
        return Err(make_emsg());
    }

    a_arg.set_type_coercion(SqlType::String);
    b_arg.set_type_coercion(SqlType::String);

    if let Some(mut limit_arg) = args.get(2) {
        limit_arg.set_type_coercion(limit_coercion);
    }

    Ok(())
}
