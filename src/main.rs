#![allow(unused)]

use bugproject::{Summarize, Categorize, Severity, BugCategory};

fn main() {
    let bugcategory = BugCategory::Performance;
    let detail_bugcategory = bugcategory.detail();
    println!("{}", detail_bugcategory);

    let severity = Severity::High;
    let detail_severity = severity.detail();
    println!("{}", detail_severity);
}
