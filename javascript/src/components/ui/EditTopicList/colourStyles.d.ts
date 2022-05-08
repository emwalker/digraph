declare type Option = {
    data: {
        color: string;
    };
    isDisabled?: boolean;
    isFocused?: boolean;
    isSelected?: boolean;
};
declare type Styles = {};
declare const _default: {
    control: (styles: Styles) => {
        backgroundColor: string;
    };
    option: (styles: Styles, { data, isDisabled, isFocused, isSelected, }: Option) => {
        backgroundColor: string | undefined;
        color: string;
        cursor: string;
    };
    multiValue: (styles: Styles, { data }: Option) => {
        backgroundColor: string;
    };
    multiValueLabel: (styles: Styles, { data }: Option) => {
        color: string;
    };
    multiValueRemove: (styles: Styles, { data }: Option) => {
        color: string;
        ':hover': {
            backgroundColor: string;
            color: string;
        };
    };
};
export default _default;
