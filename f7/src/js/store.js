
import { createStore } from 'framework7/lite';

const store = createStore({
  state: {
    user: null,
  },
  getters: {
    user({ state }) {
      return state.user;
    },
  },
  actions: {
    setUserInfo({ state }, user) {
      state.user = user
    }
  },
})
export default store;
