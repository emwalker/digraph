/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  Color: { input: any; output: any; }
  DateTime: { input: any; output: any; }
};

export type ActivityLineItem = {
  __typename?: 'ActivityLineItem';
  createdAt: Scalars['DateTime']['output'];
  description: Scalars['String']['output'];
};

export type ActivityLineItemConnection = {
  __typename?: 'ActivityLineItemConnection';
  edges?: Maybe<Array<Maybe<ActivityLineItemEdge>>>;
  pageInfo: PageInfo;
};

export type ActivityLineItemEdge = {
  __typename?: 'ActivityLineItemEdge';
  cursor: Scalars['String']['output'];
  node: ActivityLineItem;
};

export type Alert = {
  __typename?: 'Alert';
  id: Scalars['String']['output'];
  text: Scalars['String']['output'];
  type: AlertType;
};

export enum AlertType {
  Error = 'ERROR',
  Success = 'SUCCESS',
  Warn = 'WARN'
}

export type Alertable = {
  alerts: Array<Alert>;
};

export type CreateGithubSessionInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  githubAvatarUrl: Scalars['String']['input'];
  githubUsername: Scalars['String']['input'];
  name: Scalars['String']['input'];
  primaryEmail: Scalars['String']['input'];
  serverSecret: Scalars['String']['input'];
};

export type CreateGoogleSessionInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  googleAvatarUrl: Scalars['String']['input'];
  googleProfileId: Scalars['String']['input'];
  name: Scalars['String']['input'];
  primaryEmail: Scalars['String']['input'];
  serverSecret: Scalars['String']['input'];
};

export type CreateSessionPayload = Alertable & {
  __typename?: 'CreateSessionPayload';
  alerts: Array<Alert>;
  sessionEdge?: Maybe<SessionEdge>;
  userEdge?: Maybe<UserEdge>;
};

export type DeleteAccountInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  userId: Scalars['ID']['input'];
};

export type DeleteAccountPayload = Alertable & {
  __typename?: 'DeleteAccountPayload';
  alerts: Array<Alert>;
  clientMutationId?: Maybe<Scalars['String']['output']>;
  deletedUserId: Scalars['ID']['output'];
};

export type DeleteLinkInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  linkId: Scalars['String']['input'];
  repoId: Scalars['String']['input'];
};

export type DeleteLinkPayload = {
  __typename?: 'DeleteLinkPayload';
  clientMutationId?: Maybe<Scalars['String']['output']>;
  deletedLinkId?: Maybe<Scalars['String']['output']>;
};

export type DeleteSessionInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  sessionId: Scalars['ID']['input'];
};

export type DeleteSessionPayload = {
  __typename?: 'DeleteSessionPayload';
  clientMutationId?: Maybe<Scalars['String']['output']>;
  deletedSessionId?: Maybe<Scalars['ID']['output']>;
};

export type DeleteTopicInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  repoId: Scalars['String']['input'];
  topicId: Scalars['String']['input'];
};

export type DeleteTopicPayload = {
  __typename?: 'DeleteTopicPayload';
  clientMutationId?: Maybe<Scalars['String']['output']>;
  deletedTopicId?: Maybe<Scalars['String']['output']>;
};

export type Link = {
  __typename?: 'Link';
  displayParentTopics: TopicConnection;
  displayTitle: Scalars['String']['output'];
  displayUrl: Scalars['String']['output'];
  id: Scalars['String']['output'];
  loading: Scalars['Boolean']['output'];
  newlyAdded: Scalars['Boolean']['output'];
  repoLink?: Maybe<RepoLink>;
  repoLinks: Array<RepoLink>;
  sha1: Scalars['String']['output'];
  showRepoOwnership: Scalars['Boolean']['output'];
  viewerCanUpdate: Scalars['Boolean']['output'];
};


export type LinkDisplayParentTopicsArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};


export type LinkRepoLinkArgs = {
  repoId: Scalars['ID']['input'];
};

export type LinkConnection = {
  __typename?: 'LinkConnection';
  edges?: Maybe<Array<Maybe<LinkEdge>>>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int']['output'];
};

