import { Component, FormEvent, ChangeEvent } from 'react';
import { RelayProp } from 'react-relay';
import { Synonyms_topic$data as TopicType } from '__generated__/Synonyms_topic.graphql';
import { SynonymType } from 'components/types';
declare type RepoTopicType = TopicType['repoTopics'][0];
declare type Props = {
    relay: RelayProp;
    topic: TopicType;
};
declare type State = {
    inputLocale: string;
    inputName: string;
};
declare class Synonyms extends Component<Props, State> {
    constructor(props: Props);
    onLocaleChange: (event: FormEvent<HTMLSelectElement>) => void;
    onNameChange: (event: ChangeEvent<HTMLInputElement>) => void;
    onAdd: () => void;
    onDelete: (position: number) => void;
    get topicDetail(): RepoTopicType | null;
    get synonyms(): readonly {
        readonly locale: import("__generated__/Synonyms_topic.graphql").LocaleIdentifier;
        readonly name: string;
        readonly " $fragmentSpreads": import("relay-runtime").FragmentRefs<"Synonym_synonym">;
    }[];
    optimisticResponse: (synonyms: SynonymType[]) => {
        updateTopicSynonyms: {
            alerts: never[];
            clientMutationId: null;
            topic: {
                displayName: string;
                repoTopics: {
                    synonyms: SynonymType[];
                    displayName?: string | undefined;
                    topicId?: string | undefined;
                    viewerCanDeleteSynonyms?: boolean | undefined;
                    viewerCanUpdate?: boolean | undefined;
                }[];
                viewerCanUpdate: boolean;
                " $fragmentType": "Synonyms_topic";
            };
        };
    };
    updateTopicSynonyms: (synonyms: SynonymType[]) => null | undefined;
    renderSynonyms: () => JSX.Element | null;
    renderAddForm: () => JSX.Element;
    render: () => JSX.Element;
}
export declare const UnwrappedSynonyms: typeof Synonyms;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
