
import { createStore } from 'framework7/lite';

const store = createStore({
  state: {
    user: null,
    booklist: {
      n1: {},
      n2: {},
      n3: {
        index: 1
      },
      n4: {}
    },
  },
  getters: {
    user({ state }) {
      return state.user;
    },
    booklist({ state }) {
      return state.booklist;
    }
  },
  actions: {
    set_user_info({ state }, user) {
      state.user = user
    },
    update_index({ state }, level) {
      state.booklist[`n${level}`].index += 1;
    }
  },
})
export default store;
