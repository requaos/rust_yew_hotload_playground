struct AllPersistentState {
    current_term: i64,
    voted_for: Option<i32>,
    logs: Vec<LogEntry>,
}
struct LogEntry {
    command: StateCommand,
    term_idx: i64,
}
enum StateCommand {
    INSERT,
    DELETE,
}

struct AllVolatileState {
    commit_idx: i64,
    last_applied: LogEntry,
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

impl Server {
    fn RequestVote(
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

    fn AppendEntries(
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
