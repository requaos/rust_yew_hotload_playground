// Commit changes to disk before responding to RPC calls
struct AllPersistentState {
    current_term: i64,
    voted_for: Option<i32>,
    logs: Vec<LogEntry>,
}
impl Default for AllPersistentState {
    fn default() -> Self {
        Self {
            current_term: 0,
            voted_for: None,
            // Occupy index 0 with start command (noop, but may leverage for initialization checks)
            logs: LogEntry::init_logs(),
        }
    }
}

enum StateCommand {
    START,
    INSERT,
    DELETE,
}
struct LogEntry {
    command: StateCommand,
    term_idx: i64,
}
impl LogEntry {
    pub fn new(command: StateCommand, term: i64) -> Self {
        Self {
            command,
            term_idx: term,
        }
    }
    pub fn init_logs() -> Vec<LogEntry> {
        vec![[LogEntry {
            command: StateCommand::START,
            term_idx: 0,
        }]]
    }
}

struct AllVolatileState {
    // TODO: For all these monotonically increasing i64 values, reference https://docs.rs/unique_id/latest/unique_id/sequence/struct.SequenceGenerator.html#
    commit_idx: i64,
    last_applied: i64,
}
struct LeaderVolatileState {
    next_idx: Vec<i64>,
    match_idx: Vec<i64>,
}

struct Server {
    ap_state: AllPersistentState,
    av_state: AllVolatileState,

    is_leader: bool,
    lv_state: LeaderVolatileState,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ap_state: AllPersistentState::default(),
            av_state: AllVolatileState {
                commit_idx: 0,
                last_applied: 0,
            },
            is_leader: false,
            lv_state: LeaderVolatileState {
                next_idx: vec![[1]],
                match_idx: vec![[0]],
            },
        }
    }
}

impl Server {
    pub fn RequestVote(
        &self,
        term: i64,
        candidate_id: i32,
        last_log_idx: i64,
        last_log_term: i64,
    ) -> (i64, bool) {
        if self.ap_state.current_term > term {
            return (self.ap_state.current_term, false);
        }
        match self.ap_state.voted_for {
            None | Some(candidate_id) => {
                if self.av_state.commit_idx == last_log_idx
                    && self.av_state.last_applied.term_idx == last_log_term
                {
                    return (self.ap_state.current_term, true);
                }
            }
        }
        return (self.ap_state.current_term, false);
    }

    pub fn AppendEntries(
        &mut self,
        term: i64,
        leader_id: i64,
        prev_log_idx: i64,
        prev_log_term: i64,
        entries: Vec<LogEntry>,
        leader_commit: i64,
    ) -> (i64, bool) {
        if self.ap_state.current_term > term {
            return (self.ap_state.current_term, false);
        }
        if self.ap_state.logs[prev_log_idx].term_idx != prev_log_term {
            return (self.ap_state.current_term, false);
        }

        // TODO: Do write operations here for log entry comparison and append
        // - If an existing entry conflicts with a new one (same index but different terms), delete the existing entry and all that follow it
        // - Append any new entries not already in the log
        // - If leaderCommit > commitIndex, set commitIndex =  min(leaderCommit, index of last new entry)

        return (self.ap_state.current_term, false);
    }
}
