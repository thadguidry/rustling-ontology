use output::*;
use dimension::*;
use rustling::Value;
use moment::*;

pub trait ParsingContext<V: Value> {
    type O;
    fn resolve(&self, value: &V) -> Option<Self::O>;
}

pub struct IdentityContext<V: Value+Clone> {
    _phantom: ::std::marker::PhantomData<V>
}

impl<V: Value+Clone> IdentityContext<V> {
    pub fn new() -> IdentityContext<V> {
        IdentityContext {
            _phantom: ::std::marker::PhantomData,
        }
    }
}

impl<V: Value+Clone> ParsingContext<V> for IdentityContext<V> {
    type O = V;
    fn resolve(&self, value: &V) -> Option<V> {
        Some(value.clone())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ResolverContext {
    ctx: Context<Local>,
}

impl ResolverContext { 

    pub fn from_secs(secs: i64) -> ResolverContext {
        let anchor = Interval::starting_at(Moment(Local.timestamp(secs, 0)), Grain::Second);
        ResolverContext::new(anchor)
    }

    pub fn new(now: Interval<Local>) -> ResolverContext {
        ResolverContext {
           ctx: Context::for_reference(now) 
        }
    }
}

impl ParsingContext<Dimension> for ResolverContext {
    type O = Output;
    fn resolve(&self, dim: &Dimension) -> Option<Output> {
        match dim {
            &Dimension::Datetime(ref datetime_value) => {
                let mut walker = datetime_value.constraint
                    .to_walker(&self.ctx.reference, &self.ctx);
                walker.forward
                    .next()
                    .and_then(|h| {
                        if datetime_value.form.not_immediate().unwrap_or(false) && h.intersect(self.ctx.reference).is_some() {
                            walker.forward.next()
                        } else {
                            Some(h)
                        }
                    })
                    .or_else(|| walker.backward.next())
                    .map(|interval| {
                        /* Span bounded on one side */
                        if let Some(bounded_direction) = datetime_value.direction {

                            let anchor = match bounded_direction.bound {
                                Bound::Start => interval.start,
                                Bound::End { only_interval } if only_interval => interval.end.unwrap_or(interval.start),
                                Bound::End { .. } => interval.end_moment(),
                            };

                            let datetime_output_value = DatetimeOutput {
                                moment: anchor,
                                grain: interval.grain,
                                precision: datetime_value.precision,
                                latent: datetime_value.latent,
                            };

                            match bounded_direction.direction {
                                Direction::After => {
                                    let datetime_interval_output_value = DatetimeIntervalOutput::After(datetime_output_value);
                                    match datetime_value.datetime_type {
                                        DatetimeType::DatetimeGeneral |
                                        DatetimeType::DatetimeComplement => Output::DatetimeInterval(datetime_interval_output_value),
                                        DatetimeType::DatePeriod => Output::DatePeriod(datetime_interval_output_value),
                                        DatetimeType::TimePeriod => Output::TimePeriod(datetime_interval_output_value),
                                        DatetimeType::Date => {
                                            eprintln!("[1] Something went wrong - a Date match has left interval boundary");
                                            Output::Date(datetime_output_value)
                                        },
                                        DatetimeType::Time => {
                                            eprintln!("[2] Something went wrong - a Time match has left interval boundary");
                                            Output::Time(datetime_output_value)
                                        },
                                        DatetimeType::Empty => {
                                            eprintln!("[3] Empty Datetime match with left interval boundary - output as DatetimeInterval");
                                            Output::DatetimeInterval(datetime_interval_output_value)
                                        },
                                    }
                                }
                                Direction::Before => {
                                    let datetime_interval_output_value = DatetimeIntervalOutput::Before(datetime_output_value);
                                    match datetime_value.datetime_type {
                                        DatetimeType::DatetimeGeneral |
                                        DatetimeType::DatetimeComplement => Output::DatetimeInterval(datetime_interval_output_value),
                                        DatetimeType::DatePeriod => Output::DatePeriod(datetime_interval_output_value),
                                        DatetimeType::TimePeriod => Output::TimePeriod(datetime_interval_output_value),
                                        DatetimeType::Date => {
                                            eprintln!("[4] Something went wrong - a Date match has right interval boundary");
                                            Output::Date(datetime_output_value)
                                        },
                                        DatetimeType::Time => {
                                            eprintln!("[5] Something went wrong - a Time match has right interval boundary");
                                            Output::Time(datetime_output_value)
                                        },
                                        DatetimeType::Empty => {
                                            eprintln!("[6] Empty Datetime match with right interval boundary - output as DatetimeInterval");
                                            Output::DatetimeInterval(datetime_interval_output_value)
                                        },
                                    }
                                }
                            } /* Span bounded on both sides */
                        } else if let Some(end) = interval.end {

                            let datetime_interval_output_value = DatetimeIntervalOutput::Between {
                                start: interval.start,
                                end: end,
                                precision: datetime_value.precision,
                                latent: datetime_value.latent,
                            };

                            match datetime_value.datetime_type {
                                DatetimeType::DatetimeGeneral |
                                DatetimeType::DatetimeComplement => Output::DatetimeInterval(datetime_interval_output_value),
                                DatetimeType::DatePeriod => Output::DatePeriod(datetime_interval_output_value),
                                DatetimeType::TimePeriod => Output::TimePeriod(datetime_interval_output_value),
                                DatetimeType::Date => {
                                    eprintln!("[7] Something went wrong - a Date match has interval boundaries - output as DatePeriod");
                                    Output::DatePeriod(datetime_interval_output_value)
                                },
                                DatetimeType::Time => {
                                    eprintln!("[8] Something went wrong - a Time match has interval boundaries - output as TimePeriod");
                                    Output::TimePeriod(datetime_interval_output_value)
                                },
                                DatetimeType::Empty => {
                                    eprintln!("[9] Empty Datetime match with interval boundaries - output as DatetimeInterval");
                                    Output::DatetimeInterval(datetime_interval_output_value)
                                },
                            }
                        } else { /* Instant */

                            let datetime_output_value = DatetimeOutput {
                                    moment: interval.start,
                                    grain: interval.grain,
                                    precision: datetime_value.precision,
                                    latent: datetime_value.latent,
                            };

                            match datetime_value.datetime_type {
                                DatetimeType::DatetimeGeneral => Output::Datetime(datetime_output_value),
                                DatetimeType::DatetimeComplement => Output::Datetime(datetime_output_value),
                                DatetimeType::Date => Output::Date(datetime_output_value),
                                DatetimeType::Time => Output::Time(datetime_output_value),
                                DatetimeType::DatePeriod => {
                                    eprintln!("[10] Something went wrong - a DatePeriod match had no interval boundaries - output as Date");
                                    Output::Date(datetime_output_value)
                                },
                                DatetimeType::TimePeriod => {
                                    eprintln!("[11] Something went wrong - a TimePeriod match had no interval boundaries - output as Time");
                                    Output::Time(datetime_output_value)
                                },
                                DatetimeType::Empty => {
                                    eprintln!("[12] Empty Datetime match with no interval boundaries - output as Datetime");
                                    Output::Datetime(datetime_output_value)
                                },
                            }
                        }
                    })
            }
            &Dimension::Number(ref number_value) => {
                match number_value {
                    &NumberValue::Integer(ref v) => Some(Output::Integer(IntegerOutput(v.value))),
                    &NumberValue::Float(ref v) => Some(Output::Float(FloatOutput(v.value))),
                }
            }
            &Dimension::Ordinal(ref ordinal_value) => Some(Output::Ordinal(OrdinalOutput(ordinal_value.value))),
            &Dimension::AmountOfMoney(ref aom) => Some(Output::AmountOfMoney(AmountOfMoneyOutput {
                value: aom.value,
                precision: aom.precision,
                unit: aom.unit,
            })),
            &Dimension::Temperature(ref temp_value) => Some(Output::Temperature(TemperatureOutput {
                value: temp_value.value,
                unit: temp_value.unit,
                latent: temp_value.latent,
            })),
            &Dimension::Duration(ref duration) => Some(Output::Duration(DurationOutput {
                period: duration.period.clone(),
                precision: duration.precision,
            })),
            &Dimension::Percentage(ref percentage) => Some(Output::Percentage(PercentageOutput(percentage.0))),
            _ => None,
        }
    }
}

