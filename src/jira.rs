use surrealdb::engine::any::connect;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

use self::{
    auth::{jira_authentication, JiraAuth},
    issue::JiraIssues,
    projects::JiraProjects,
};

pub mod auth;
pub mod issue;
pub mod projects;

pub struct Jira {
    pub auth: JiraAuth,
    pub db: Surreal<Any>,
    pub projects: JiraProjects,
    pub issues: JiraIssues,
}

impl Jira {
    pub async fn new() -> anyhow::Result<Jira> {
        let auth = jira_authentication();
        let db = connect("mem://").await?;
        db.use_ns("noc").use_db("database").await?;
        let projects: JiraProjects = JiraProjects::new(&auth, &db).await?;
        let issues: JiraIssues = JiraIssues::new(&auth).await?;

        Ok(Self {
            auth,
            db,
            projects,
            issues,
        })
    }
}
