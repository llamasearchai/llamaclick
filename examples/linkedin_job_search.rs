// LinkedIn Job Search Automation Example
//
// This example demonstrates how to use LlamaClick to automate LinkedIn job searches.
// It shows how to:
// 1. Log in to LinkedIn
// 2. Search for jobs with specific criteria
// 3. Extract job details
// 4. Save results to a CSV file
// 5. Optionally apply to jobs that match certain criteria

use llamaclick::browser::{BrowserController, ChromiumBrowser};
use llamaclick::config::Config;
use llamaclick::error::Result;
use llamaclick::linkedin::{
    LinkedInClient,
    JobSearchCriteria,
    JobDetails,
    LinkedInConfig,
    FilterType,
    ExperienceLevel,
};
use llamaclick::utils::file_system::write_to_file;

use std::env;
use std::path::Path;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== LlamaClick LinkedIn Job Search Example ===\n");
    
    // For this example, we'll simulate the process to avoid requiring real credentials
    // In a real application, you would use your actual credentials
    let simulation_mode = true;
    
    // 1. Configure the LinkedIn automation
    let config = Config::default()
        .with_browser_options(Default::default())
        .with_linkedin_config(LinkedInConfig {
            email: env::var("LINKEDIN_EMAIL").unwrap_or_else(|_| "example@email.com".to_string()),
            password: env::var("LINKEDIN_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            auto_apply: false, // Set to true to enable automatic job applications
            session_timeout: Duration::from_secs(3600),
            resume_path: Some(String::from("./resume.pdf")),
            cover_letter_path: None,
            job_preferences: JobSearchCriteria {
                keywords: vec!["Rust developer".to_string(), "Systems programming".to_string()],
                location: Some("Remote".to_string()),
                distance: None,
                experience_level: Some(vec![
                    ExperienceLevel::MidSeniorLevel,
                    ExperienceLevel::Associate,
                ]),
                date_posted: Some(FilterType::Past24Hours),
                remote: Some(true),
                .. Default::default()
            },
            .. Default::default()
        });
    
    println!("Initializing LinkedIn automation...");
    
    // 2. Create LinkedIn client
    let linkedin = if simulation_mode {
        println!("Using simulation mode...");
        create_simulated_linkedin_client()
    } else {
        println!("Creating real LinkedIn client...");
        // In a real application, this would initiate a real browser session
        LinkedInClient::new(&config)?
    };
    
    // 3. Log in to LinkedIn
    println!("\nLogging in to LinkedIn...");
    if simulation_mode {
        println!("Simulating login process...");
        println!("Login successful!");
    } else {
        linkedin.login()?;
    }
    
    // 4. Search for jobs
    println!("\nSearching for jobs with criteria:");
    println!("  Keywords: {:?}", config.linkedin.job_preferences.keywords);
    println!("  Location: {:?}", config.linkedin.job_preferences.location);
    println!("  Experience: {:?}", config.linkedin.job_preferences.experience_level);
    println!("  Remote: {:?}", config.linkedin.job_preferences.remote);
    println!("  Posted: {:?}", config.linkedin.job_preferences.date_posted);
    
    let jobs = linkedin.search_jobs(&config.linkedin.job_preferences)?;
    
    println!("\nFound {} matching jobs", jobs.len());
    
    // 5. Display job results
    println!("\n=== Job Search Results ===");
    for (i, job) in jobs.iter().enumerate() {
        println!("\nJob #{}", i + 1);
        println!("Title: {}", job.title);
        println!("Company: {}", job.company);
        println!("Location: {}", job.location);
        println!("Posted: {}", job.posted_date);
        println!("Job Type: {}", job.job_type);
        println!("Salary: {}", job.salary.clone().unwrap_or_else(|| "Not specified".to_string()));
        println!("Application Link: {}", job.application_url);
        println!("---------------------------");
    }
    
    // 6. Save results to CSV
    println!("\nSaving job results to CSV...");
    let csv_content = jobs_to_csv(&jobs);
    write_to_file("linkedin_jobs.csv", &csv_content)?;
    println!("Results saved to linkedin_jobs.csv");
    
    // 7. Filter jobs that match specific criteria for potential application
    if config.linkedin.auto_apply {
        println!("\nFiltering jobs for automatic application...");
        let jobs_to_apply = jobs.iter()
            .filter(|job| {
                // Apply only to jobs that match specific criteria
                // This is just an example - you would customize these filters
                let title_match = job.title.to_lowercase().contains("rust") || 
                                 job.title.to_lowercase().contains("systems");
                let remote_match = job.location.to_lowercase().contains("remote");
                let recency_match = job.posted_date.contains("hour") || 
                                   job.posted_date.contains("day") && 
                                   !job.posted_date.contains("30+ days");
                
                title_match && remote_match && recency_match
            })
            .collect::<Vec<_>>();
        
        println!("Found {} jobs matching application criteria", jobs_to_apply.len());
        
        // 8. Apply to filtered jobs
        if !jobs_to_apply.is_empty() {
            println!("\nPreparing to apply to matching jobs...");
            for job in jobs_to_apply {
                println!("Applying to: {} at {}", job.title, job.company);
                
                if simulation_mode {
                    println!("  [Simulation] Application submitted successfully!");
                } else {
                    // In a real application, this would submit the application
                    match linkedin.apply_to_job(&job.id) {
                        Ok(_) => println!("  Application submitted successfully!"),
                        Err(e) => println!("  Failed to apply: {}", e),
                    }
                }
            }
        }
    }
    
    // 9. Log out and clean up
    println!("\nLogging out of LinkedIn...");
    if !simulation_mode {
        linkedin.logout()?;
    }
    
    println!("\n=== LinkedIn job search example completed successfully ===");
    Ok(())
}

