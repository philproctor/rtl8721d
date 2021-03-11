import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    service: {
      online: false,
    },
  },
  getters: {
    serviceOnline: (state) => {
      return state.service.online
    },
  },
  mutations: {
    statusUpdate: (state, status) => {
      state.service.online = status
    },
  },
  actions: {
    async getStatus({ commit }) {
      const request = new Request('/api/status', {
        method: 'GET',
        // cache: false,
        body: null,
      })
      const res = await fetch(request)
      const data = await res.json()
      commit('statusUpdate', data.running)
    },
  },
})