export type LinkEdge = {
  __typename?: 'LinkEdge';
  cursor: Scalars['String']['output'];
  node: Link;
};

export type LiveSearchTopicsPayload = {
  __typename?: 'LiveSearchTopicsPayload';
  synonyms: Array<SynonymEntry>;
};

export enum LocaleIdentifier {
  Ar = 'ar',
  De = 'de',
  El = 'el',
  En = 'en',
  Es = 'es',
  Fa = 'fa',
  Fi = 'fi',
  Fr = 'fr',
  Hi = 'hi',
  It = 'it',
  Ja = 'ja',
  Ji = 'ji',
  Ko = 'ko',
  La = 'la',
  Nl = 'nl',
  No = 'no',
  Pt = 'pt',
  Ru = 'ru',
  Sv = 'sv',
  Tr = 'tr',
  Uk = 'uk',
  Zh = 'zh'
}

export type Mutation = {
  __typename?: 'Mutation';
  createGithubSession?: Maybe<CreateSessionPayload>;
  createGoogleSession?: Maybe<CreateSessionPayload>;
  deleteAccount?: Maybe<DeleteAccountPayload>;
  deleteLink?: Maybe<DeleteLinkPayload>;
  deleteSession?: Maybe<DeleteSessionPayload>;
  deleteTopic?: Maybe<DeleteTopicPayload>;
  removeTopicTimerange?: Maybe<RemoveTopicTimerangePayload>;
  selectRepository?: Maybe<SelectRepositoryPayload>;
  updateLinkParentTopics?: Maybe<UpdateLinkParentTopicsPayload>;
  updateTopicParentTopics?: Maybe<UpdateTopicParentTopicsPayload>;
  updateTopicSynonyms?: Maybe<UpdateTopicSynonymsPayload>;
  upsertLink?: Maybe<UpsertLinkPayload>;
  upsertTopic?: Maybe<UpsertTopicPayload>;
  upsertTopicTimerange?: Maybe<UpsertTopicTimerangePayload>;
};


export type MutationCreateGithubSessionArgs = {
  input: CreateGithubSessionInput;
};


export type MutationCreateGoogleSessionArgs = {
  input: CreateGoogleSessionInput;
};


export type MutationDeleteAccountArgs = {
  input: DeleteAccountInput;
};


export type MutationDeleteLinkArgs = {
  input: DeleteLinkInput;
};


export type MutationDeleteSessionArgs = {
  input: DeleteSessionInput;
};


export type MutationDeleteTopicArgs = {
  input: DeleteTopicInput;
};


export type MutationRemoveTopicTimerangeArgs = {
  input: RemoveTopicTimerangeInput;
};


export type MutationSelectRepositoryArgs = {
  input: SelectRepositoryInput;
};


export type MutationUpdateLinkParentTopicsArgs = {
  input: UpdateLinkParentTopicsInput;
};


export type MutationUpdateTopicParentTopicsArgs = {
  input: UpdateTopicParentTopicsInput;
};


export type MutationUpdateTopicSynonymsArgs = {
  input: UpdateTopicSynonymsInput;
};


export type MutationUpsertLinkArgs = {
  input: UpsertLinkInput;
};


export type MutationUpsertTopicArgs = {
  input: UpsertTopicInput;
};


export type MutationUpsertTopicTimerangeArgs = {
  input: UpsertTopicTimerangeInput;
};

export enum OnMatchingSynonym {
  Ask = 'ASK',
  CreateDistinct = 'CREATE_DISTINCT',
  Update = 'UPDATE'
}

