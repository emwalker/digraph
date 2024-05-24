import { SynonymType } from 'components/types';
import { RepoTopicSynonyms_repoTopic$data as RepoTopicType } from '__generated__/RepoTopicSynonyms_repoTopic.graphql';
type Props = {
    selectedRepoId: string | null;
    repoTopic: RepoTopicType;
    setInputName: (inputName: string) => void;
};
export declare function makeUpdateTopicSynonymsCallback({ selectedRepoId, repoTopic, setInputName, }: Props): (synonymUpdate: SynonymType[]) => void;
export {};
