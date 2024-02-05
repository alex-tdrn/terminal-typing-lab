pub enum Action {
    ToggleFrameStatistics,
    ToggleLiveTypingStatistics,
    CharacterInput(char),
    DeleteCharacter,
    DeleteWord,
    Restart,
    NextTest,
    NextCorpus,
    PreviousCorpus,
    IncreaseTestLength,
    DecreaseTestLength,
    Quit,
}
