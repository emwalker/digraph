import { Component, FormEvent, ChangeEvent } from 'react';
import { RelayProp } from 'react-relay';
import { Synonyms_topicDetail as TopicDetailType } from '__generated__/Synonyms_topicDetail.graphql';
import { SynonymType } from 'components/types';
declare type Props = {
    relay: RelayProp;
    topicDetail: TopicDetailType;
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
    get synonyms(): readonly {
        readonly name: string;
        readonly locale: import("__generated__/Synonyms_topicDetail.graphql").LocaleIdentifier;
        readonly " $fragmentRefs": import("relay-runtime").FragmentRefs<"Synonym_synonym">;
    }[];
    optimisticResponse: (synonyms: SynonymType[]) => {
        updateTopicSynonyms: {
            alerts: never[];
            clientMutationId: null;
            topic: {
                displayName: string;
                synonyms: SynonymType[];
                topicId: string;
                viewerCanDeleteSynonyms: boolean;
                viewerCanUpdate: boolean;
                " $refType": "Synonyms_topicDetail";
            };
        };
    };
    updateTopicSynonyms: (synonyms: SynonymType[]) => void;
    renderSynonyms: () => JSX.Element;
    renderAddForm: () => JSX.Element;
    render: () => JSX.Element;
}
export declare const UnwrappedSynonyms: typeof Synonyms;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
