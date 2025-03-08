mod is_git_repo;
pub use is_git_repo::is_git_repo;

mod get_git_dif;
pub use get_git_dif::get_git_diff;

mod read_config;
pub use read_config::read_config;

mod read_guideline;
pub use read_guideline::read_guideline;

mod get_commit_from_deepseek;
pub use get_commit_from_deepseek::get_commit_from_deepseek;

//mod validate_commit_message;
//pub use validate_commit_message::validate_commit_message;

mod create_git_commit;
pub use create_git_commit::create_git_commit;

mod run_pre_commits;
pub use run_pre_commits::run_pre_commits;
