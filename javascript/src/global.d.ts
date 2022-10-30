import { AlertMessageType } from 'components/types';
import { User } from 'express';
import { RouteMatch, Location, RouteObjectBase } from 'found'
import { ReactNode } from 'react-markdown';
import { StoreEnhancer } from 'redux'

declare global {
  type AddFlashMessagesProps = {
    text: string,
    type: string,
    id: string
  }

  type PreloadsType = {
    [key: string]: string
  }

  interface Window {
    flashMessages: {
      addMessage: (AddFlashMessagesProps) => void,
      addAlert: (ReactNode) => void,
      removeAlert: (alert: ReactElement<typeof Alert>) => void,
      removeMessage: (alert: AlertMessageType) => void,
    } | undefined,
    __PRELOADED_STATE__?: StoreEnhancer<unknown, unknown>
    __RELAY_PAYLOADS__?: PreloadsType
  }

  namespace Express {
    interface User {
      id?: string,
      sessionId?: string,
    }
  }

  interface Location {
    state: {}
  }

  // found-relay extensions
  interface RouteObjectBase {
    prepareVariables?: () => void,
  }
};
