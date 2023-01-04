pub mod activity;
mod checks;
pub mod core;
pub mod testing;

mod account;
pub use account::{DeleteAccount, EnsurePersonalRepo, EnsurePersonalRepoResult};

mod client;
pub use client::{parse_path, Client, DataRoot, GitPaths, Mutation};

mod ext;
pub use ext::{Link, Object, ObjectBuilders, RepoLinkWrapper, RepoTopicWrapper, Synonyms, Topic};

mod index;
pub(crate) use index::{
    ChangeReference, Phrase, SaveChangesForPrefix, SearchTokenIndex, SynonymIndex, SynonymMatch,
};
pub use index::{IndexMode, SearchEntry, SynonymEntry};

mod link;
pub use link::{
    DeleteLink, DeleteLinkResult, UpdateLinkParentTopics, UpdateLinkParentTopicsResult, UpsertLink,
    UpsertLinkResult,
};

mod search;
pub use search::{
    FetchTopicLiveSearch, FetchTopicLiveSearchResult, FindMatches, FindMatchesResult,
    RedisFetchDownSet, Search, SearchMatch, SortKey,
};

mod stats;
pub use stats::{CacheStats, FetchStats, FetchStatsResult, RepoStats};

mod topic;
pub use topic::{
    DeleteTopic, DeleteTopicResult, OnMatchingSynonym, RemoveTopicTimerange,
    RemoveTopicTimerangeResult, UpdateTopicParentTopics, UpdateTopicParentTopicsResult,
    UpdateTopicSynonyms, UpdateTopicSynonymsResult, UpsertTopic, UpsertTopicResult,
    UpsertTopicTimerange, UpsertTopicTimerangeResult,
};

mod types;
pub use types::{
    DownsetIter, Kind, OuterRepoObject, ParentTopic, RepoLink, RepoLinkDetails, RepoLinkMetadata,
    RepoObject, RepoTopic, RepoTopicDetails, RepoTopicMetadata, Synonym, TopicChild,
    TopicDownsetIter, Visitor,
};
