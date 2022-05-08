import { EditorState, RawDraftContentState } from 'draft-js';
declare class Query {
    data: RawDraftContentState;
    constructor(editorState: EditorState);
    get parts(): string[];
    toString: () => string;
}
declare const queryFromState: (editorState: EditorState) => Query;
export default queryFromState;
