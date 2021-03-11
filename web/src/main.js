import 'purecss/build/base-min.css'
import 'purecss/build/buttons-min.css'
import 'purecss/build/grids-min.css'
import 'purecss/build/menus-min.css'

import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: (h) => h(App),
}).$mount('#app')
