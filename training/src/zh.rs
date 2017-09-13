use super::*;
use rustling_ontology_values::check::*;

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 0, 0, 1]), "1 秒钟", "一 秒鐘", "一 秒");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1]), "1 分鐘", "一 分鐘");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "1 小時", "一 小時");
    example!(v, check_duration!([0, 0, 0, 5]), "5 天", "五 天", "五 日");
    example!(v, check_duration!([0, 10]), "10 月", "十 月");
    example!(v, check_duration!([0, 1, 0, 15]), "一个半月");
//    example!(v, check_duration!([1, 16]), "一年半");
    example!(v, check_duration!([0, 0, 0, 15]), "半个月");
    example!(v, check_duration!([0, 6]), "半年");
}


pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 10]), "前天", "前日", "上周日", "上星期天", "上礼拜天", "上週日", "上星期天", "上禮拜天", "上禮拜日");
    example!(v, check_moment!(c, [2013, 2, 10]), "周日, 二月十号", "星期天, 二月十号", "礼拜天, 二月十号", "週日, 二月十號", "星期天, 二月十號", "禮拜天, 二月十號", "禮拜日, 二月十號");
    example!(v, check_moment!(c, [2013, 2, 13]),"星期三", "周三", "礼拜三", "禮拜三", "週三", "明天", "明日", "聽日");
    example!(v, check_moment!(c, [2013, 2, 14]),"星期四", "周四", "礼拜四", "禮拜四", "週四", "后天", "後天", "後日");
    example!(v, check_moment!(c, [2013, 2, 15]), "星期五", "周五", "礼拜五", "禮拜五", "週五", "15号", "二月十五", "2-15");
    example!(v, check_moment!(c, [2013, 2, 16]), "星期六", "周六", "礼拜六", "禮拜六", "週六");
    example!(v, check_moment!(c, [2013, 2, 17]), "星期日", "星期天", "礼拜天", "周日", "禮拜天", "週日", "禮拜日");
    example!(v, check_moment!(c, [2013, 2, 18]), "星期一", "周一", "礼拜一", "禮拜一", "週一");
    example!(v, check_moment!(c, [2013, 2, 19]), "星期二", "周二", "礼拜二", "禮拜二", "週二");
    example!(v, check_moment!(c, [2013, 2, 14]), "情人节", "情人節");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "15:15", "15.15");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "1:30", "1.30");
    example!(v, check_moment!(c, [2014, 1, 1]), "元旦", "元旦节", "元旦節");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "现在", "此时", "此刻", "当前", "現在", "此時", "當前", "宜家", "而家", "依家");
    example!(v, check_moment!(c, [2013, 2, 15]), "2/15", "02/15");
    example!(v, check_moment!(c, [2013, 12, 23]), "12/23");
    example!(v, check_moment!(c, [2013, 2, 12, 14, 15]), "1415");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 45]), "545");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "周末", "週末", "这周末", "這週末");
    example!(v, check_moment!(c, [2012]), "去年", "上年");
    example!(v, check_moment!(c, [2013]),"今年");
    example!(v, check_moment!(c, [2014]), "明年", "下年");
    example!(v, check_moment!(c, [2013, 2, 11]), "昨天", "昨日", "尋日");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "上午");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "早上", "早晨");
    example!(v, check_moment!(c, [2013, 12, 25]), "圣诞", "圣诞节",  "圣诞節", "聖誕", "聖誕节", "聖誕節");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 0]), "昨晚", "昨天晚上", "尋晚");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 0]), "今晚", "今天晚上");
    example!(v, check_moment!(c, [2013, 8, 1]), "建军节", "建軍節");
    example!(v, check_moment!(c, [2013, 6, 1]), "儿童节", "儿童節", "兒童节", "兒童節");
    example!(v, check_moment!(c, [2013, 3, 8]), "妇女节", "妇女節", "婦女节", "婦女節");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 0]), "晚上", "晚间");
    example!(v, check_moment!(c, [2017, 8, 22]), "08/22/2017", "8/22/2017");
    example!(v, check_moment!(c, [2013, 2, 12]), "今天", "今日");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 9]), "周一早晨", "周一早上", "礼拜一早上", "週一早上", "禮拜一早上", "礼拜一早晨", "週一早晨", "禮拜一早晨");
    example!(v, check_moment!(c, [2013, 2, 12]), "今天", "今日");
    example!(v, check_moment!(c, [2013, 10, 7]), "十月第一个星期一", "十月的第一个星期一", "十月第一個星期一", "十月的第一個星期一");
    example!(v, check_moment!(c, [2013, 2, 5]), "上周二", "上礼拜二", "上週二", "上禮拜二", "上星期二");
    example!(v, check_moment!(c, [2013, 3, 1]), "三月一号", "三月一日", "三月一號");
    example!(v, check_moment!(c, [2015, 3, 3]), "2015年3月3号", "2015年3月三号", "2015年三月3号", "2015年三月三号", "2015年3月3號", "2015年3月三號", "2015年三月3號", "2015年三月三號", "3/3/2015", "3/3/15", "2015-3-3", "2015-03-03");
    example!(v, check_moment!(c, [2013, 2, 15]), "2013年2月15号", "2013年二月十五号", "2月15号", "二月十五号", "2013年2月15號", "2013年二月十五號", "2月15號", "二月十五號", "2/15");
    example!(v, check_moment!(c, [1974, 10, 31]), "10/31/1974", "10/31/74");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 9]), "二月十五号早上", "2月15号早上", "二月十五號早上", "2月15號早上", "二月十五号早晨", "2月15号早晨", "二月十五號早晨", "2月15號早晨");
    example!(v, check_moment!(c, [2013, 2, 19]), "下周二", "下週二", "下星期二", "下禮拜二", "下礼拜二");
    example!(v, check_moment!(c, [2013, 2, 13]), "这周三", "这礼拜三", "這週三", "這禮拜三", "今個星期三", "今個礼拜三", "今個禮拜三");
    example!(v, check_moment!(c, [2013, 2, 13]), "下周三", "下礼拜三", "下週三", "下禮拜三", "下星期三");
    example!(v, check_moment!(c, [2013, 2, 18]), "这周一", "这礼拜一", "這週一", "這禮拜一", "今個星期一", "今個礼拜一", "今個禮拜一");
    example!(v, check_moment!(c, [2013, 2, 12]), "这周二", "这礼拜二", "這週二", "這禮拜二", "今個星期二", "今個礼拜二", "今個禮拜二");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "这周", "这一周", "这礼拜", "这一礼拜", "這週", "這一周", "這禮拜", "這一禮拜", "今個星期", "今個礼拜", "今個禮拜");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "上周", "上週", "上個星期", "上個礼拜", "上個禮拜");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "下周", "下週", "下星期", "下礼拜", "下禮拜");
    example!(v, check_moment!(c, [2013, 1]), "上月", "上个月", "上個月");
    example!(v, check_moment!(c, [2013, 3]), "下月", "下个月", "下個月");
    example!(v, check_moment!(c, [2012]), "去年", "上年");
    example!(v, check_moment!(c, [2013]), "今年", "这一年", "這一年");
    example!(v, check_moment!(c, [2014]), "明年", "下年");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 0]), "上两秒", "上二秒", "前两秒", "前二秒", "上兩秒", "前兩秒", "兩秒前", "兩秒之前");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 1], [2013, 2, 12, 4, 30, 4]), "下三秒", "后三秒", "後三秒", "三秒後", "三秒之後");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "上两分钟", "上二分钟", "前两分钟", "前二分钟", "上兩分鐘", "上二分鐘", "前兩分鐘", "前二分鐘", "兩分鐘前", "兩分鐘之前");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "下三分钟", "后三分钟", "下三分鐘", "後三分鐘", "三分鐘後", "三分鐘之後");
    example!(v, check_moment_span!(c, [2013, 2, 12, 2], [2013, 2, 12, 4]), "上两小时", "上二小时", "前两小时", "前二小时", "上兩小時", "上二小時", "前兩小時", "前二小時", "兩小時之前", "兩個鐘之前", "兩小時前", "兩個鐘前");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "下三小时", "后三小时", "下三小時", "後三小時", "三小時之後", "三個鐘之後", "三小時後", "三個鐘後");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "上两天", "前两天", "上兩天", "前兩天", "兩天前", "兩天之前", "兩日前", "兩日之前");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "下三天", "后三天", "後三天", "三天後", "三天之後", "三日後", "三日之後");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "上两周", "上二周", "上兩週", "上二週", "兩星期前", "兩星期之前", "兩個禮拜前", "兩個禮拜之前");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "下三周", "下三个周", "下三週", "下三個週", "三星期後", "三星期之後", "三個禮拜後", "三個禮拜之後");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 2]), "上两个月", "上二个月", "上二月", "前两个月", "前二个月", "前两月", "上兩個月", "上二個月", "前兩個月", "前二個月", "前兩月", "兩個月前", "兩個月之前");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "下三个月", "后三个月", "後三個月", "三個月後", "三個月之後");
    example!(v, check_moment_span!(c, [2011], [2013]), "前两年", "前兩年", "兩年前", "兩年之前");
    example!(v, check_moment_span!(c, [2014], [2017]), "下三年", "三年後", "三年之後");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "三点", "三點", "3pm", "下午三點",  "下午三時");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "下午三点十五", "下午3:15", "15:15", "3:15pm", "3:15PM", "3:15p", "下午三點十五");
    example!(v, check_moment!(c, [2015, 4, 14]), "2015年4月14号", "2015年4月14號");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "今晚8点", "今晚八点", "今晚8點", "今晚八點");
    example!(v, check_moment!(c, [2014, 1, 1]), "元旦", "元旦节", "元旦節");
    example!(v, check_moment!(c, [2013, 2, 14]), "情人节", "情人節");
    example!(v, check_moment!(c, [2013, 3, 8]), "妇女节", "婦女節");
    example!(v, check_moment!(c, [2013, 5, 1]), "劳动节", "勞動節");
    example!(v, check_moment!(c, [2013, 6, 1]), "儿童节", "兒童節");
    example!(v, check_moment!(c, [2013, 10, 1]), "国庆", "國慶", "国庆节", "國慶節");
    example!(v, check_moment!(c, [2013, 12, 25]), "圣诞", "聖誕", "圣诞节", "聖誕節");
    example!(v, check_moment_span!(c, [2013, 10, 1, 18], [2013, 10, 2, 0]), "国庆节晚上", "國慶節晚上");
    example!(v, check_moment!(c, [2013, 6, 1, 15, 15]), "儿童节下午三点十五", "兒童節下午三點十五");
    example!(v, check_moment!(c, [2013, 2, 19]), "19日", "19号", "19号星期二", "19日周二", "19日星期二", "十九号星期二", "十九号周二", "十九日星期二", "十九日周二");
    example!(v, check_moment!(c, [2013, 8, 15]), "15号星期四", "15日周四");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "这时", "这个时候", "这个时间");
    example!(v, check_moment!(c, [2013, 2]), "2013/2", "2013/02");
    example!(v, check_moment!(c, [2015, 3, 3]), "2015/3/3", "15/3/3", "2015-03-03");
    example!(v, check_moment!(c, [1974, 10, 31]), "1974/10/31");
    example!(v, check_moment!(c, [2013, 7]), "7月", "7月份", "七月", "七月份");
    example!(v, check_moment!(c, [2013, 7, 18]), "7月18", "七月十八");
    example!(v, check_moment!(c, [2014, 7, 18]), "七月十八周五", "七月十八星期五");
    example!(v, check_moment!(c, [2013, 2, 19]), "十九号礼拜二", "十九日礼拜二");
    example!(v, check_moment!(c, [2014]), "二零一四年");
    example!(v, check_moment!(c, [2014, 10]), "2014年10月", "二零一四年十月", "二零一四年十月");
    example!(v, check_moment!(c, [2013, 3, 1]), "后周周五", "后周五", "后个星期五", "下下个星期五", "下下周周五");
    example!(v, check_moment!(c, [2015, 3]), "后年三月");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 1], [2013, 2, 12, 4, 30, 4]), "之后三秒", "后面三秒", "未来三秒", "下面三秒");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "之后三分钟", "后面三分钟", "下面三分钟", "未来三分钟");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "之后三小时", "后面三小时", "下面三小时", "未来三小时");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "之后三天", "后面三天", "下面三天", "未来三天", "之后几天", "后面几天", "下面几天", "未来几天");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "未来三礼拜", "之后三礼拜", "后面三礼拜", "下面三礼拜");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "之后三个月", "后面三个月", "下面三个月", "未来三个月");
    example!(v, check_moment_span!(c, [2014], [2017]), "之后三年", "后面三年", "下面三年", "未来三年");
    example!(v, check_moment!(c, [2013, 2, 12, 8]), "上午八点");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "下午八点");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "下午三点十五分");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "下午三点二十分");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "下午三点三十分");
    example!(v, check_moment!(c, [2013, 7, 13]), "七月十三");
    example!(v, check_moment!(c, [2013, 2, 28]), "28日", "28号");
    example!(v, check_moment_span!(c, [2013, 2, 14, 4], [2013, 2, 14, 12]), "周四上午");
    example!(v, check_moment_span!(c, [2013, 2, 14, 12], [2013, 2, 14, 19]), "周四下午");
    example!(v, check_moment_span!(c, [2013, 2, 14, 18], [2013, 2, 15, 00]), "周四晚上");
    example!(v, check_moment_span!(c, [2013, 2, 14, 4], [2013, 2, 14, 9]), "周四早上");
    example!(v, check_moment_span!(c, [2013, 2, 13, 1, 00], [2013, 2, 13, 2, 30]), "明天一点到两点半");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11, 00]), "从九点半到十一点");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "周四上午从九点到十一点");
    example!(v, check_moment_with_direction!(c, [2013, 2, 13], Direction::After), "从明天开始");
    example!(v, check_moment!(c, [2013, 2, 10]), "二月十号周天", "二月十日周天");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "前天上午八点");
    example!(v, check_moment_span!(c, [2013, 2, 4], [2013, 2, 11]), "过去一周");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 2, 25] ), "未来一周");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "前天上午八点");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "这个季度");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "下个季度");
    example!(v, check_moment!(c, [2012, 10, 1], Grain::Quarter), "上个季度");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "第三季度");
    //    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "2018年第四季度");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26, 0, 0, 0]), "两周之内", "两周内");
    example!(v, check_moment_span!(c, [2013, 10, 1], [2013, 10, 7]), "十月六号那周", "十月六日那周", "十月六号那一周", "十月六日那一周", "十月六号那一个星期", "十月六日那一个星期");
    example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "十月七号那周", "十月七日那周", "十月七号那一周", "十月七日那一周", "十月七号那一个星期", "十月七日那一个星期");
    example!(v, check_moment!(c, [2013, 3, 25]), "三月的最后一个周一", "三月最后一个周一", "三月最后一个星期一");
    example!(v, check_moment!(c, [2014, 3, 30]), "2014年3月的最后一个周天", "2014年3月的最后一个星期天", "2014年3月的最后一个周日", "2014年3月的最后一个星期日");
    example!(v, check_moment!(c, [2015, 10, 31]), "2015年10月最后一天");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "2014年9月最后一周");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "2014年10月的第一周", "2014年10月的第一个星期");
    example!(v, check_moment!(c, [2014, 9, 16]), "2014年9月的第三个周二", "2014年9月的第三个星期二");
    example!(v, check_moment!(c, [2014, 10, 1]), "2014年10月的第一个星期三", "2014年10月的第一个周三");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "早上三点", "早上3点");
    example!(v, check_moment!(c, [2013, 2, 12]), "2013.2.12");
    example!(v, check_moment!(c, [1974, 10, 31]), "1974/10/31");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "差一刻十二点");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 50]), "差十分八点");
    example!(v, check_moment!(c, [2013, 2, 12, 9, 40]), "差二十十点", "差二十分十点", "差二十分钟十点");