export type Organization = {
  __typename?: 'Organization';
  createdAt: Scalars['DateTime']['output'];
  defaultRepository: Repository;
  id?: Maybe<Scalars['ID']['output']>;
  login: Scalars['String']['output'];
  name: Scalars['String']['output'];
  public: Scalars['Boolean']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type PageInfo = {
  __typename?: 'PageInfo';
  endCursor?: Maybe<Scalars['String']['output']>;
  hasNextPage: Scalars['Boolean']['output'];
  hasPreviousPage: Scalars['Boolean']['output'];
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type Query = {
  __typename?: 'Query';
  alerts: Array<Alert>;
  fakeError?: Maybe<Scalars['String']['output']>;
  view: View;
};


export type QueryViewArgs = {
  repoIds?: InputMaybe<Array<Scalars['ID']['input']>>;
  searchString?: InputMaybe<Scalars['String']['input']>;
  viewerId: Scalars['ID']['input'];
};

export type QueryInfo = {
  __typename?: 'QueryInfo';
  phrases: Array<Scalars['String']['output']>;
  topics: Array<Topic>;
};

export type RemoveTopicTimerangeInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  repoId: Scalars['String']['input'];
  topicId: Scalars['String']['input'];
};

export type RemoveTopicTimerangePayload = {
  __typename?: 'RemoveTopicTimerangePayload';
  clientMutationId?: Maybe<Scalars['String']['output']>;
  updatedRepoTopic: RepoTopic;
  updatedTopic: Topic;
};

export type ReorderSynonymsInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  synonymIds: Array<Scalars['ID']['input']>;
  topicId: Scalars['String']['input'];
};

export type ReorderSynonymsPayload = {
  __typename?: 'ReorderSynonymsPayload';
  clientMutationId?: Maybe<Scalars['String']['output']>;
};

export type RepoLink = {
  __typename?: 'RepoLink';
  availableParentTopics: LiveSearchTopicsPayload;
  createdAt: Scalars['DateTime']['output'];
  details?: Maybe<RepoLinkDetails>;
  displayColor: Scalars['Color']['output'];
  inWikiRepo: Scalars['Boolean']['output'];
  link: Link;
  linkId: Scalars['ID']['output'];
  parentTopics: TopicConnection;
  repo: Repository;
  updatedAt: Scalars['DateTime']['output'];
  viewerCanUpdate: Scalars['Boolean']['output'];
};


export type RepoLinkAvailableParentTopicsArgs = {
  searchString?: InputMaybe<Scalars['String']['input']>;
};


export type RepoLinkParentTopicsArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type RepoLinkDetails = {
  __typename?: 'RepoLinkDetails';
  title: Scalars['String']['output'];
  url: Scalars['String']['output'];
};

export type RepoTopic = {
  __typename?: 'RepoTopic';
  availableParentTopics: LiveSearchTopicsPayload;
  color: Scalars['Color']['output'];
  createdAt: Scalars['DateTime']['output'];
  details?: Maybe<RepoTopicDetails>;
  displayColor: Scalars['Color']['output'];
  displayName: Scalars['String']['output'];
  id: Scalars['String']['output'];
  inWikiRepo: Scalars['Boolean']['output'];
  parentTopics: TopicConnection;
  repo: Repository;
  repoId: Scalars['ID']['output'];
  timerangePrefix: Scalars['String']['output'];
  topicId: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
  viewerCanDeleteSynonyms: Scalars['Boolean']['output'];
  viewerCanUpdate: Scalars['Boolean']['output'];
};


export type RepoTopicAvailableParentTopicsArgs = {
  searchString?: InputMaybe<Scalars['String']['input']>;
};


export type RepoTopicParentTopicsArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type RepoTopicDetails = {
  __typename?: 'RepoTopicDetails';
  synonyms: Array<Synonym>;
  timerange?: Maybe<Timerange>;
};

export type Repository = {
  __typename?: 'Repository';
  displayColor: Scalars['Color']['output'];
  displayName: Scalars['String']['output'];
  fullName: Scalars['String']['output'];
  id?: Maybe<Scalars['String']['output']>;
  isPrivate: Scalars['Boolean']['output'];
  name: Scalars['String']['output'];
  organization: Organization;
  owner: User;
  rootTopic: Topic;
};

export type RepositoryConnection = {
  __typename?: 'RepositoryConnection';
  edges?: Maybe<Array<Maybe<RepositoryEdge>>>;
};

export type RepositoryEdge = {
  __typename?: 'RepositoryEdge';
  cursor: Scalars['String']['output'];
  isSelected: Scalars['Boolean']['output'];
  node: Repository;
};

export type SearchMatch = Link | Topic;

export type SearchMatchEdge = {
  __typename?: 'SearchMatchEdge';
  cursor: Scalars['String']['output'];
  node: SearchMatch;
};

