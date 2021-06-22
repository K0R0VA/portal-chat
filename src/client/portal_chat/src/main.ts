import { createApp } from 'vue'
import App from './App.vue'
import './registerServiceWorker'
import router from './router'
import store from './store'
import { IonicVue } from '@ionic/vue'

createApp(App)
  .use(store)
  .use(router)
  .use(IonicVue)
  .mount('#app')
