// @flow
/* eslint import/no-extraneous-dependencies: 0 */
import { configure } from 'enzyme'
import Adapter from '@wojtekmaj/enzyme-adapter-react-17'

configure({ adapter: new Adapter() })
