import { ReactElement } from 'react';
import { RenderOptions } from '@testing-library/react';
import { MockEnvironment } from 'relay-test-utils';
declare function customRender(ui: ReactElement, options?: RenderOptions): {
    environment: MockEnvironment;
};
declare function renderWithUser(component: ReactElement): {
    environment: MockEnvironment;
    user: import("@testing-library/user-event/dist/types/setup/setup").UserEvent;
};
export { customRender as render, renderWithUser };