export type SearchResultConnection = {
  __typename?: 'SearchResultConnection';
  edges?: Maybe<Array<Maybe<SearchMatchEdge>>>;
  pageInfo: PageInfo;
};

export type SelectRepositoryInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  currentTopicId?: InputMaybe<Scalars['ID']['input']>;
  repoId?: InputMaybe<Scalars['ID']['input']>;
};

export type SelectRepositoryPayload = {
  __typename?: 'SelectRepositoryPayload';
  currentTopic?: Maybe<Topic>;
  repo?: Maybe<Repository>;
  viewer: User;
};

export type Session = {
  __typename?: 'Session';
  id: Scalars['ID']['output'];
};

export type SessionEdge = {
  __typename?: 'SessionEdge';
  cursor: Scalars['String']['output'];
  node: Session;
};

export type Synonym = {
  __typename?: 'Synonym';
  locale: LocaleIdentifier;
  name: Scalars['String']['output'];
};

export type SynonymEntry = {
  __typename?: 'SynonymEntry';
  displayName: Scalars['String']['output'];
  id: Scalars['String']['output'];
};

export type SynonymInput = {
  locale: Scalars['String']['input'];
  name: Scalars['String']['input'];
};

export type Timerange = {
  __typename?: 'Timerange';
  endsAt?: Maybe<Scalars['DateTime']['output']>;
  prefixFormat: TimerangePrefixFormat;
  startsAt: Scalars['DateTime']['output'];
};

export type TimerangeEdge = {
  __typename?: 'TimerangeEdge';
  cursor: Scalars['String']['output'];
  node: Timerange;
};

export enum TimerangePrefixFormat {
  None = 'NONE',
  StartYear = 'START_YEAR',
  StartYearMonth = 'START_YEAR_MONTH'
}

export type Topic = {
  __typename?: 'Topic';
  activity: ActivityLineItemConnection;
  children: SearchResultConnection;
  displayName: Scalars['String']['output'];
  displayParentTopics: TopicConnection;
  displaySynonyms: Array<Synonym>;
  displayTimerange?: Maybe<Timerange>;
  id: Scalars['String']['output'];
  loading: Scalars['Boolean']['output'];
  newlyAdded: Scalars['Boolean']['output'];
  repoTopic?: Maybe<RepoTopic>;
  repoTopics: Array<RepoTopic>;
  showRepoOwnership: Scalars['Boolean']['output'];
  viewerCanUpdate: Scalars['Boolean']['output'];
};


export type TopicActivityArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};


export type TopicChildrenArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  searchString?: InputMaybe<Scalars['String']['input']>;
};


export type TopicDisplayParentTopicsArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};


export type TopicRepoTopicArgs = {
  repoId: Scalars['ID']['input'];
};

export type TopicConnection = {
  __typename?: 'TopicConnection';
  edges?: Maybe<Array<Maybe<TopicEdge>>>;
  pageInfo: PageInfo;
};

export type TopicEdge = {
  __typename?: 'TopicEdge';
  cursor: Scalars['String']['output'];
  node: Topic;
};

export type UpdateLinkParentTopicsInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  linkId: Scalars['String']['input'];
  parentTopicIds?: InputMaybe<Array<Scalars['String']['input']>>;
  repoId: Scalars['String']['input'];
};

export type UpdateLinkParentTopicsPayload = {
  __typename?: 'UpdateLinkParentTopicsPayload';
  link: Link;
};

export type UpdateTopicParentTopicsInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  parentTopicIds?: InputMaybe<Array<Scalars['String']['input']>>;
  repoId: Scalars['String']['input'];
  topicId: Scalars['String']['input'];
};

export type UpdateTopicParentTopicsPayload = Alertable & {
  __typename?: 'UpdateTopicParentTopicsPayload';
  alerts: Array<Alert>;
  topic: Topic;
};

export type UpdateTopicSynonymsInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  repoId: Scalars['String']['input'];
  synonyms: Array<SynonymInput>;
  topicId: Scalars['String']['input'];
};

