#![allow(unused)]

use std::fmt::Display;
use bugproject::{Summarize, Categorize, Severity, BugCategory, BugAnalyzer, BugReport};

fn main() {
    let mut analyzer: BugAnalyzer<BugReport> = BugAnalyzer::new(); //BugReport has been included 

    let report1 = BugReport {
        title: "bug in open button",
        reporter: "makoo",
        desc: "there is a bug when I click an oppen button in main interface."
    };
    let report2 = BugReport {
        title: "sloww performancee",
        reporter: "abdi",
        desc: "slow performance when I shift from one page into another!"
    };

    //println!("the summarize_1: {} \nthe summarize_2 {}", report1.detail(), report2.detail());
    analyzer.add_reports(report1);
    analyzer.add_reports(report2);

    analyzer.analyze_all();
}
