use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;
use moment::{Weekday, Grain, PeriodComp};


pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"秒(?:钟|鐘)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );

    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"分(?:钟|鐘)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );

    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"小时|小時|鐘頭?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );

    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"天|日"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );

    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"周|週|礼拜|禮拜|星期"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );

    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"月"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );

    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"年"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );

    b.rule_2("<integer> <unit-of-duration>",
             integer_check!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );

    b.rule_2("half an hour",
             b.reg(r#"半"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Hour),
             |_, _| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );

    b.rule_2("half a month",
             b.reg(r#"半个?"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Month),
             |_, _| Ok(DurationValue::new(PeriodComp::days(15).into()))
    );

    b.rule_2("half a year",
             b.reg(r#"半"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Year),
             |_, _| Ok(DurationValue::new(PeriodComp::months(6).into()))
    );

    b.rule_3("integer and an half <cycle(month, hour)>",
             integer_check!(),
             b.reg(r#"半"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Month || cycle.grain == Grain::Hour),
             |integer, _, cycle| {
                 match cycle.value().grain {
                     Grain::Month => {
                         Ok(DurationValue::new(PeriodComp::months(integer.value().value).into()) + DurationValue::new(PeriodComp::days(15).into()))
                     }
                     Grain::Hour => {
                         Ok(DurationValue::new(PeriodComp::hours(integer.value().value).into()) + DurationValue::new(PeriodComp::minutes(30).into()))
                     }
                     _ => return Err(RuleErrorKind::Invalid.into())
                 }
             }
    );

    b.rule_3("integer and an half <cycle(year, week, minute)>",
             integer_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Year || cycle.grain == Grain::Week || cycle.grain == Grain::Minute || cycle.grain == Grain::Second),
             b.reg(r#"半"#)?,
             |integer, cycle, _| {
                 match cycle.value().grain {
                     Grain::Year => {
                         Ok(DurationValue::new(PeriodComp::years(integer.value().value).into()) + DurationValue::new(PeriodComp::months(6).into()))
                     }
                     Grain::Week => {
                         Ok(DurationValue::new(PeriodComp::weeks(integer.value().value).into()) + DurationValue::new(PeriodComp::days(3).into()))
                     }
                     Grain::Minute => {
                         Ok(DurationValue::new(PeriodComp::minutes(integer.value().value).into()) + DurationValue::new(PeriodComp::seconds(60).into()))
                     }
                     _ => return Err(RuleErrorKind::Invalid.into())
                 }
             }
    );

    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"秒(?:钟|鐘)?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );

    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"分(?:钟|鐘)?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );

    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"小时|小時|鐘頭?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );

    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"天|日"#)?,
                      |_| CycleValue::new(Grain::Day)
    );

    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"周|週|礼拜|禮拜|星期"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"月"#)?,
                      |_| CycleValue::new(Grain::Month)
    );

    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"年"#)?,
                      |_| CycleValue::new(Grain::Year)
    );

    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"季度"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );

    Ok(())
}


pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)一"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)二"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)三"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)四"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)五"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"(?:星期|周|(?:礼|禮)拜|週)六"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );

    b.rule_1_terminal("named-day",
                      b.reg(r#"星期日|星期天|礼拜天|周日|禮拜天|週日|禮拜日|周天"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"一月份?"#)?,
                      |_| helpers::month(1)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"二月份?"#)?,
                      |_| helpers::month(2)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"三月份?"#)?,
                      |_| helpers::month(3)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"四月份?"#)?,
                      |_| helpers::month(4)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"五月份?"#)?,
                      |_| helpers::month(5)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"六月份?"#)?,
                      |_| helpers::month(6)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"七月份?"#)?,
                      |_| helpers::month(7)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"八月份?"#)?,
                      |_| helpers::month(8)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"九月份?"#)?,
                      |_| helpers::month(9)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十月份?"#)?,
                      |_| helpers::month(10)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十一月份?"#)?,
                      |_| helpers::month(11)
    );

    b.rule_1_terminal("named-month",
                      b.reg(r#"十二月份?"#)?,
                      |_| helpers::month(12)
    );

    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"后天|後天|後日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );

    b.rule_1_terminal("valentine's day",
                      b.reg(r#"情人(?:节|節)"#)?,
                      |_| helpers::month_day(2, 14)
    );

    b.rule_1_terminal("hh:.mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:\.]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
    );

    b.rule_1_terminal("new year's day",
                      b.reg(r#"元旦(?:节|節)?"#)?,
                      |_| helpers::month_day(1, 1)
    );

    b.rule_1_terminal("christmas",
                      b.reg(r#"(?:圣诞|聖誕)(?:节|節)?"#)?,
                      |_| helpers::month_day(12, 25)
    );

    b.rule_1_terminal("now",
                      b.reg(r#"现在|此时|此刻|当前|現在|此時|當前|宜家|而家|依家"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );


    b.rule_1_terminal("at this time",
                      b.reg(r#"这个?时(?:候|间)?"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );

    b.rule_1_terminal("mm/dd",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| {
                          helpers::month_day(text_match.group(1).parse()?,
                                             text_match.group(2).parse()?)
                      }
    );

    b.rule_1("hhmm (military time-of-day)",
             b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
             |text_match| Ok(helpers::hour_minute(
                 text_match.group(1).parse()?,
                 text_match.group(2).parse()?,
                 false
             )?.latent())
    );

    b.rule_1_terminal("week-end",
                      b.reg(r#"(周|週)末"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );

    b.rule_1_terminal("last year",
                      b.reg(r#"(?:去|上)年"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Year, -1)
                      }
    );

    b.rule_1_terminal("next year",
                      b.reg(r#"明年|下年"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Year, 1)
                      }
    );

    b.rule_1_terminal("yesterday",
                      b.reg(r#"昨天|昨日|尋日"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Day, -1)
                      }
    );

    b.rule_1_terminal("yyyy/mm",
                      b.reg(r#"(\d{2,4})/(0?[1-9]|1[0-2])"#)?,
                      |text_match| {
                          let month = helpers::year(text_match.group(1).parse()?)?;
                          let year = helpers::month(text_match.group(2).parse()?)?;
                          year.intersect(&month)
                      }
    );

    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})[/\-\.](0?[1-9]|1[0-2])[/\-\.](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"上午"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay { extended_scope: Some(PartOfDayScope::am()) }))
                      }
    );

    b.rule_1_terminal("early morning",
                      b.reg(r#"早上|早晨"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay { extended_scope: Some(PartOfDayScope::am()) }))
                      }
    );

    b.rule_1_terminal("last night",
                      b.reg(r#"昨晚|昨天晚上|尋晚"#)?,
                      |_| {
                          let yesterday = helpers::cycle_nth(Grain::Day, -1)?;
                          let night = helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .form(Form::PartOfDay { extended_scope: None });
                          Ok(yesterday.intersect(&night)?.form(Form::PartOfDay { extended_scope: None }))
                      }
    );

    b.rule_1_terminal("army's day",
                      b.reg(r#"建(?:军节|軍節)"#)?,
                      |_| helpers::month_day(8, 1)
    );

    b.rule_1_terminal("tonight",
                      b.reg(r#"今晚|今天晚上"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay { extended_scope: None }))
                      }
    );

    b.rule_1_terminal("tomorrow night",
                      b.reg(r#"明晚|明天晚上|聽晚"#)?,
                      |_| {
                          let tomorrow = helpers::cycle_nth(Grain::Day, 1)?;
                          let night = helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .form(Form::PartOfDay { extended_scope: None });
                          Ok(tomorrow.intersect(&night)?.form(Form::PartOfDay { extended_scope: None }))
                      }
    );

    b.rule_1_terminal("army's day",
                      b.reg(r#"(?:儿|兒)童(?:节|節)"#)?,
                      |_| helpers::month_day(6, 1)
    );


    b.rule_1_terminal("this year",
                      b.reg(r#"今年"#)?,
                      |_| helpers::cycle_nth(Grain::Year, 0)
    );

    b.rule_1_terminal("women's day",
                      b.reg(r#"(?:妇|婦)女(?:节|節)"#)?,
                      |_| helpers::month_day(3, 8)
    );

    b.rule_1_terminal("evening|night",
                      b.reg(r#"晚上|晚间"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay { extended_scope: None }))
                      }
    );

    b.rule_1_terminal("mm/dd/yyyy",
                      b.reg(r#"(0?[1-9]|1[0-2])/(3[01]|[12]\d|0?[1-9])/(\d{2,4})"#)?,
                      |text_match| helpers::ymd(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );

    b.rule_1_terminal("tomorrow",
                      b.reg(r#"明天|明日|聽日"#)?,
                      |_| {
                          helpers::cycle_nth(Grain::Day, 1)
                      }
    );

    b.rule_1("time-of-day (latent)",
             integer_check!(0, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );

    b.rule_2("<time-of-day> o'clock",
             time_check!(form ! (Form::TimeOfDay(_))),
             b.reg(r#"點|点|時"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );

    b.rule_1("number (as relative minutes)",
             integer_check!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );

    b.rule_2("number minutes (as relative minutes)",
             integer_check!(1, 59),
             b.reg(r#"分钟?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );

    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             time_check!(form ! (Form::TimeOfDay(_))),
             b.reg(r#"(?:点|點)?差"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 -1 * relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock)
    );

    b.rule_3("relative minutes after|past  <integer> (hour-of-day)",
             time_check!(form ! (Form::TimeOfDay(_))),
             b.reg(r#"点|點|过|過"#)?,
             relative_minute_check!(),
             |time, _, relative_minute| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 relative_minute.value().0,
                 time.value().form_time_of_day()?.is_12_clock)
    );

    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"一刻"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );

    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"半"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );


    b.rule_2("this <day-of-week>",
             b.reg(r#"这|這|今(?:个|個)"#)?,
             time_check!(form ! (Form::DayOfWeek{..})),
             |_, a| a.value().the_nth_not_immediate(0)
    );

    b.rule_4("nth <time> of <time>",
             time_check!(),
             b.reg(r#"的"#)?,
             ordinal_check!(),
             time_check!(),
             |a, _, ordinal, b| {
                 a.value().intersect(b.value())?.the_nth(ordinal.value().value - 1)
             }
    );

    b.rule_2("last <time>",
             b.reg(r#"去|上(?:个|個)?"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );

    b.rule_2("in <duration>",
             b.reg(r#"再"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );


    b.rule_1_terminal("national day",
                      b.reg(r#"(?:国庆|國慶)(?:节|節)?"#)?,
                      |_| helpers::month_day(10, 1)
    );

    b.rule_4("the <cycle> after <time>",
             b.reg(r#"那"#)?,
             cycle_check!(),
             b.reg(r#"之?(?:后|後)"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );

    b.rule_4("<cycle> before <time>",
             b.reg(r#"那"#)?,
             cycle_check!(),
             b.reg(r#"之?前"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );

    b.rule_1_terminal("noon",
                      b.reg(r#"中午"#)?,
                      |_| helpers::hour(12, false)
    );

    b.rule_1_terminal("today",
                      b.reg(r#"今天|今日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );

    b.rule_2("this|next <day-of-week>",
             b.reg(r#"今(?:个|個)?|明|下(?:个|個)?"#)?,
             time_check!(form ! (Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth_not_immediate(0)
             }
    );

    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"前天|前日"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );

    b.rule_1_terminal("labor day",
                      b.reg(r#"劳动节|勞動節"#)?,
                      |_| helpers::month_day(5, 1)
    );

    b.rule_2("next <cycle>",
             b.reg(r#"下(?:个|個)?"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );

    b.rule_2("<duration> from now",
             duration_check!(),
             b.reg(r#"后|後|之後"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_2("last <cycle>",
             b.reg(r#"上(?:个|個)?"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );

    b.rule_1_terminal("afternoon",
                      b.reg(r#"下午"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay { extended_scope: Some(PartOfDayScope::pm()) }))
                      }
    );

    b.rule_1_terminal("midnight",
                      b.reg(r#"午夜|凌晨|半夜"#)?,
                      |_| helpers::hour(0, false)
    );

    b.rule_2("in|during the <part-of-day>",
             time_check!(form ! (Form::PartOfDay {..})),
             b.reg(r#"点|點"#)?,
             |time, _| Ok(time.value().clone().not_latent())
    );

    b.rule_3("intersect by \",\"",
             time_check!( | time: & TimeValue | ! time.latent),
             b.reg(r#","#)?,
             time_check!( | time: & TimeValue | ! time.latent),
             |a, _, b| a.value().intersect(b.value())
    );

    b.rule_2("year (numeric with year symbol)",
             integer_check!(1000, 9999),
             b.reg(r#"年"#)?,
             |integer, _| helpers::year(integer.value().value as i32)
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"之?前"#)?,
             |a, _| a.value().ago()
    );

    b.rule_3("last n <cycle>",
             b.reg(r#"上|前"#)?,
             integer_check!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_3("n <cycle> last",
             integer_check!(1, 9999),
             cycle_check!(),
             b.reg(r#"之?前"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_2("intersect",
             time_check!( | time: & TimeValue | ! time.latent),
             time_check!( | time: & TimeValue| ! time.latent),
             |a, b| a.value().intersect(b.value())
    );

    b.rule_3("nth <time> of <time>",
             time_check!(),
             ordinal_check!(),
             time_check!(),
             |a, ordinal, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );

    b.rule_2("<time> <part-of-day>",
             time_check!(),
             time_check!(form!(Form::PartOfDay {..})),
             |time, part_of_day| time.value().intersect(&part_of_day.value())
    );

    b.rule_2("next <time>",
             b.reg(r#"明|下(?:个|個)?"#)?,
             time_check!( | time: & TimeValue | ! time.latent),
             |_, a| {
                 a.value().the_nth(0)
             }
    );

    b.rule_3("next n <cycle>",
             b.reg(r#"下|后|後"#)?,
             integer_check!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_3("next n <cycle>",
             integer_check!(1, 9999),
             cycle_check!(),
             b.reg(r#"下|之?后|之?後"#)?,
             |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_2("this <cycle>",
             b.reg(r#"这(?:一|个)?|這一?|今個"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );

    b.rule_2("this <time>",
             b.reg(r#"今(?:个|個)?|这个?|這個?"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );

    b.rule_2("<time-of-day> am|pm",
             time_check!(form ! (Form::TimeOfDay(_))),
             b.reg(r#"([ap])(?:\s|\.)?m?\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(Form::TimeOfDay(None)))
             }
    );

    b.rule_2("integer day",
             integer_check!(1, 31),
             b.reg(r#"号|號|日"#)?,
             |integer, _| helpers::day_of_month(integer.value().value as u32)
    );

    b.rule_2("day integer",
             b.reg(r#"号|號|日"#)?,
             integer_check!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );

    b.rule_2("last <time>",
             b.reg(r#"上"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );

    b.rule_2("<part-of-day> <time>",
             time_check!(form ! (Form::PartOfDay {..})),
             time_check!(),
             |part_of_day, time| {
                 let span_value = if let Form::PartOfDay { extended_scope: Some(scope) } = part_of_day.value().clone().form {
                     scope.build_extensible_span()?
                 } else {
                     part_of_day.value().clone()
                 };
                 time.value().intersect(&span_value)
             }
    );

    b.rule_2("month (numeric with month symbol)",
             integer_check!(1, 12),
             b.reg(r#"月"#)?,
             |integer, _| Ok(helpers::month(integer.value().value as u32)?.latent())
    );

    b.rule_2("absorption of , after named day",
             time_check!(form ! (Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );

    b.rule_2("<integer> month",
             integer_check!(1, 12),
             b.reg(r#"月份?"#)?,
             |a, _| helpers::month(a.value().value as u32)
    );

    b.rule_2("<integer> year",
             integer_check!(),
             b.reg(r#"年"#)?,
             |integer, _| helpers::year(integer.value().value as i32)
    );

    b.rule_2("<named-month> <day-of-month>",
             time_check!(form ! (Form::Month(_))),
             integer_check!(1, 31),
             |a, integer| {
                 a.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
             }
    );

    b.rule_2("<day-of-month> <named-day>",
             time_check!(form ! (Form::DayOfMonth)),
             time_check!(form ! (Form::DayOfWeek{..})),
             |a, b| {
                 a.value().intersect(&b.value())
             }
    );

    b.rule_2("after next week <day-of-week>",
             b.reg(r#"(?:后|下下)个?"#)?,
             time_check!(form ! (Form::DayOfWeek{..})),
             |_, a| helpers::cycle_nth(Grain::Week, 2)?.intersect(&a.value())
    );

    b.rule_2("after next week <day-of-week>",
             b.reg_neg_lh(r#"(?:后|下下)个?周"#, r#"五"#)?,
             time_check!(form ! (Form::DayOfWeek{..})),
             |_, a| helpers::cycle_nth(Grain::Week, 2)?.intersect(&a.value())
    );

    b.rule_2("after next year named-month",
             b.reg(r#"后年"#)?,
             time_check!(form ! (Form::Month(_))),
             |_, a| {
                 helpers::cycle_nth(Grain::Year, 2)?.intersect(&a.value())
             }
    );

    b.rule_3("next n <cycle>",
             b.reg(r#"未来|之后|(?:下|后)面"#)?,
             integer_check!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_3("<datetime> - <datetime> (interval)",
             time_check!( | time: & TimeValue | ! time.latent & & excluding_form ! (Form::TimeOfDay(_))(time)),
             b.reg(r#"\s?(?:-|~)\s?|到"#)?,
             time_check!( | time: & TimeValue| ! time.latent & & excluding_form ! (Form::TimeOfDay(_))(time)),
             |a, _, b| a.value().span_to(b.value(), true)
    );

    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             time_check!( | time: & TimeValue | ! time.latent & & form ! (Form::TimeOfDay(_))(time)),
             b.reg(r#"\s?(?:-|~)\s?|到"#)?,
             time_check!(form ! (Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );

    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"从"#)?,
             time_check!( | time: & TimeValue | ! time.latent & & excluding_form ! (Form::TimeOfDay(_))(time)),
             b.reg(r#"\s?(?:-|~)\s?|到"#)?,
             time_check!( | time: & TimeValue | ! time.latent & & excluding_form ! (Form::TimeOfDay(_))(time)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );

    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"从"#)?,
             time_check!( | time: & TimeValue | ! time.latent & & form ! (Form::TimeOfDay(_))(time)),
             b.reg(r#"\s?(?:-|~)\s?|到"#)?,
             time_check!(form ! (Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );

    b.rule_3("from <time>",
             b.reg(r#"从"#)?,
             time_check!(),
             b.reg(r#"开始"#)?,
             |_, time, _| Ok(time.value().clone().direction(Some(Direction::After)))
    );

    b.rule_5("4 integers year",
             integer_check!(1, 9),
             integer_check!(0, 9),
             integer_check!(0, 9),
             integer_check!(0, 9),
             cycle_check!( | cycle: & CycleValue | cycle.grain == Grain::Year),
             |a, b, c, d, _| {
                 let year = 1000 * a.value().value + 100 * b.value().value + 10 * c.value().value + d.value().value;
                 helpers::year(year as i32)
             }
    );

    b.rule_4("3 integers year",
             integer_check!(1, 9),
             integer_check!(0, 9),
             integer_check!(0, 9),
             cycle_check!( | cycle: & CycleValue| cycle.grain == Grain::Year),
             |a, b, c, _| {
                 let year = 100 * a.value().value + 10 * b.value().value + c.value().value;
                 helpers::year(year as i32)
             }
    );

    b.rule_3("2 integers year",
             integer_check!(1, 9),
             integer_check!(0, 9),
             cycle_check!( | cycle: & CycleValue | cycle.grain == Grain::Year),
             |a, b, _| {
                 let year = 10 * a.value().value + b.value().value;
                 helpers::year(year as i32)
             }
    );

    b.rule_3("coming n <cycle>",
             b.reg(r#"未来"#)?,
             integer_check!(),
             cycle_check!(),
             |_, integer, cycle| {
                 helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
             }
    );

    b.rule_3("past n <cycle>",
             b.reg(r#"过去"#)?,
             integer_check!(),
             cycle_check!(),
             |_, integer, cycle| {
                 helpers::cycle_n_not_immediate(cycle.value().grain, -integer.value().value)
             }
    );

    b.rule_2(
        "<ordinal> quarter",
        ordinal_check!(),
        cycle_check!( | cycle: & CycleValue | cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );

    b.rule_2("within <duration>",
             duration_check!(),
             b.reg(r#"之?内"#)?,
             |a, _| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );

    b.rule_3("the week of <time>",
             time_check!(form ! (Form::DayOfMonth)),
             b.reg(r#"那一?个?"#)?,
             cycle_check!( | cycle: & CycleValue | cycle.grain == Grain::Week),
             |a, _, _| {
                 helpers::cycle_nth_after(Grain::Week, 0, &a.value())
             }
    );

    b.rule_3("last <day-of-week> of <time>",
             time_check!(),
             b.reg(r#"的?最后一个"#)?,
             time_check!(form ! (Form::DayOfWeek{..})),
             |a, _, b| {
                 b.value().last_of(a.value())
             }
    );

    b.rule_3("last <cycle> of <time>",
             time_check!(),
             b.reg(r#"的?最后一个?"#)?,
             cycle_check!(),
             |a, _, b| {
                 b.value().last_of(a.value())
             }
    );

    b.rule_4("<ordinal> <cycle> of <time>",
             time_check!(),
             b.reg(r#"的?"#)?,
             ordinal_check!(),
             cycle_check!(),
             |time, _, ordinal, cycle| {
                 helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
             }
    );

    b.rule_4("<ordinal> <day-of-week> of <time>",
             time_check!(),
             b.reg(r#"的?"#)?,
             ordinal_check!(),
             time_check!(form!(Form::DayOfWeek{..})),
             |time, _, ordinal, day| {
                 let week = helpers::cycle_nth_after_not_immediate(Grain::Week, ordinal.value().value - 1, time.value())?;
                 week.intersect(day.value())
             }
    );

    b.rule_1_terminal("season",
                      b.reg(r#"夏(?:天|季)?"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"秋(?:天|季)?"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"冬(?:天|季)?"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"春(?:天|季)?"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );

    b.rule_4("<relative-minutes> to <time-of-day>",
             b.reg(r#"差"#)?,
             relative_minute_check!(),
             b.reg(r#"分?钟?"#)?,
             time_check!(),
             |_, relative_minute, _, time| helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour,
                 -1 * relative_minute.value().0,
                 true)
    );

    Ok(())
}


pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| {
                 Ok(TemperatureValue {
                     value: a.value().value(),
                     unit: None,
                     latent: true,
                 })
             });

    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });

    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:摄|攝)氏(?:°|度)|°c"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });

    b.rule_3("Celcius <temp>",
             b.reg(r#"(?:摄|攝)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             }
    );

    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"(?:华|華)氏(?:°|度)|°f"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });

    b.rule_3("Fahrenheit <temp>",
             b.reg(r#"(?:华|華)氏"#)?,
             temperature_check!(),
             b.reg(r#"度|°"#)?,
             |_, b, _| {
                 Ok(TemperatureValue {
                     value: b.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             }
    );

    b.rule_2("below <temp>",
             b.reg(r#"零下"#)?,
             temperature_check!(),
             |_, b| {
                 Ok(TemperatureValue {
                     value: -1.0 * b.value().value,
                     ..b.value().clone()
                 })
             });

    Ok(())
}


pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
             integer_filter!( |integer: & IntegerValue | integer.grain.unwrap_or(0) > 1),
             integer_check!(),
             |a, b| {
                 IntegerValue::new_with_grain(a.value().value + b.value().value, a.value().grain.unwrap_or(0))
             });

    b.rule_1_terminal("integer (0..10)",
                      b.reg(r#"(〇|零|一|二|两|兩|三|四|五|六|七|八|九|十|壹|贰|叁|肆|伍|陆|柒|捌|玖|拾)(?:个|個)?"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "〇" => 0,
                              "零" => 0,
                              "一" => 1,
                              "壹" => 1,
                              "二" => 2,
                              "两" => 2,
                              "兩" => 2,
                              "贰" => 2,
                              "三" => 3,
                              "叁" => 3,
                              "四" => 4,
                              "肆" => 4,
                              "五" => 5,
                              "伍" => 5,
                              "六" => 6,
                              "陆" => 6,
                              "七" => 7,
                              "柒" => 7,
                              "八" => 8,
                              "捌" => 8,
                              "九" => 9,
                              "玖" => 9,
                              "十" => 10,
                              "拾" => 10,
                              _ => return Err(RuleErrorKind::Invalid.into())
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });

    b.rule_1_terminal(
        "integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| IntegerValue::new(text_match.group(0).parse()?));

    b.rule_1("decimal number", b.reg(r#"(\d*\.\d+)"#)?, |text_match| {
        let value: f32 = text_match.group(0).parse()?;
        Ok(FloatValue {
            value: value,
            ..FloatValue::default()
        })
    });

    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|负\s?|負\s?"#)?,
             number_check!( |number: & NumberValue | ! number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });

    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f32 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });

    b.rule_2("<number>个",
             number_check!(),
             b.reg(r#"个"#)?,
             |number, _| Ok(number.value().clone()));

    b.rule_2("integer (20..90)",
             integer_check!(2, 9),
             b.reg(r#"十"#)?,
             |a, _| {
                 Ok(IntegerValue {
                     value: a.value().value * 10,
                     ..a.value().clone()
                 })
             });

    b.rule_2("numbers suffixes (K, M, G)",
             number_check!( | number: & NumberValue | ! number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€元¥(?:人民币)]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleErrorKind::Invalid.into()),
                 };
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f32);
                         if product.floor() == product {
                             IntegerValue {
                                 value: product as i64,
                                 suffixed: true,
                                 ..IntegerValue::default()
                             }
                                 .into()
                         } else {
                             FloatValue {
                                 value: product,
                                 suffixed: true,
                                 ..float
                             }
                                 .into()
                         }
                     }
                 })
             });

    b.rule_2("integer 21..99",
             integer_check!(10, 90, | integer: & IntegerValue| integer.value % 10 == 0),
             integer_check!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));

    b.rule_2("integer (11..19)",
             b.reg(r#"十"#)?,
             integer_check!(1, 9),
             |_, b| IntegerValue::new(10 + b.value().value));

    b.rule_1("integer with thousands separator, ",
             b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(",", "");
                 let value: i64 = reformatted_string.parse()?;
                 Ok(IntegerValue {
                     value: value,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("ordinal (digits)",
             b.reg(r#"第"#)?,
             integer_check!(),
             |_, b| {
                 Ok(OrdinalValue {
                     value: b.value().value,
                     prefixed: true
                 })
             }
    );

    b.rule_1_terminal("hundred",
                      b.reg(r#"百|佰"#)?,
                      |_| IntegerValue::new_with_grain(100, 2)
    );

    b.rule_1_terminal("thousand",
                      b.reg(r#"千|仟"#)?,
                      |_| IntegerValue::new_with_grain(1000, 3)
    );


    b.rule_1_terminal("ten-thousand",
                      b.reg(r#"万"#)?,
                      |_| IntegerValue::new_with_grain(10000, 4)
    );


    b.rule_1_terminal("billion",
                      b.reg(r#"亿"#)?,
                      |_| IntegerValue::new_with_grain(1000000000, 9)
    );

    b.rule_1_terminal("dozen",
                      b.reg(r#"打"#)?,
                      |_| Ok(IntegerValue {
                          value: 12,
                          grain: Some(1),
                          group: true,
                          ..IntegerValue::default()
                      })
    );

    b.rule_2("number dozen",
             integer_check!(1, 10),
             integer_filter!(| integer: & IntegerValue | integer.group),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("number hundreds",
             integer_check!(1, 9),
             integer_check!(100, 100),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("number thousands",
             integer_check!(1, 9),
             integer_check!(1000, 1000),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("number ten-thousands",
             integer_check!(1, 9999),
             integer_check!(10000, 10000),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });

    b.rule_2("number billion",
             integer_check!(1, 999),
             integer_check!(1000000000, 1000000000),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     ..IntegerValue::default()
                 })
             });


    b.rule_3("number dot number",
             number_check!( | number: & NumberValue | ! number.prefixed()),
             b.reg(r#"点"#)?,
             number_check!( | number: &NumberValue | ! number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                     value: b.value().value() * 0.1 + a.value().value(),
                     ..FloatValue::default()
                 })
             });

    b.rule_1("few",
             b.reg(r#"几"#)?, |_| {
            Ok(IntegerValue {
                value: 3,
                grain: Some(1),
                precision: Approximate,
                ..IntegerValue::default()
            })
        });

    Ok(())
}