export type UpdateTopicSynonymsPayload = {
  __typename?: 'UpdateTopicSynonymsPayload';
  alerts: Array<Alert>;
  clientMutationId?: Maybe<Scalars['String']['output']>;
  updatedRepoTopic: RepoTopic;
  updatedTopic: Topic;
};

export type UpsertLinkInput = {
  addParentTopicId?: InputMaybe<Scalars['String']['input']>;
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  linkId?: InputMaybe<Scalars['String']['input']>;
  repoId: Scalars['String']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
  url: Scalars['String']['input'];
};

export type UpsertLinkPayload = Alertable & {
  __typename?: 'UpsertLinkPayload';
  alerts: Array<Alert>;
  linkEdge?: Maybe<LinkEdge>;
};

export type UpsertTopicInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  onMatchingSynonym: OnMatchingSynonym;
  parentTopicId: Scalars['String']['input'];
  repoId: Scalars['String']['input'];
  updateTopicId?: InputMaybe<Scalars['String']['input']>;
};

export type UpsertTopicPayload = Alertable & {
  __typename?: 'UpsertTopicPayload';
  alerts: Array<Alert>;
  matchingTopics: Array<Topic>;
  topicEdge?: Maybe<TopicEdge>;
  updatedParentTopic: Topic;
};

export type UpsertTopicTimerangeInput = {
  clientMutationId?: InputMaybe<Scalars['String']['input']>;
  endsAt?: InputMaybe<Scalars['DateTime']['input']>;
  prefixFormat: TimerangePrefixFormat;
  repoId: Scalars['String']['input'];
  startsAt: Scalars['DateTime']['input'];
  topicId: Scalars['String']['input'];
};

export type UpsertTopicTimerangePayload = Alertable & {
  __typename?: 'UpsertTopicTimerangePayload';
  alerts: Array<Alert>;
  timerangeEdge?: Maybe<TimerangeEdge>;
  updatedRepoTopic: RepoTopic;
  updatedTopic: Topic;
};

export type User = {
  __typename?: 'User';
  avatarUrl: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  defaultRepo?: Maybe<Repository>;
  id?: Maybe<Scalars['ID']['output']>;
  isGuest: Scalars['Boolean']['output'];
  name: Scalars['String']['output'];
  primaryEmail: Scalars['String']['output'];
  repos: RepositoryConnection;
  selectedRepo?: Maybe<Repository>;
  selectedRepoId?: Maybe<Scalars['ID']['output']>;
  updatedAt: Scalars['DateTime']['output'];
};


export type UserReposArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type UserEdge = {
  __typename?: 'UserEdge';
  cursor: Scalars['String']['output'];
  node: User;
};

export type View = {
  __typename?: 'View';
  activity: ActivityLineItemConnection;
  currentRepository?: Maybe<Repository>;
  defaultOrganization: Organization;
  link?: Maybe<Link>;
  links: LinkConnection;
  queryInfo: QueryInfo;
  searchString?: Maybe<Scalars['String']['output']>;
  stats: ViewStats;
  topic?: Maybe<Topic>;
  topicGraph?: Maybe<Scalars['String']['output']>;
  topicLiveSearch: LiveSearchTopicsPayload;
  viewer: User;
};


export type ViewActivityArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  topicId?: InputMaybe<Scalars['String']['input']>;
};


export type ViewLinkArgs = {
  id: Scalars['ID']['input'];
};


export type ViewLinksArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  searchString?: InputMaybe<Scalars['String']['input']>;
};


export type ViewTopicArgs = {
  id: Scalars['ID']['input'];
};


export type ViewTopicLiveSearchArgs = {
  searchString?: InputMaybe<Scalars['String']['input']>;
};

export type ViewStats = {
  __typename?: 'ViewStats';
  calculating: Scalars['Boolean']['output'];
  linkCount?: Maybe<Scalars['Int']['output']>;
  topicCount?: Maybe<Scalars['Int']['output']>;
};

export type SearchBoxQueryVariables = Exact<{
  repoIds: Array<Scalars['ID']['input']> | Scalars['ID']['input'];
  searchString: Scalars['String']['input'];
  topicSynonymSearchString: Scalars['String']['input'];
  viewerId: Scalars['ID']['input'];
}>;