//    example!(v, check_moment!(c, [2013, 2, 13, 3, 18]), "早上3:18", "早上三点十八分");
}


pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(45.0, Some("degree")), "45°", "45度");
    example!(v, check_temperature(45.0, Some("degree")), "45°", "45度");
    example!(v,
             check_temperature(50.0, Some("fahrenheit")),
             "50°F",
             "华氏50°",
             "華氏50°",
             "华氏50度",
             "華氏50度",
             "50华氏°",
             "50華氏°",
             "50华氏度",
             "50華氏度");
    example!(v,
             check_temperature(70.0, Some("fahrenheit")),
             "70°F",
             "华氏70°",
             "華氏70°",
             "华氏70度",
             "華氏70度",
             "70华氏°",
             "70華氏°",
             "70华氏度",
             "70華氏度");
    example!(v,
             check_temperature(23.0, Some("celsius")),
             "23°C",
             "摄氏23°",
             "攝氏23°",
             "摄氏23度",
             "攝氏23度",
             "23摄氏°",
             "23攝氏°",
             "23摄氏度",
             "23攝氏度");
    example!(v,
             check_temperature(37.0, Some("celsius")),
             "37°C",
             "摄氏37°",
             "攝氏37°",
             "摄氏37度",
             "攝氏37度",
             "37摄氏°",
             "37攝氏°",
             "37摄氏度",
             "37攝氏度");
    example!(v, check_temperature(-2.0, Some("degree")), "零下2度");
    example!(v, check_temperature(-2.0, Some("celsius")), "零下2摄氏度");
    example!(v, check_temperature(-20.0, Some("fahrenheit")), "零下20华氏度");
}


pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "〇", "零", "零个", "0个");
    example!(v, check_integer(1), "1", "一", "一个", "1个", "壹");
    example!(v, check_integer(2), "2", "二", "两", "兩", "二個", "二个", "贰");
    example!(v, check_integer(3), "3", "三", "叁");
    example!(v, check_integer(4), "4", "四", "肆");
    example!(v, check_integer(5), "5", "五", "伍");
    example!(v, check_integer(6), "6", "六", "陆");
    example!(v, check_integer(7), "7", "七", "柒");
    example!(v, check_integer(8), "8", "八", "捌");
    example!(v, check_integer(9), "9", "九", "玖");
    example!(v, check_integer(10), "10", "十", "拾");
    example!(v, check_integer(100), "100", "百", "佰");
    example!(v, check_integer(1000), "1000",  "千", "仟");
    example!(v, check_integer(10000), "10000", "万");
    example!(v, check_integer(1000000000), "1000000000", "亿");
    example!(v, check_integer(24), "24", "两打", "2打");
    example!(v, check_integer(100000), "十万");
    example!(v, check_float(1.1), "1.1", "1.10", "01.10");
    example!(v, check_float(0.77), "0.77", ".77");
    example!(v, check_integer(20), "20", "二十");
    example!(v, check_integer(30), "30", "三十");
    example!(v, check_integer(40), "40", "四十");
    example!(v, check_integer(90), "90", "九十");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3,000,000", "三百万", "300万");
    example!(v, check_integer(1200000), "1200K", "1200.0K", "1.2M", ".0012G", "1,200,000", "120万", "一百二十万");
    example!(v, check_float(1200000.0), "1,200,000.00");
    example!(v, check_integer(33), "33", "三十三");
    example!(v, check_integer(96), "96", "九十六");
    example!(v, check_integer(14), "14", "十四");
    example!(v, check_integer(11), "11", "十一");
    example!(v, check_integer(5000), "5k", "五千");
    example!(v,
             check_integer(-1200000),
             "- 1,200,000",
             "-1200000",
             "负1,200,000",
             "负 1,200,000",
             "負 1,200,000",
             "负1200000",
             "负 1200000",
             "-1.2M",
             "-1200K",
             "-.0012G",
             "负一百二十万");
    example!(v, check_ordinal(7), "第七");
    example!(v, check_ordinal(11), "第十一");
    example!(v, check_ordinal(91), "第九十一");
    example!(v, check_integer(122), "一百二十二");
    example!(v, check_integer(200000), "20万");
    example!(v, check_integer(21011), "两万一千零十一");
    example!(v, check_integer(721012), "七十二万一千零十二");
    example!(v, check_integer(2200000), "2.2M", "220万");
    example!(v, check_float(122.2), "一百二十二点二");
    example!(v, check_integer(3), "几");
}