// Helper function to convert job details to CSV format
fn jobs_to_csv(jobs: &[JobDetails]) -> String {
    let mut csv = String::from("Title,Company,Location,Posted Date,Job Type,Salary,Application URL\n");
    
    for job in jobs {
        // Escape commas and quotes in fields
        let title = escape_csv_field(&job.title);
        let company = escape_csv_field(&job.company);
        let location = escape_csv_field(&job.location);
        let posted_date = escape_csv_field(&job.posted_date);
        let job_type = escape_csv_field(&job.job_type);
        let salary = escape_csv_field(&job.salary.clone().unwrap_or_default());
        let application_url = escape_csv_field(&job.application_url);
        
        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            title, company, location, posted_date, job_type, salary, application_url
        ));
    }
    
    csv
}

// Helper function to escape CSV fields
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        // Escape quotes by doubling them and wrap in quotes
        let escaped = field.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

// Helper function to create a simulated LinkedIn client for the example
fn create_simulated_linkedin_client() -> LinkedInClient {
    // This function would create a real LinkedIn client in a real application
    // For the example, we're creating a simulated version that returns mock data
    
    // Create some simulated job listings
    let simulated_jobs = vec![
        JobDetails {
            id: "job123".to_string(),
            title: "Senior Rust Developer".to_string(),
            company: "TechCorp Inc.".to_string(),
            location: "Remote".to_string(),
            posted_date: "2 hours ago".to_string(),
            job_type: "Full-time".to_string(),
            salary: Some("$120,000 - $150,000 a year".to_string()),
            description: "We're looking for an experienced Rust developer to join our systems team...".to_string(),
            application_url: "https://www.linkedin.com/jobs/view/job123".to_string(),
            easy_apply: true,
        },
        JobDetails {
            id: "job456".to_string(),
            title: "Systems Engineer (Rust/C++)".to_string(),
            company: "StartupXYZ".to_string(),
            location: "San Francisco, CA (Remote available)".to_string(),
            posted_date: "1 day ago".to_string(),
            job_type: "Full-time".to_string(),
            salary: Some("$130,000 - $160,000 a year".to_string()),
            description: "Join our team building high-performance systems in Rust and C++...".to_string(),
            application_url: "https://www.linkedin.com/jobs/view/job456".to_string(),
            easy_apply: true,
        },
        JobDetails {
            id: "job789".to_string(),
            title: "Backend Developer - Rust".to_string(),
            company: "FinTech Solutions".to_string(),
            location: "Remote".to_string(),
            posted_date: "3 days ago".to_string(),
            job_type: "Full-time".to_string(),
            salary: None,
            description: "We are seeking a talented backend developer with Rust experience...".to_string(),
            application_url: "https://www.linkedin.com/jobs/view/job789".to_string(),
            easy_apply: false,
        },
        JobDetails {
            id: "job101".to_string(),
            title: "Distributed Systems Engineer".to_string(),
            company: "CloudScale".to_string(),
            location: "New York, NY (Remote)".to_string(),
            posted_date: "1 week ago".to_string(),
            job_type: "Full-time".to_string(),
            salary: Some("Competitive".to_string()),
            description: "Help us build resilient distributed systems using Rust and Kubernetes...".to_string(),
            application_url: "https://www.linkedin.com/jobs/view/job101".to_string(),
            easy_apply: true,
        },
        JobDetails {
            id: "job202".to_string(),
            title: "Senior Systems Programmer - Rust".to_string(),
            company: "Security Innovations".to_string(),
            location: "Remote (US)".to_string(),
            posted_date: "Just now".to_string(),
            job_type: "Full-time".to_string(),
            salary: Some("$140,000 - $180,000 a year".to_string()),
            description: "Join our security team building high-performance, secure systems in Rust...".to_string(),
            application_url: "https://www.linkedin.com/jobs/view/job202".to_string(),
            easy_apply: true,
        },
    ];
    
    // Create a simulated LinkedIn client that returns the mock data
    LinkedInClient::new_with_simulated_data(simulated_jobs)
} 