export type SearchBoxQuery = { __typename?: 'Query', view: { __typename?: 'View', topicLiveSearch: { __typename?: 'LiveSearchTopicsPayload', synonyms: Array<{ __typename?: 'SynonymEntry', displayName: string, id: string }> }, queryInfo: { __typename?: 'QueryInfo', phrases: Array<string>, topics: Array<{ __typename?: 'Topic', displayName: string, id: string }> } } };

export type SearchResultsQueryVariables = Exact<{
  repoIds: Array<Scalars['ID']['input']> | Scalars['ID']['input'];
  topicId: Scalars['ID']['input'];
  searchString: Scalars['String']['input'];
  viewerId: Scalars['ID']['input'];
}>;


export type SearchResultsQuery = { __typename?: 'Query', view: { __typename?: 'View', topic?: { __typename?: 'Topic', displayName: string, displaySynonyms: Array<{ __typename?: 'Synonym', name: string }>, displayParentTopics: { __typename?: 'TopicConnection', edges?: Array<{ __typename?: 'TopicEdge', node: { __typename?: 'Topic', id: string, displayName: string } } | null> | null }, children: { __typename?: 'SearchResultConnection', edges?: Array<{ __typename?: 'SearchMatchEdge', node: { __typename?: 'Link', id: string, displayTitle: string, displayUrl: string, displayParentTopics: { __typename?: 'TopicConnection', edges?: Array<{ __typename?: 'TopicEdge', node: { __typename?: 'Topic', displayName: string, id: string } } | null> | null } } | { __typename?: 'Topic', id: string, displayName: string, displayParentTopics: { __typename?: 'TopicConnection', edges?: Array<{ __typename?: 'TopicEdge', node: { __typename?: 'Topic', displayName: string, id: string } } | null> | null } } } | null> | null } } | null } };


export const SearchBoxDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"SearchBox"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"repoIds"}},"type":{"kind":"NonNullType","type":{"kind":"ListType","type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"searchString"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"topicSynonymSearchString"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"viewerId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"view"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"repoIds"},"value":{"kind":"Variable","name":{"kind":"Name","value":"repoIds"}}},{"kind":"Argument","name":{"kind":"Name","value":"searchString"},"value":{"kind":"Variable","name":{"kind":"Name","value":"searchString"}}},{"kind":"Argument","name":{"kind":"Name","value":"viewerId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"viewerId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"topicLiveSearch"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"searchString"},"value":{"kind":"Variable","name":{"kind":"Name","value":"topicSynonymSearchString"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"synonyms"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"queryInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"topics"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"id"}}]}},{"kind":"Field","name":{"kind":"Name","value":"phrases"}}]}}]}}]}}]} as unknown as DocumentNode<SearchBoxQuery, SearchBoxQueryVariables>;
export const SearchResultsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"SearchResults"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"repoIds"}},"type":{"kind":"NonNullType","type":{"kind":"ListType","type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"topicId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"searchString"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"viewerId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"view"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"repoIds"},"value":{"kind":"Variable","name":{"kind":"Name","value":"repoIds"}}},{"kind":"Argument","name":{"kind":"Name","value":"searchString"},"value":{"kind":"Variable","name":{"kind":"Name","value":"searchString"}}},{"kind":"Argument","name":{"kind":"Name","value":"viewerId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"viewerId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"topic"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"topicId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"displaySynonyms"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"Field","name":{"kind":"Name","value":"displayParentTopics"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"first"},"value":{"kind":"IntValue","value":"10"}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"displayName"}}]}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"children"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"searchString"},"value":{"kind":"Variable","name":{"kind":"Name","value":"searchString"}}},{"kind":"Argument","name":{"kind":"Name","value":"first"},"value":{"kind":"IntValue","value":"50"}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Topic"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"displayParentTopics"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"first"},"value":{"kind":"IntValue","value":"10"}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Link"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"displayTitle"}},{"kind":"Field","name":{"kind":"Name","value":"displayUrl"}},{"kind":"Field","name":{"kind":"Name","value":"displayParentTopics"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"first"},"value":{"kind":"IntValue","value":"10"}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"displayName"}},{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<SearchResultsQuery, SearchResultsQueryVariables>;