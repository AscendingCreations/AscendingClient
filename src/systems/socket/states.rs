use mio::Interest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum PollState {
    ///None defaults to Read as read or write must exist to register.
    #[default]
    None,
    Read,
    Write,
    ReadWrite,
}

impl PollState {
    #[inline]
    pub fn add(&mut self, state: PollState) {
        match (*self, state) {
            (PollState::None, _) => *self = state,
            (PollState::Read, PollState::Write) => *self = PollState::ReadWrite,
            (PollState::Write, PollState::Read) => *self = PollState::ReadWrite,
            (_, _) => {}
        }
    }

    #[inline]
    pub fn set(&mut self, state: PollState) {
        *self = state;
    }

    #[inline]
    pub fn remove(&mut self, state: PollState) {
        match (*self, state) {
            (PollState::Read, PollState::Read) => *self = PollState::None,
            (PollState::Write, PollState::Write) => *self = PollState::None,
            (PollState::ReadWrite, PollState::Write) => *self = PollState::Read,
            (PollState::ReadWrite, PollState::Read) => *self = PollState::Write,
            (_, PollState::ReadWrite) => *self = PollState::None,
            (_, _) => {}
        }
    }

    pub fn contains(&mut self, state: PollState) -> bool {
        ((*self == PollState::Read || *self == PollState::ReadWrite)
            && (state == PollState::Read || state == PollState::ReadWrite))
            || ((*self == PollState::Write || *self == PollState::ReadWrite)
                && (state == PollState::Write || state == PollState::ReadWrite))
    }

    pub fn to_interest(self) -> Interest {
        match self {
            PollState::None => Interest::READABLE,
            PollState::Read => Interest::READABLE,
            PollState::Write => Interest::WRITABLE,
            PollState::ReadWrite => Interest::READABLE.add(Interest::WRITABLE),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    Open,
    Closing,
    Closed,
    New,
}
