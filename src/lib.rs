use std::{fmt::Display, vec};

pub struct BugReport<'a> {
    title: &'a str,
    reporter: &'a str,
    desc: &'a str
}

pub struct BugAnalyzer<T: Categorize + Summarize> {
    reports: Vec<T>
}

#[derive(Debug, PartialEq, Clone)] //the enum provides == and !=
pub enum BugCategory {
    Crash,
    Performance,
    Ui,
    Security,
    Network,
    Unknown
}

#[derive(Debug, PartialEq, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical
}

pub trait Summarize {
    fn summary(&self) -> String;

    fn detail(&self) -> String {
        format!("{} \nfor more detail: \n[Detail]", self.summary())
    }
}

impl<'a> Summarize for BugReport<'a> {
    fn summary(&self) -> String {
        format!("bug title: {}, reporter: {}, description: {}", self.title, self.reporter, self.desc)
    }
}

impl<'a> Display for BugReport<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

pub trait Categorize {
    fn categorize(&self) -> BugCategory;

    fn severity(&self) -> Severity;
}

impl<'a> Categorize for BugReport<'a> {
    fn categorize(&self) -> BugCategory {
        let desc = self.desc.to_string().to_lowercase();
        if desc.contains("crash") {
            BugCategory::Crash
        } else if desc.contains("performance") {
            BugCategory::Performance
        } else if desc.contains("ui") || desc.contains("interface") {
            BugCategory::Ui
        } else if desc.contains("security") {
            BugCategory::Security
        } else if desc.contains("network") {
            BugCategory::Network
        } else {
            BugCategory::Unknown
        }
    }

    fn severity(&self) -> Severity{
        let desc = self.desc.to_string().to_lowercase();
        if desc.contains("fatal") || desc.contains("critical") {
            Severity::Critical
        } else if desc.contains("crash") {
            Severity::High
        } else if desc.contains("mid") || desc.contains("medium") {
            Severity::Medium
        } else {
            Severity::Low
        }
    }
}

impl<T: Summarize + Categorize + Display> BugAnalyzer<T> {
    fn new(&self) -> BugAnalyzer<T> {
        BugAnalyzer {
            reports: Vec::new()
        }
    }

    fn add_reports(&mut self, report: T) -> () {
        self.reports.push(report);
    }

    fn analyze_all(&self) -> () {
        for report in &self.reports {
            println!("{}", report);
            println!("category: {}", report.categorize());
            println!("severity: {}", report.severity());
            println!("____________________________________")
        }
    }

    fn count_by_category(&self) -> () {
        //body
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self {
            Severity::Low => "Low",
            Severity::Medium => "Medium",
            Severity::High => "High",
            Severity::Critical => "Critical",
        };

        write!(f, "{}", severity)
    }
}

impl Display for BugCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let category = match self {
            BugCategory::Crash => "Crash",
            BugCategory::Performance => "Performance",
            BugCategory::Ui => "Ui",
            BugCategory::Security => "Security",
            BugCategory::Network => "Network",
            BugCategory::Unknown => "Unknown",
        };

        write!(f, "{}", category)
    }
}