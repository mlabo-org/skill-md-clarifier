//! Agent host resolution. Every host-dependent rule reads the host from here.
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHost {
    Codex,
    ClaudeCode,
}

impl AgentHost {
    /// `AGENT_HOST` (`codex` or `claude_code`) wins; otherwise `CLAUDECODE=1`
    /// means Claude Code; otherwise Codex. Other `AGENT_HOST` values are ignored.
    pub fn resolve() -> Self {
        Self::from_env(
            std::env::var("AGENT_HOST").ok().as_deref(),
            std::env::var("CLAUDECODE").ok().as_deref(),
        )
    }

    pub fn from_env(agent_host: Option<&str>, claudecode: Option<&str>) -> Self {
        match agent_host {
            Some("codex") => Self::Codex,
            Some("claude_code") => Self::ClaudeCode,
            _ if claudecode == Some("1") => Self::ClaudeCode,
            _ => Self::Codex,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::ClaudeCode => "claude_code",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AgentHost::{self, ClaudeCode, Codex};

    #[test]
    fn explicit_host_then_claudecode_then_codex() {
        assert_eq!(AgentHost::from_env(Some("codex"), Some("1")), Codex);
        assert_eq!(AgentHost::from_env(Some("claude_code"), None), ClaudeCode);
        assert_eq!(AgentHost::from_env(Some("other"), Some("1")), ClaudeCode);
        assert_eq!(AgentHost::from_env(None, Some("1")), ClaudeCode);
        assert_eq!(AgentHost::from_env(Some("other"), None), Codex);
        assert_eq!(AgentHost::from_env(None, None), Codex);
    }
}
